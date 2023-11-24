mod dossier;
mod repository_location;

pub use repository_location::RepositoryLocation;

pub use self::dossier::Dossier;


pub struct Repository {
    name: String,
    dossiers: Vec<Dossier>
}

