const url = 'http://127.0.0.1:1234/preview-state-info';
const MIN_SCRAPE_INTERVAL = 1000;


var scrapeInterval = MIN_SCRAPE_INTERVAL;

var interval = null;

function stopScraping() {
    if (!!interval) {
        clearInterval(interval);
        interval = null;
    }

}

function startScraping() {

    stopScraping();
    interval = setInterval(checkPreviewUpdates, scrapeInterval);
}

async function checkPreviewUpdates() {

    console.log("checking preview updates...");

    try {
        const response = await fetch(url);
        if (!response.ok) {
            throw new Error('network response was not ok');
        }

        const data = await response.json();
        const lastUpdateTimestamp = !data.last_update_timestamp ? null : new Date(data.last_update_timestamp);
        const lastSeenTimestamp = !data.last_seen_timestamp ? null : new Date(data.last_seen_timestamp);

        if (!!data.scrape_interval && data.scrape_interval != scrapeInterval) {

            console.log(`new scrape interval found (before: ${scrapeInterval})`);

            scrapeInterval = Math.max(data.scrape_interval, MIN_SCRAPE_INTERVAL);

            console.log(`new scrape interval: ${scrapeInterval}`);
            
            stopScraping();
            startScraping();
        }

        console.log("last update timestamp: " + lastUpdateTimestamp);
        console.log("last seen timestamp: " + lastSeenTimestamp);

        if (lastUpdateTimestamp !== null && lastUpdateTimestamp >= lastSeenTimestamp) {

            console.log("new preview found!");
            console.log("reloading...");

            window.location.reload();
        }
    } catch (error) {
        console.error('error occurs during check preview updates:', error);
    }
}


startScraping();