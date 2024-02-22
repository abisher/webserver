// const {app, BrowserWindow} =  import('electron').then();
// const path =  import("path");
// const isDev =  import("electron-is-dev");

async function load() {
    let {app, BrowserWindow} = await import('electron');
    let path = await import("path");
    let isDev = await import("electron-is-dev");

    return [app, BrowserWindow, path, isDev];
}

let [app, BrowserWindow, path, isDev] = await load();

console.log(app, BrowserWindow);
function createWindow() {
    const mainWindow = new BrowserWindow({
        width: 800,
        height: 600,
        webPreferences: {
            nodeIntegration: true,
            enableRemoteModule: true,
            contextIsolation: false
        },
    });

    mainWindow.loadURL(
        isDev
            ? "http://localhost:3000"
            : `file://${path.join(__dirname, "../build/index.html")}`
    );

    if (isDev) {
        mainWindow.webContents.openDevTools();
    }
}

app.whenReady().then(() => {
    createWindow();
    app.on("activate", function () {
        if (BrowserWindow.getAllWindows().length === 0) {
            createWindow();
        }
    })
})


app.on("window-all-closed", function() {
    if (process.platform !== "darwin") {
        app.quit();
    }
})