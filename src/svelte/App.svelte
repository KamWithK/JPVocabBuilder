<script lang='ts'>
	import Analysis from './Analysis.svelte';
	import Dropzone from './Dropzone.svelte';

	export let wasm;

	let current_stage = 'select';
	function switchPage(target) {
		current_stage = target;
	}

	let total_files, processed;

	async function onFilesSelected(event) {
		wasm.parse_subtitle(event.detail);
	}
</script>

<main class='flex flex-col h-full w-full'>
	<h1 class='text-7xl text-red-400 uppercase font-thin text-center'>Japanese Vocab Builder</h1>
	<h2 class='text-4xl text-gray-600 uppercase text-center mb-5'>Learn Words which <em>Matter</em></h2>
	{#if current_stage == 'select'}
		<Dropzone on:file_read={onFilesSelected} total_files={total_files} processed={processed}/>
	{:else if current_stage == 'analyse'}
		<Analysis wasm={wasm}/>
	{/if}
	<div class='flex justify-between m-5'>
		<button class='self-start place-self-start border-0 text-7xl' class:invisible={current_stage == 'select'} on:click={() => switchPage('select')}><i class='fas fa-arrow-alt-circle-left'></i></button>
		<button class='self-end place-self-end border-0 text-7xl' class:invisible={current_stage == 'analyse'} on:click={() => switchPage('analyse')}><i class='fas fa-arrow-alt-circle-right'></i></button>
	</div>
</main>

<style global>
	@tailwind base;
	@tailwind components;
	@tailwind utilities;
</style>