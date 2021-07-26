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

<main class="flex flex-col h-full w-full">
	<h1 class='text-7xl text-red-400 uppercase font-thin text-center'>Japanese Vocab Builder</h1>
	<h2 class='text-4xl text-gray-600 uppercase text-center mb-5'>Learn Words which <em>Matter</em></h2>
	<Dropzone on:file_read={onFilesSelected}/>
</main>

<style global>
	@tailwind base;
	@tailwind components;
	@tailwind utilities;
</style>