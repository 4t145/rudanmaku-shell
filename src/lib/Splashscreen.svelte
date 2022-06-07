<script lang="ts">
    import {listen} from '@tauri-apps/api/event';
    import { onMount } from 'svelte';
    onMount(()=>{
        listen('SPLASHSCREEN_UPDATE', (evt)=>{
            msg = evt.payload as string;
        })
    })
    let msg:string;
</script>

<div>
    {msg}
</div>