<script lang="ts">
    import Tab, {Label} from '@smui/tab';
    import TabBar from '@smui/tab-bar';
    import CoreConsole from './tabpages/CoreConsole.svelte';
    import Webapp from './tabpages/Webapp.svelte';
    import {listen} from '@tauri-apps/api/event';
    import { onMount } from 'svelte';
    import Splashscreen from './lib/Splashscreen.svelte';
    let tabs = ['CoreConsole', 'WebApp'];
    let tab_active:'CoreConsole'|'WebApp' = 'CoreConsole';

    let show_splash_screen:boolean = false;

    onMount(()=>{

        listen('SPLASHSCREEN_CLOSE', (_) => {
            show_splash_screen = false;
        })
        listen('SPLASHSCREEN_OPEN', (_) => {
            show_splash_screen = true;
        })
    })

</script>

<main>
  <TabBar tabs={tabs} let:tab bind:active={tab_active}>
    <!-- Note: the `tab` property is required! -->
    <Tab {tab}>
      <Label>{tab}</Label>
    </Tab>
  </TabBar>
  <div id="content">
    <div style={tab_active==='CoreConsole'?'':'display:none'}>
        <CoreConsole></CoreConsole>
    </div>
    <div style={tab_active==='WebApp'?'':'display:none'}>
        <Webapp/>
    </div>
  </div>
</main>
{#if show_splash_screen}
<div id="splash-screen">
    <Splashscreen></Splashscreen>
</div>
{/if}
<style>
  :root {

  }
  #splash-screen {
    position: absolute;
    display: block;
  }
  main {
    text-align: center;
    padding: 1em;
    margin: 0 auto;
  }
</style>
