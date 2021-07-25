<script lang="ts">
    import '@fortawesome/fontawesome-free/js/brands.js';
    import '@fortawesome/fontawesome-free/js/solid.js';
    import '@fortawesome/fontawesome-free/js/fontawesome.js';

    import { createEventDispatcher } from 'svelte';

    const dispatch = createEventDispatcher();

	async function handleFiles(event) {
        let files = event instanceof FileList ? event : event.target.files;

        for (var index = 0; index < files.length; index++) {
			let file = files[index];
			const extention = file.name.substring(file.name.lastIndexOf('.') + 1);

			let reader = new FileReader();
			reader.readAsText(file);

            reader.onload = e => dispatch('file_read', {
                content: e.target.result,
                extention: extention
            });
		}
	}

    function update(event) {
        event.stopPropagation();
        event.preventDefault();
    }

    function drop(event) {
        update(event);

        handleFiles(event.dataTransfer.files);
    }
</script>

<main>
    <input type="file" id="fileInput" multiple class="visually-hidden" on:change={handleFiles}/>
    <label id="dropzone" for="fileInput" on:dragenter={update} on:dragover={update} on:drop={drop}>
        <h1>Drag Your Subtitles In for Analysis</h1>
        <i id="symbol" class="fas fa-folder-open" data-fa-transform="rotate-20 down-1"></i>
    </label>
</main>

<style>
    main {
        flex-grow: 1;
    }

	.visually-hidden {
		position: absolute !important;
		height: 1px;
		width: 1px;
		overflow: hidden;
		clip: rect(1px, 1px, 1px, 1px);
	}

	input.visually-hidden:focus + label {
		outline: thin dotted;
	}

	input.visually-hidden:focus-within + label {
		outline: thin dotted;
	}

    #dropzone {
        height: 100%;
        width: 100%;
		text-align: center;
    }

    #symbol {
        font-size: 50vmin;
    }
</style>