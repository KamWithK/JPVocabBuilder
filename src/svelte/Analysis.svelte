<script lang='ts'>
    export let wasm;
	let words = JSON.parse(wasm.get_best());
</script>

<div class='flex flex-col self-center text-center'>
	{#each words as word}
		<div class='flex flex-row justify-between cursor-pointer rounded-lg m-3 bg-gradient-to-r from-red-600 via-pink-300 to-red-600 text-5xl'>
			<p class='p-3'>{word['word']}</p>
			<p class='rounded-r-lg w-36 p-3 bg-red-400'>{word['frequency']}</p>
		</div>
	{/each}
</div>

<style global>
	@tailwind base;
	@tailwind components;
	@tailwind utilities;
</style>