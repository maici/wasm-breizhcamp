import init, { parse_markdown } from 'lib';
import * as monaco from 'monaco-editor';

const render = document.getElementById("render");

const editor = monaco.editor.create(document.getElementById('editor'), {
    language: 'markdown',
    theme: "vs-dark",
    value: `# Hello Breizhcamp
![hello-img](https://cdna.artstation.com/p/assets/images/images/046/611/722/large/pet-lover-hulk-1.jpg?1645537751)
> this is generated from webassembly`,
})

function renderHtml() {
    const html = parse_markdown(editor.getValue());
    render.innerHTML = html;
}

init().then(() => {
    
    renderHtml();

    editor.getModel().onDidChangeContent(() => {
        renderHtml();
    });  

}).catch(console.error);
