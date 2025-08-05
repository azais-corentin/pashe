<script lang="ts">
  // import { invoke } from "@tauri-apps/api/core";

  async function greet() {
    // This calls the Rust function 'greet' on the local machine
    // const message = await invoke("greet", { name: "from Web" });
    // console.log(message);
  }
</script>

<button on:click={greet}>Call Rust API</button>
