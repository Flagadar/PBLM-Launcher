const { invoke } = window.__TAURI__.tauri;
const { listen } = window.__TAURI__.event;

let logEl;

async function launch() {
    logEl = document.querySelector("#launcher-log");
    await listen("launcher-log", (e) => {
        logEl.textContent = e.payload;
    });
    await invoke("launch");
}

window.addEventListener("DOMContentLoaded", () => {
        document.querySelector("#launch-btn").addEventListener("click", (e) => {
        e.preventDefault();
        launch();
    });
});

//let greetInputEl;
//let greetMsgEl;

//async function greet() {
//  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
//}

//window.addEventListener("DOMContentLoaded", () => {
//  greetInputEl = document.querySelector("#greet-input");
//  greetMsgEl = document.querySelector("#greet-msg");
//  document.querySelector("#greet-form").addEventListener("submit", (e) => {
//    e.preventDefault();
//    greet();
//  });
//});
