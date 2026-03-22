const {app, BrowserWindow} = require('electron')
const path = require('path')

// URL is configurable via PENPOT_URL environment variable.
// Default: http://localhost:9001
const PENPOT_URL = process.env.PENPOT_URL || 'http://localhost:9001'

function createWindow () {
  const mainWindow = new BrowserWindow({
    width: 3840,
    height: 2160,
    icon: path.join(__dirname, '../../resources/penpot.jpeg'),
    title: "Penpot",
    fullscreen: false,
    autoHideMenuBar: true,
    transparent: 'true',
    webPreferences: {
      preload: path.join(__dirname, '../renderer/preload.js')
    }
  })
  mainWindow.setMenuBarVisibility(false)
  mainWindow.loadURL(PENPOT_URL)
}

app.whenReady().then(() => {
  createWindow()

  app.on('activate', function () {
    if (BrowserWindow.getAllWindows().length === 0) createWindow()
  })
})


app.on('window-all-closed', function () {
  if (process.platform !== 'darwin') app.quit()
})

