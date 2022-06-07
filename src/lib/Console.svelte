<script lang="ts">
    export let lines:Array<string> = [];
</script>


<div id="console">
    {#each lines as line}
        <code>{line}</code><br/>
    {/each}
</div>
<style>
    #console {
        font-family: 'Sarasa Mono CL', 'monospace';
        text-align: left;
        background-color: #fffff0;
        height: 100%;
        width: 100%;
        color: #2e2a50;
    }
</style>