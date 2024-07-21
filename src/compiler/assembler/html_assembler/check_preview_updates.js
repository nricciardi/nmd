const url = 'http://127.0.0.1:1234/check-preview-updates';

let lastDate = null;

async function checkPreviewUpdates() {

    console.log("checking preview updates...");

    try {
        const response = await fetch(url);
        if (!response.ok) {
            throw new Error('network response was not ok');
        }

        const data = await response.json();
        const newDate = new Date(data.date);

        if (lastDate === null || newDate > lastDate) {
            lastDate = newDate;
            
            console.log("new preview found!");
            console.log("reloading...");

            window.location.reload();
        }
    } catch (error) {
        console.error('error occurs during check preview updates:', error);
    }
}

const interval = 1000;

setInterval(checkPreviewUpdates, interval);
