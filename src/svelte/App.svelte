<script lang="ts">
	import rust from '../../Cargo.toml';
	import Dropzone from './Dropzone.svelte';

	let wasm;

	async function init() {
		wasm = await rust();
	}

	init();

	async function onFilesSelected(event) {
		wasm.parse_subtitle(event.detail.content, event.detail.extention);
	}
</script>

<main>
	<h1>Japanese Vocab Builder</h1>
	<h2>Learn Words which <em>Matter</em></h2>
	<Dropzone on:file_read={onFilesSelected}/>
</main>

<style global>
	@tailwind base;
	@tailwind components;
	@tailwind utilities;
	
	main {
		display: flex;
		flex-direction: column;
		width: 100%;
		height: 100%;
		padding: 0 em;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		text-align: center;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
		margin-bottom: 0;
	}

	h2 {
		text-align: center;
		text-transform: uppercase;
		font-size: 2em;
		font-weight: 500;
		margin-top: 0;
	}
</style>