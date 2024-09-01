use std::{collections::HashSet, future::Future, path::PathBuf, pin::Pin, sync::mpsc::RecvError, time::SystemTime};

use getset::{Getters, Setters};
use notify::{Error, Event, RecursiveMode, Watcher};
use thiserror::Error;
use tokio::{sync::mpsc::Receiver, task::{JoinError, JoinHandle}};

use super::preview::PreviewError;


#[derive(Error, Debug)]
pub enum WatcherError {

    #[error(transparent)]
    WatcherError(#[from] notify::Error),

    #[error(transparent)]
    ChannelError(#[from] RecvError),

    #[error(transparent)]
    PreviewError(#[from] PreviewError),

    #[error("elaboration error: {0}")]
    ElaborationError(String),

    #[error(transparent)]
    JoinError(#[from] JoinError),

    #[error("send error")]
    SendError,
}

pub type CheckIfElaborateFn<'a> = Box<dyn FnMut(Event) -> Pin<Box<dyn Future<Output = Result<bool, WatcherError>> + Send>> + Send + Sync + 'a>;
pub type OnStartFn<'a> = Box<dyn Fn() -> Pin<Box<dyn Future<Output = Result<(), WatcherError>> + Send>> + Send + Sync + 'a>;
pub type ElaborateFn<'a> = Box<dyn Fn(HashSet<PathBuf>) -> Pin<Box<dyn Future<Output = Result<(), WatcherError>> + Send>> + Send + Sync + 'a>;


#[derive(Getters, Setters)]
pub struct NmdWatcher<'a> {

    rx: Receiver<Result<Event, Error>>,

    on_start_fn: OnStartFn<'a>,

    check_if_elaborate_fn: CheckIfElaborateFn<'a>,

    check_if_elaborate_skipping_timeout_fn: CheckIfElaborateFn<'a>,

    elaborate_fn: ElaborateFn<'a>,

    min_elapsed_time_between_events_in_secs: u64,
}

impl<'a> NmdWatcher<'a> {

    pub async fn new(min_elapsed_time_between_events_in_secs: u64, input_path: &PathBuf, on_start_fn: OnStartFn<'a>, check_if_elaborate_skipping_timeout_fn: CheckIfElaborateFn<'a>, check_if_elaborate_fn: CheckIfElaborateFn<'a>, elaborate_fn: ElaborateFn<'a>) -> Result<Self, WatcherError> {
        
        let (tx, rx) = tokio::sync::mpsc::channel(4096);

        let input_path = input_path.clone();

        let _: JoinHandle<Result<(), WatcherError>> = tokio::spawn(async move {
            let (notify_tx, notify_rx) = std::sync::mpsc::channel();

            let mut watcher = notify::recommended_watcher(move |res| {
                    
                notify_tx.send(res).unwrap_or_else(|val| {
                    log::error!("error occurs during watching: {}", val);
                });
            })?;

            watcher.watch(&input_path, RecursiveMode::Recursive)?;

            while let Ok(event) = notify_rx.recv() {
                if let Err(_) = tx.send(event).await {
                    return Err(WatcherError::SendError)
                }
            }

            Ok(())
        });

        let s = Self {
            min_elapsed_time_between_events_in_secs,
            rx,
            on_start_fn,
            check_if_elaborate_fn,
            check_if_elaborate_skipping_timeout_fn,
            elaborate_fn
        };

        Ok(s)
    }

    pub async fn start(&mut self) -> Result<(), WatcherError> {

        let mut last_event_time = SystemTime::now();

        let mut paths_change_detection_from_last_elaboration: HashSet<PathBuf> = HashSet::new(); 

        (self.on_start_fn)().await?;

        loop {

            if let Ok(recv_res) = self.rx.try_recv() {

                match recv_res {
                    Ok(event) => {
                        log::debug!("new event from watcher: {:?}", event);
                        log::debug!("change detected on file(s): {:?}", event.paths);

                        event.clone().paths.iter().for_each(|path| {
                            paths_change_detection_from_last_elaboration.insert(path.clone());
                        });

                        if (self.check_if_elaborate_skipping_timeout_fn)(event.clone()).await? {

                            (self.elaborate_fn)(paths_change_detection_from_last_elaboration.clone()).await?;

                            continue;
                        }
                        
                        let event_time = SystemTime::now();

                        let elapsed_time = event_time.duration_since(last_event_time).unwrap();

                        if elapsed_time.as_secs() < self.min_elapsed_time_between_events_in_secs {
                            log::info!("change detected, but minimum elapsed time not satisfied ({}/{} s)", elapsed_time.as_secs(), self.min_elapsed_time_between_events_in_secs);

                            continue;
                        }

                        if (self.check_if_elaborate_fn)(event).await? {
                            (self.elaborate_fn)(paths_change_detection_from_last_elaboration.clone()).await?;

                            last_event_time = event_time;
                            
                            continue;
                        }
                    },
                    Err(err) => {
                        log::error!("error: {}", err.to_string());
                    },
                }

            } else {
                
                tokio::task::yield_now().await;
            }
        }
    }
}