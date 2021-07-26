<script lang='ts'>
	import rust from '../../Cargo.toml';
	import Dropzone from './Dropzone.svelte';

	let wasm;

	async function init() {
		wasm = await rust();
	}

	init();

	let current_stage = 'select';
	function notThisPage(target) {
		return current_stage == target ? "invisible" : "visible"
	}

	async function onFilesSelected(event) {
		wasm.parse_subtitle(event.detail.content, event.detail.extention);
	}
</script>

<main class='flex flex-col h-full w-full'>
	<h1 class='text-7xl text-red-400 uppercase font-thin text-center'>Japanese Vocab Builder</h1>
	<h2 class='text-4xl text-gray-600 uppercase text-center mb-5'>Learn Words which <em>Matter</em></h2>
	<Dropzone on:file_read={onFilesSelected}/>
	<div class='flex justify-between m-5'>
		<button class='self-start place-self-start border-0 text-7xl {notThisPage('select')}'><i class='fas fa-arrow-alt-circle-left'></i></button>
		<button class='self-end place-self-end border-0 text-7xl {notThisPage('analyse')}'><i class='fas fa-arrow-alt-circle-right'></i></button>
	</div>
</main>

<style global>
	@tailwind base;
	@tailwind components;
	@tailwind utilities;
</style>