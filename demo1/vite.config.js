import { defineConfig } from 'vite';
import monacoEditorPlugin from "vite-plugin-monaco-editor";
import wasmPack from 'vite-plugin-wasm-pack';

export default defineConfig({
  plugins: [
    wasmPack('./lib'),
    monacoEditorPlugin(),
  ]
});