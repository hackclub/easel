const { app, BrowserWindow, contextBridge, ipcRenderer } = require('electron');

app.whenReady().then(() => {
    const mainWindow = new BrowserWindow({
        webPreferences: {
            contextIsolation: false,
        },
    });

});


contextBridge.exposeInMainWorld('electron', {
    writeFile: (data, path) => {
        ipcRenderer.send('write-file', data, path);
    },
    readFile: (path) => {
        return ipcRenderer.sendSync('read-file', path);
    },
    getCompilerPath: () => {
        return ipcRenderer.sendSync('get-compiler-path');
    },
    getCompiledCode: () => {
        const compilerPath = JSON.parse(fs.readFileSync(path.join(__dirname, 'config.json'), 'utf-8'))["compilerPath"];
        if (compilerPath == "auto") {
            const os = require('os');
            if (os.platform() == "win32") {
                compilerPath = path.join(__dirname, 'lppc.exe');
            }
            else {
                compilerPath = path.join(__dirname, 'lppc');
            }
        }
        const codePath = path.join(__dirname,"_tmp_lppcode.lpp")
        console.log("executing: ", compilerPath, codePath);

        const compiledCodePath = path.join(os.tmpdir(), '_tmp_lppcompiledcode.js');
        const command = `${compilerPath} ${codePath} -o ${compiledCodePath}`;

        const { exec } = require('child_process');
        exec(command, (error, stdout, stderr) => {
        
            if (error) {
                console.error(`Error: ${error.message}`);
                return `console.log(\`Error: ${error.message}\`);`;
            }
            else if (stderr) {
                console.error(`Stderr: ${stderr}`);
                return `console.log(\`Error: ${stderr}\`);`;
            }
            else {
                console.log(`Output: ${stdout}`);
                
                return fs.readFileSync(compiledCodePath, 'utf-8');;
            }
        });

        
    },
    runCmd: (cmd) => {
        const { exec } = require('child_process');
        exec(cmd, (error, stdout, stderr) => {
            if (error) {
                return error.message;
            }
            if (stderr) {
                return stderr;
            }
            return stdout;
        });
    },
});

