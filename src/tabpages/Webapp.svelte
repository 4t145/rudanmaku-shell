<script lang="ts">
    // import Console from "../lib/Console.svelte";
    // import {listen} from "@tauri-apps/api/event";
    import { onMount } from "svelte";

    import {BaseDirectory, readDir} from '@tauri-apps/api/fs';
    import WebappItem from '../lib/WebappItem.svelte';

    onMount(()=>{})

    async function read_webapps():Promise<string[]>{
        let app_list = [];
        let entries = await readDir('webapp', {dir:BaseDirectory.Resource});

        for(const entry of entries) {
            if(entry.children !== null && entry.children !== undefined) {
                app_list.push(entry.path);
            }
        }
        return app_list;
    }
</script>


{#await read_webapps()}
    loading...
{:then app_list} 
    {#each app_list as webapp_path}
    <WebappItem path={webapp_path}></WebappItem>
    {/each}
{/await}


<style>

</style>