
const { ipcRenderer, app } = require("electron");
const { exec, execSync } = require('child_process');
const fs = require("fs")
const path = require('path');
const os = require('os');
const keywords = "int float bool string void fn if else ret var with".split(" ");
const operators = "+ - * / % = == != < > <= >= && || !".split(" ");
const delimiters = " ( ) { } [ ] , ;".split(" ");
const openDelimiters = " ( { [".split(" ");
const closeDelimiters = " ) } ]".split(" ");
const literals = "true false".split(" ");

let foundNames = []

const delimiterMap = {
    "(": ")",
    "{": "}",
    "[": "]",
}   
var fontSize = 16;

const highlightedCode = document.getElementById('highlighted-code');
const outputBox = document.getElementById("output-box")
const editor = document.getElementById('editor');

editor.addEventListener('input', updateSuggestionHover);
document.addEventListener('click', updateSuggestionHover);
editor.addEventListener('keyup', updateSuggestionHover);


var curSuggestion = "";
var holdingControl = false;

var sampleProgram = `var void fizzbuzz = void <int n, int j> { // The function fizzbuzz returns void and takes two integer functions n and j as arguments
    if (j % 15 == 0) { // If statement: if j mod 15 is 0 then execute the body
        log with (string <> { ret "FizzBuzz\\n" }) // The body: calls the function "log" with a function argument that returns "FizzBuzz
    }
    else if (j % 5 == 0){ // If the previous if was false, check if j mod 5 is 0, if it is execute the body
        log with (string <> { ret "Buzz\\n" }) // call "log" with a function that returns "Buzz" 
    }
    else if (j % 3 == 0){
        log with (string <> { ret "Fizz\\n" })
    }
    else {
        log with (int <> { ret ^j }, string <> { ret "\\n"})
    }
    if (j < n) {
        ^fizzbuzz with (int <> { ret ^n } , int <> { ret ^j + 1 }) // Recursive call
    }
}
fizzbuzz with (int <> { ret 100 }, int <> { ret 0 }) // Recursively call the fizzbuzz function with n = 100 and j = 0
`

function isNodeInstalled() {
    return new Promise((resolve, reject) => {
        exec('node -v', (error, stdout, stderr) => {
            if (error | stderr) {
                console.error(`Error: ${error.message}`);
                resolve(false); // Not installed
            } else {
                setOutput(`Node.js version: ${stdout}`, "text");
                resolve(true); // Installed
            }
        });
    });
}
function disableRunButton() {
    const rb = document.getElementById("run-button")
    rb.disabled = true;
    rb.style.cursor = "not-allowed";
}

function setThemeColors(){
    const file = fs.readFileSync(path.join(__dirname, 'themes/theme.json'), 'utf-8')
    const theme = JSON.parse(file)
    const root = document.documentElement;

    for (let key in theme){
        const color = theme[key];
        root.style.setProperty(`--${key}-color`, `rgb(${color[0]}, ${color[1]}, ${color[2]})`);
    }

}

window.addEventListener("DOMContentLoaded", (e)=>{
    isNodeInstalled().then((installed) => {
        if (!installed){
            disableRunButton();
            setOutput("Node.js is not installed. Please install Node.js from <a style='color: yellow;' href='https://nodejs.org'>https://nodejs.org</a>", "error", raw=true);
        }
        editor.value = sampleProgram;
        updateFontSize()    
        highlightedCode.innerHTML = highlight(editor.value);
        updateSuggestionHover()
        suggest()
        setThemeColors()
    });
    
})

editor.addEventListener('input', (event) => {

    if (openDelimiters.includes(event.data)) {
        const start = editor.selectionStart;

        let cdAhead = delimiterMap[event.data] == editor.value[start]
        let odBehind = openDelimiters.includes(editor.value[start - 2])
        console.log(cdAhead, editor.value[start], odBehind, editor.value[start - 1])
        if (!cdAhead || (cdAhead && odBehind)) 
            editor.value = editor.value.substring(0, start) + closeDelimiters[openDelimiters.indexOf(event.data)] + editor.value.substring(start);
        editor.selectionStart = start;
        editor.selectionEnd = start;
    }
    highlightedCode.innerHTML = highlight(editor.value);
    
    document.querySelector("#hover-box").innerHTML = highlight(curSuggestion);
});
var setScroll = () => {
    highlightedCode.scrollTop = editor.scrollTop;
    highlightedCode.scrollLeft = editor.scrollLeft;
    document.querySelector("#numbers").scrollTop = editor.scrollTop;
}
editor.addEventListener('scroll', setScroll);

function updateSuggestionHover(){
    let pos = getCaretPosition(editor);
    let suggestionOffset = {
        left: 0, 
        top: fontSize + 5,
    }
    const elem = document.querySelector("#hover-box")
    elem.style.top = `${pos.top + suggestionOffset.top}px`;
    elem.style.left = `${pos.left + suggestionOffset.left}px`;
}

function removeLastWord(code, cursor) {
    let i = cursor - 1;
    while (i >= 0) {
        if (code[i] == " " || code[i] == "\n" || delimiters.includes(code[i]) || operators.includes(code[i])) {
            break;
        }
        i--;
    }
    return code.substring(0, i + 1);
}
function updateFontSize(){
    editor.style.fontSize = fontSize + "px";
    highlightedCode.style.fontSize = fontSize + "px";
    document.querySelector("#hover-box").style.fontSize = fontSize + "px";

    editor.style.left = `${fontSize}px`
    highlightedCode.style.left = `${fontSize}px`

    document.querySelector("#numbers").style.fontSize = `${fontSize}px`;
}
editor.addEventListener("keydown", (e) => {
    //curSuggestion = suggest();
    if (!file_modified && file_lastLoad != null){ 
        const qp = document.getElementById("quicksave-path");
        qp.innerText += " ●";
        file_modified = true;
    }

    
    if (e.key == "Tab") {
        e.preventDefault();
        const start = e.target.selectionStart;
        const end = e.target.selectionEnd;
        const selection = e.target.value.substring(start, end);
        const tab = curSuggestion == "" ? "    " : curSuggestion;
        if (curSuggestion != "") {
            e.target.value = removeLastWord(e.target.value, start) + tab + e.target.value.substring(end);
            e.target.selectionStart = start + tab.length;
            e.target.selectionEnd = start;
        }
        else {
            e.target.value = e.target.value.substring(0, start) + tab + selection + e.target.value.substring(end);
            e.target.selectionStart = start + tab.length;
            e.target.selectionEnd = start
        }
        // Set cursor position
        editor.focus();
        editor.setSelectionRange(start + tab.length, start + tab.length);
    }
    else if (holdingControl){
        // Up arrow
        if (e.key == "ArrowUp"){
            e.preventDefault()
            fontSize += 1;
            updateFontSize()
            return;
        }
        // Down arrow
        else if (e.key == "ArrowDown"){
            e.preventDefault()
            fontSize -= 1;
            updateFontSize()
            return;
        }
        else if (e.key.toLowerCase() == "s"){
            saveFile(true)
        }
        else if (e.key.toLowerCase() == "r") {
            setThemeColors()
        }
    }
    else if (e.key == "Control") {
        holdingControl = true;
    }
    updateSuggestionHover()
    highlightedCode.innerHTML = highlight(editor.value);
});

editor.addEventListener("keyup", (e) => {
    if (e.key == "Control") {
        holdingControl = false;
    }
    highlightedCode.innerHTML = highlight(editor.value);
});


function currentWord(code, cursor) {
    let word = "";
    let i = cursor - 1;
    while (i >= 0) {
        if (code[i] == " " || code[i] == "\n" || delimiters.includes(code[i]) || operators.includes(code[i])) {
            break;
        }
        word = code[i] + word;
        i--;
    }
    return word;
}

function closestTo(word, targets){
    let closest = -3;
    let cWord = "";
    if (word.length == 0) return ""
    for (let i = 0; i < targets.length; i++){
        w = targets[i];
        c = 0;
        if (word.length >= w.length){
            if (word[word.length - 1] != w[w.length - 1]) {
                c = -Infinity;
                continue;
            }
        }
        let errors = 0;
        for (let j = 0; j < Math.min(word.length, w.length); j++){
            if(w[j] == word[j]) c+=1;
            else errors++;
        }
        c = c - Math.abs(word.length - w.length) - errors*errors
        if (c > closest){
            closest = c;
            cWord = w;
            console.log("Found: ", cWord, c);
        }
    }
    return cWord;
}

function suggest() {
    const code = editor.value;
    const cursor = editor.selectionStart;
    const cw = currentWord(code, cursor);
    //console.log("Current word: ", cw, "Cursor: ", cursor, "Code: ", code);
    
    const c = closestTo(cw, keywords.concat(literals).concat(foundNames));
    if (c.length == 0){
        document.querySelector("#hover-box").style.display = 'none';
    }else{
        document.querySelector("#hover-box").style.display = 'flex';
    }
    return c 
}
function highlightWord(word){
    if (/^\d+/.test(word)) {
        return `<span class="number">${word}</span>`;
    } else if (/\d+\.\d+/.test(word)) {
        return `<span class="number">${word}</span>`;
    } else if (keywords.includes(word)) {
        return `<span class="keyword">${word}</span>`;
    } else if (operators.includes(word)) {
        const replace = {
            ">": "&gt;",
            "<": "&lt;",
            "&&": "&amp;&amp;",
            "||": "&#124;&#124;",
            "!": "&#33;",
            "==": "&#61;&#61;",
            "!=": "&#33;&#61;",
        }
        if (word in replace){
            return `<span class="operator">${replace[word]}</span>`;
        }
        return `<span class="operator">${word}</span>`;
    } else if (delimiters.includes(word)) {
        return `<span class="delimiter">${word}</span>`;
    } else if (literals.includes(word)) {
        return `<span class="literal">${word}</span>`;
    } else {
        //if (!(word in foundNames) && word.length > 1){
        //    foundNames.push(word)
        //}
        return word;
    }
}
let DP = {}
function highlightLine(line) {
    if (line in DP) 
        return DP[line];

    let highlightedLine = "";
    let word = "";
    let inWord = false;
    for (let i = 0; i < line.length; i++) {
        // "hello"
        // iter2, i = 7
        const char = line[i]; // "
        if (char == "/"){
            if (line[i+1] == "/"){
                highlightedLine += `<span class="comment">${line.substring(i)}</span>`
                break;
            }
        }
        if (char == "\""){
            if(word.length != 0){
                highlightedLine += highlightWord(word);
                word = "";
            }
            let j = i+1;

            let cstr = "\""
            while (line[j] != "\"" && j < line.length){
                if (line[j] == "\\"){
                    // Got escape char
                    j++;
                    switch (line[j]){
                        case "x":
                            j += 2;    
                            cstr += `<span class="escape-char">${line[j-3]+line[j-2]+line[j-1]+line[j]}</span>`
                            break;
                        default:
                            cstr += `<span class="escape-char">${line[j-1]+line[j]}</span>`
                            break;
                        
                    }

                }
                else cstr += line[j]
                j++;
            }
            // cstr = "hello
            i = j; // i = 7
            cstr += line[j] == "\"" ? "\"" : "" // adds 
            highlightedLine += `<span class="string">${cstr}</span>` // <span class="string">"hello"</span>
        }
        else if (operators.includes(char)) {
            if (inWord) {
                inWord = false;
                highlightedLine += highlightWord(word);
                word = "";
            }
            highlightedLine += highlightWord(char);
        }
        else if (char == " " || char == "\n" || delimiters.includes(char)) {
            if (inWord) {
                inWord = false;
                highlightedLine += highlightWord(word);
                word = "";
            }
            highlightedLine += char;
        } else {
            if (!inWord) {
                inWord = true;
            }
            word += char;
        }
    }
    if (inWord) {
        highlightedLine += highlightWord(word);
    }
    DP[line] = highlightedLine;
    return highlightedLine;
}

function getNewlines(code){
    let lines = code.split("\n");
    let newlines = "";
    for (let i = 0; i < lines.length; i++){
        newlines += i + 1 + "<br>";
    }
    return newlines + "<br>".repeat(1000);
}

function highlight(code) {
    let highlightedCode = "";
    const lines = code.split("\n");
    for (let i = 0; i < lines.length; i++) {
        highlightedCode += highlightLine(lines[i]);
        if (i != lines.length - 1) {
            highlightedCode += "\n";
        }
    }
    setScroll()
    //document.querySelector("#numbers").innerHTML = getNewlines(code)
    return highlightedCode + "\n".repeat(10);

}
function outputStyle(text, type, raw=false){
    let out = "";
    if (!raw) text = text.replace(/\n/g, "<br>").replace(/\t/g, "&nbsp;&nbsp;&nbsp;&nbsp;").replace(/ /g, "&nbsp;")
    console.log("Sanitized text: ", text)
    if (type == "text"){
        out = `<span class="output-text">${text}</span><br>`;
    }
    else if (type == "error"){
        out = `<span class="output-error">${text}</span><br>`;
    }
    else if (type == "warning"){
        out = `<span class="output-warning">${text}</span><br>`;
    }
    return out;
}
function setOutput(text, type, raw=false){
    document.querySelector("#output-box").innerHTML = outputStyle(text, type, raw=raw);
}

function appendOutput(text, type){
    document.querySelector("#output-box").innerHTML += outputStyle(text, type);
}

function getCompiledCode(){
    return new Promise((resolve, reject) => {
        appendOutput("Reading compiler path...\n", "text")
        let compilerPath = JSON.parse(fs.readFileSync(path.join(__dirname, 'config.json'), 'utf-8'))["compilerPath"];
        if (compilerPath == "auto") {
            try{
                
                appendOutput("Auto detecting compiler path in src dir...\n", "text")
                if (os.platform() == "win32") {
                    appendOutput("Windows detected...\n", "text")
                    compilerPath = path.join(__dirname, 'fnhell.exe');
                }
                else {
                    appendOutput("Linux detected...\n", "text")
                    compilerPath = path.join(__dirname, 'fnhell');
                }
                if (!fs.existsSync(compilerPath)){
                    throw "Compiler not found"
                }
            }
            catch(e){
                setOutput("Error: Unable to find fnhell in source directory, please set 'compilerPath' in config.json\n", "error")
            }
        }
        const codePath = path.join(__dirname,"_tmp_lppcode.lpp")
        const compiledCodePath = path.join(__dirname, './_tmp_lppcompiledcode.js');
        appendOutput("Compiling code...\n", "text")
        const command = `${compilerPath} ${codePath} -o ${compiledCodePath} --log`;
        
        exec(command, (error, stdout, stderr) => {
            if (error) {
                console.log(`Error: ${error.message}`);
                appendOutput(`Error: ${error.message}`);
                reject(error);
            }
            else if (stderr) {
                console.log(`Stderr: ${stderr}`);
                appendOutput(`Error: ${stderr}`);
                reject(stderr);
            }
            appendOutput(stdout, "text");
            const compiledCode = fs.readFileSync(compiledCodePath, 'utf-8');
            resolve(compiledCode);
        })
    });
}

document.querySelector("#run-button").addEventListener("click", () => {
    setOutput("Running code...\n", "text");
    const code = editor.value;
    fs.writeFileSync(__dirname + "/_tmp_lppcode.lpp", code);
    
    
    getCompiledCode().then((compiledCode) => {;
        // Run the compiled code
        let output = "There was an error, abort!";
        appendOutput("\nRunning compiled code...\n", "text");
        const cmd = `node ${__dirname}/_tmp_lppcompiledcode.js`;
        exec(cmd, (error, stdout, stderr) => {
            if (error) {
                output = error.message;
                console.error(`Error: ${error.message}`);
            }
            else if (stderr) {
                output = stderr;
                console.error(`Stderr: ${stderr}`);
            }
            else {
                output = stdout.replace(/\n/g, "<br>");
                console.log(`Output: ${stdout}`);
            }
            appendOutput(output, "text");
            outputBox.scrollTop = outputBox.scrollHeight
        });
    }).catch((error) => {
        console.error(error);
        appendOutput("Error: " + error, "error");
    });
});