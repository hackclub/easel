import { Types } from "./parser.js";

export class CodeGen {
  constructor(ast) {
    this.ast = ast;

    this.css = `
#container,#container>button,#container>button:hover{font-family:"Special Elite",system-ui}body{width:100vw;height:100vh;overflow-x:hidden;margin:5rem 10rem}#container{width:calc(100vw - 20rem);font-size:20px}#container>*{margin-top:0;margin-bottom:7px}#container>button{width:100%;border:none;background:0 0;color:#414141;font-size:15px}#container>button,.end{animation:.6s cubic-bezier(.38,.97,.56,.76) .1s forwards show;opacity:0;transform:rotateX(-90deg);transform-origin:top center}#container>button:hover{color:#4b4b4b;font-size:18px}@keyframes show{100%{opacity:1;transform:none}}@media only screen and (max-width:40rem){body{margin-left:5rem;margin-right:5rem}#container{width:calc(100vw - 10rem)}}
`.trim();
    this.js = `
window.container=document.getElementById("container"),window.writingList=[],window.writingControl=()=>{let t=[""];var e,n=0,i=t[0].length,r=0,l="";function o(s){l=" ",e=Math.max(0,n-20);for(var a=s;e<n;)l+=t[e++]+"<br />";a.innerHTML=l+t[n].substring(0,r)+"_",r++==i?(r=0,++n!=t.length?(i=t[n].length,setTimeout(()=>{o(s)},500)):(a.innerHTML=l+t[n-1].substring(0,i),c=!0,window.writingList.shift())):setTimeout(()=>{o(s)},50)}let c=!0;function s(){setTimeout(function(){if(c&&window.writingList[0]){if(c=!1,"text"===window.writingList[0].type){let e=document.createElement("p");window.writingList[0].color?(e.style.color=window.writingList[0].color,container.appendChild(document.createElement("br")),container.appendChild(e),container.appendChild(document.createElement("br"))):container.appendChild(e),t=[window.writingList[0].content],l="",r=0,n=0,i=t[0].length,o(e)}"choice"===window.writingList[0].type&&(container.appendChild(window.writingList[0].button),window.writingList.shift(),c=!0)}else c&&!window.writingList[0]&&0===Array.prototype.slice.call(container.getElementsByTagName("button"),0).length&&window.end();s()},0)}s()},window.text=(t,e)=>{writingList.push({type:"text",content:t,color:e})},window.choice=(t,e)=>{let n=document.createElement("button");n.innerHTML=t,n.onclick=()=>{Array.prototype.slice.call(container.getElementsByTagName("button"),0).forEach(t=>{t.remove()}),window.text(t,"gray"),e()},writingList.push({type:"choice",button:n})},window.diversion=t=>{if(t)t();else throw"Error: Section"+t+"is undefined"},window.end=()=>{let t=document.createElement("p");throw t.classList.add("end"),t.innerHTML="--- The End ---",container.appendChild(document.createElement("br")),container.appendChild(t),"Thanks for playing!!!"},window.writingControl();
`.trim();

    this.jsList = [];
  }

  push(node, list) {
    if (node.type === Types["Section"]) {
      list.push(`function ${node.name}(){`);
      node.body.forEach((child) => {
        this.push(child, list);
      });
      list.push(`}`);
    } else if (node.type === Types["Text"]) {
      list.push(`text(\`${node.content}\`);`);
    } else if (node.type === Types["Choice"]) {
      let choiceList = [];
      node.body.forEach((child) => {
        this.push(child, choiceList);
      });
      node.body = choiceList.join("");
      list.push(`choice(\`${node.content}\`,()=>{${node.body}});`);
    } else if (node.type === Types["Diversion"]) {
      list.push(`diversion(${node.section});`);
    } else if (node.type === Types["Var"]) {
      list.push(
        `try{${node.name} = ${node.value}}catch{'Error: Struggling with variable ${node.name}'}`,
      );
    } else if (node.type === Types["If"]) {
      list.push(`if(${node.condition.slice(0, -1)}){`);
      node.body.forEach((child) => {
        this.push(child, list);
      });
      list.push(`}`);
    }
  }

  generate() {
    this.ast.forEach((node) => {
      this.push(node, this.jsList);
    });

    this.js += "(()=>{";

    this.jsList.forEach((node) => {
      this.js += node;
    });

    this.js += "})()";

    this.html = `<!DOCTYPE html><html lang="en"><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Iris</title><link rel="preconnect" href="https://fonts.googleapis.com"><link rel="preconnect" href="https://fonts.gstatic.com" crossorigin><link href="https://fonts.googleapis.com/css2?family=Special+Elite&display=swap" rel="stylesheet"><style>${this.css}</style></head><body><div id="container"></div><script>${this.js}</script></body></html>`;
  }
}
