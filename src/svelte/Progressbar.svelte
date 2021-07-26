<script lang='ts'>
    export let total_files: number;
    export let processed: number;
</script>

<progress class='h-5 w-2/5 self-center mt-20' value={total_files == 0 ? 0 : Math.round((processed / total_files) * 100) / 100}/>
<em>Processed {processed}/{total_files} Files</em>

<style>
</style>