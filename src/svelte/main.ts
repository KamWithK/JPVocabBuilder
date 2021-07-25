// @ts-ignore
import rust from "../../Cargo.toml";
import App from './App.svelte';

const init = async() => {
  const wasm = await rust();
  wasm.set_panic_hook(); // better debug messages

  const app = new App({
    target: document.body,
    props: {}
  });

}

init();
