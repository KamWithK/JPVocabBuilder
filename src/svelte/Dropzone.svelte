<script lang='ts'>
    import '@fortawesome/fontawesome-free/js/brands.js';
    import '@fortawesome/fontawesome-free/js/solid.js';
    import '@fortawesome/fontawesome-free/js/fontawesome.js';

    import { createEventDispatcher } from 'svelte';
    import Progressbar from './Progressbar.svelte';

    const dispatch = createEventDispatcher();
    let total_files = 0;
    let processed = 0;

	async function handleFiles(event) {
        let files = event instanceof FileList ? event : event.target.files;
        total_files += files.length;

        for (var index = 0; index < files.length; index++) {
			let reader = new FileReader();
			reader.readAsText(files[index]);

            reader.onload = e => {
                dispatch('file_read', e.target.result);

                processed += 1;
            };
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

<input type='file' id='fileInput' multiple class='hidden' on:change={handleFiles}/>
<label class='flex flex-col text-center h-full w-full pt-10 border-4 cursor-pointer rounded-lg hover:shadow-inner'
    for='fileInput' on:dragenter={update} on:dragover={update} on:drop={drop}>
    <div class='flex justify-around'>
        <div>
            <i class='symbol fas fa-closed-captioning'></i>
            <p class='font-semibold'>Drag in your Subtitles</p>
        </div>
        <div>
            <i class='symbol fas fa-file-contract'></i>
            <p class='font-semibold'>Analyse the Language</p>
        </div>
        <div>
            <i class='symbol fas fa-sort-amount-down-alt'></i>
            <p class='font-semibold'>Study Essential Words</p>
        </div>
    </div>

    <Progressbar total_files={total_files} processed={processed}/>
</label>

<style>
    .symbol {
        font-size: 30vmin;
    }
</style>