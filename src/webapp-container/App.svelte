<script lang="ts">
    import { appWindow, LogicalSize} from '@tauri-apps/api/window'
    import { emit, listen } from '@tauri-apps/api/event'
    import { Icon } from '@smui/common';
    import { onMount } from 'svelte';
    let search = new URLSearchParams(window.location.search);
    let ih: number;
    let iw: number;
    let oh: number;
    let url;
    let locked:boolean = false;
    $: url = decodeURI(search.get('url'));
    $: appWindow.setSize(new LogicalSize(iw, ih));
    onMount(()=>{
      listen('WEBAPP_UNLOCK', ()=>{
        unlock()
      })
    })
    function lock() {
      locked = true;
      emit('WEBAPP_LOCK')
    }
    function unlock() {
      locked = false;
    }
</script>
<div data-tauri-drag-region class="titlebar">
  {#if locked}
    <div class="titlebar-button" id="titlebar-unlock" on:click={unlock}>
      <Icon class="material-icons">lock</Icon>
    </div>
  {:else}
    <div class="titlebar-button" id="titlebar-lock" on:click={lock}>
      <Icon class="material-icons">lock_open</Icon>
    </div>
    <div class="titlebar-button" id="titlebar-minimize" on:click={()=>appWindow.minimize()}>
      <Icon class="material-icons">minimize</Icon>
    </div>
    <div class="titlebar-button" id="titlebar-close" on:click={()=>appWindow.close()}>
      <Icon class="material-icons">close</Icon>
    </div>
  {/if}
</div>
<iframe title="" src={url} id="viewport" height={ih-40} width={iw} bind:offsetHeight={oh} ></iframe>
<svelte:window bind:innerHeight={ih} bind:innerWidth={iw}></svelte:window>
<style>
    .titlebar {
        height: 30px;
        background: rgb(0, 172, 66);
        user-select: none;
        display: flex;
        border: none;
        border-radius: 8px 8px 0px 0px;
        justify-content: flex-end;
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
    }
    .titlebar-button {
        display: inline-flex;
        justify-content: center;
        align-items: center;
        width: 30px;
        height: 30px;
    }
    .titlebar-button:hover {
        background-color: aquamarine;
    }
    #viewport {
      background-color: transparent;
      border: none;
      top: 30px;
      left: 0;
      right: 0;
      bottom: 0;
      position: absolute;
      overflow-y: hidden;
    }
</style>