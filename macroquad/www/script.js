let romData = "";

let uploadDivElem = document.getElementById("rom-upload-div");
let inputElem = document.getElementById("rom-upload");
let canvasElem = document.getElementById("glcanvas");

function startEmulator(e) {
    romData = e.target.result;

    uploadDivElem.remove();
    canvasElem.style.visibility = "visible";

    load("rustyboy_macroquad.wasm");
}

function handler() {
    let f = this.files[0];

    let reader = new FileReader();
    reader.onload = startEmulator;
    reader.readAsBinaryString(f);
}

inputElem.addEventListener("change", handler);

function register_plugin(importObject) {
    importObject.env.get_rom_data = () => {
        return js_object(romData);
    }
}

miniquad_add_plugin({ register_plugin, function() { } });
