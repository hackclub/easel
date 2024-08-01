var file_lastLoad = null;
var file_modified = false;
function newPath(){
    document.getElementById("quicksave-path").innerText = file_lastLoad;
}
function saveFile(quickSave = false){
    var sf = (path) => {
        const fileContent = document.getElementById('editor').value;
        fs.writeFileSync(path, fileContent);
        file_lastLoad = path;
        newPath()
        file_modified = false;
    } 
    if (quickSave == true){
        if (file_lastLoad == null){
            setOutput("Error: Unable to quicksave as no file is opened.", "error")
            return;
        }
        else{
            sf(file_lastLoad)
            document.getElementById("output-box").innerHTML = "Quicksave success!"
            return;
        }
    }
    
    const fileName = document.getElementById('file-name').value;
    sf(fileName)
    document.getElementById('save-modal').style.display = 'none';
}
function loadFile(){
    
    const file = document.getElementById('load-file-selector').files[0];
    const path = file.path;
    file_lastLoad = path;
    newPath();
    const reader = new FileReader();
    reader.onload = function() {
        console.log(reader.result);
        const e = document.getElementById('editor')
        e.value = reader.result;
        document.getElementById('load-modal').style.display = 'none';
        document.getElementById("highlighted-code").innerHTML = highlight(e.value)
        file_modified = false;
    }
    reader.readAsText(file);
}
document.getElementById('save-file').addEventListener('click', saveFile);
document.getElementById('load-file').addEventListener('click', loadFile);