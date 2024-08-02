const { app, BrowserWindow, ipcMain, ipcRenderer } = require('electron');
const os = require('os');
const path = require('path');
const fs = require('fs');
function createWindow() {
    const win = new BrowserWindow({
        width: 800,
        height: 600,
        webPreferences: {
            contextIsolation: false,
            enableRemoteModule: false,
            nodeIntegration: true,
        },
        
    });

    win.loadFile('src/index.html');
}

app.whenReady().then(createWindow);

app.on('window-all-closed', () => {
    if (process.platform !== 'darwin') {
        app.quit();
    }
});

app.on('activate', () => {
    if (BrowserWindow.getAllWindows().length === 0) {
        createWindow();
    }
});


ipcMain.on('write-file', (event, data, p) => {
    fs.writeFileSync(path.join(__dirname, p), data);
});

ipcMain.on('run-cmd', (event, cmd) => {
    const { exec } = require('child_process');
    execSync(cmd, (error, stdout, stderr) => {
        if (error) {
            event.returnValue = error.message;
            return;
        }
        if (stderr) {
            event.returnValue = stderr;
            return;
        }
        event.returnValue = stdout;
    });
});


ipcMain.on('read-file', (event, path) => {
    event.returnValue = fs.readFileSync(path, 'utf-8');
});

ipcMain.on('get-compiler-path', (event) => {
    const compilerPath = JSON.parse(fs.readFileSync(path.join(__dirname, 'config.json'), 'utf-8'))["compilerPath"];
    if (compilerPath == "auto") {
        const os = require('os');
        if (os.platform() == "win32") {
            event.returnValue = path.join(__dirname, 'lppc.exe');
        }
        else {
            event.returnValue = path.join(__dirname, 'lppc');
        }
    }
    event.returnValue = compilerPath;
});

ipcMain.on('get-compiled-code', (event) => {
    // Run command
    let compilerPath = JSON.parse(fs.readFileSync(path.join(__dirname, 'config.json'), 'utf-8'))["compilerPath"];
    if (compilerPath == "auto") {
        const os = require('os');
        if (os.platform() == "win32") {
            compilerPath = path.join(__dirname, 'fnhell.exe');
        }
        else {
            compilerPath = path.join(__dirname, 'fnhell');
        }
    }
    const codePath = path.join(__dirname,"_tmp_lppcode.lpp")
    console.log("executing: ", compilerPath, codePath);

    const compiledCodePath = path.join(__dirname, './_tmp_lppcompiledcode.js');
    const command = `${compilerPath} ${codePath} -o ${compiledCodePath}`;

    const { execSync } = require('child_process');
    execSync(command, (error, stdout, stderr) => {
        if (error) {
            console.log(`Error: ${error.message}`);
            event.returnValue = `console.log(\`Error: ${error.message}\`);`;
            return;
        }
        else if (stderr) {
            console.log(`Stderr: ${stderr}`);
            event.returnValue = `console.log(\`Error: ${stderr}\`);`;
            return;
        }
        console.log(`Output: ${stdout}`);
        console.log("Command executed: ", command);

        const compiledCode = fs.readFileSync(compiledCodePath, 'utf-8');
        event.returnValue = compiledCode;
    })

});

