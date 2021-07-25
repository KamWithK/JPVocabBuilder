<script lang="ts">
	import rust from '../../Cargo.toml';
	import Dropzone from './Dropzone.svelte';

	async function onFilesSelected(event) {
		const wasm = await rust();
		wasm.parse_subtitle(event.detail.content, event.detail.extention);
	}
</script>

<main>
	<h1>Japanese Vocab Builder</h1>
	<Dropzone on:file_read={onFilesSelected}/>
</main>

<style>
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
	}
</style>