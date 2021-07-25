<script lang="ts">
	import rust from "../../Cargo.toml";

	async function onFilesSelected(e) {
		let files = e.target.files;
		const wasm = await rust();

		for (var index = 0; index < files.length; index++) {
			let file = files[index];
			const extention = file.name.substring(file.name.lastIndexOf('.') + 1);

			let reader = new FileReader();
			reader.readAsText(file);
			reader.onload = e => wasm.parse_subtitle(e.target.result, extention);
		}
	}
</script>

<main>
	<h1>Japanese Vocab Builder</h1>
	<input type="file" id="fileInput" multiple on:change={onFilesSelected}/>
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>