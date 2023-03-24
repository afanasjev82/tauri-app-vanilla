const { invoke } = window.__TAURI__.tauri;
import { getMatches } from '@tauri-apps/api/cli';

let greetInputEl;
let greetMsgEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document
    .querySelector("#greet-button")
    .addEventListener("click", () => greet());
});

//window.location.replace("https://google.com")


getMatches().then((matches) => {
  // do something with the { args, subcommand } matches
  greetMsgEl.textContent = JSON.stringify(null,2,matches.args);
})