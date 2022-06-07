<script lang="ts">

    import CoreConfigPage from "../lib/CoreConfig.svelte";
    import Console from "../lib/Console.svelte";
    import {emit, listen} from "@tauri-apps/api/event";
    import { onMount } from "svelte";
    import Button , {Label} from '@smui/button';
    import DraggableFab  from '../lib/DraggableFab.svelte';
    import {readTextFile, BaseDirectory, writeFile, readDir, createDir} from '@tauri-apps/api/fs';
    import {appDir} from '@tauri-apps/api/path';
    import {save, open, message} from '@tauri-apps/api/dialog';
    import {type CoreConfig, dump, parse} from '../model/core-config';
    
    const DEFAULT_CORE_CONFIG:string = 
    `
    [net]
    ipv4=[127,0,0,1]
    port=10200
    [[room]]
    roomid=21672023
    channel=["json"]
    `
    const default_config:CoreConfig =  {
        net: {
            ipv4:[127,0,0,1],
            port: 10200,
        },
        room: []
    }
    let fab_left = 600;
    let fab_top = 100;
    let app_dir;
    let core_dir;
    let config: CoreConfig = default_config;
    let console_buf = [];
    let console_show = false;
    onMount(()=>{

        listen<{id:string, output:string}>('CONSOLE_OUT', (evt)=> {
            if (evt.payload.id === 'core') {
                console_buf = [...console_buf, evt.payload.output];
            }
        });
        appDir().then((dir)=>{app_dir=dir})
        // readDir('core', {dir: BaseDirectory.App})
    })

    async function startup() {
        await writeFile({path: 'core/config.temporary.toml', contents:dump(config)}, {dir: BaseDirectory.App});
        await emit('START_CORE', {
            config_path: `${app_dir}core/config.temporary.toml`
        });
    }

    async function load_default_config() {
        try {
            let config_toml = await readTextFile('core/config.default.toml', {dir: BaseDirectory.App});            
            let loaded_config = parse(config_toml);
            if(loaded_config&&config_toml) {
                config = loaded_config
            } 
        } catch (_) {
            await fix_core_config_dir();
            await load_default_config();
        }
    }

    async function fix_core_config_dir() {

        try {
            await readDir('core', {dir: BaseDirectory.App});
        } catch (error) {
            createDir('core', {dir: BaseDirectory.App})
        }

        writeFile({
            path:'core/config.default.toml',
            contents: DEFAULT_CORE_CONFIG
        }, {dir: BaseDirectory.App})
        

    }

    async function load_config() {
        let filename = await open({
            title: '打开配置文件',
            defaultPath: `${app_dir}core`,
            directory: false,
            multiple: false,
            filters: [
                {
                    extensions: ['toml'],
                    name: 'toml文件'
                }
            ]
        });
        if(typeof filename === 'string') {
            let config_toml = await readTextFile(filename);
            let loaded_config = parse(config_toml);
            if(loaded_config&&config_toml) {
                config = loaded_config
            } else {
                message("无法解析此文件捏")
            }
        }
    }
    async function save_config() {
        let path = await save({
            title: '保存配置文件',
            defaultPath: `${app_dir}core`,
            filters: [
                {
                    extensions: ['toml'],
                    name: 'toml文件'
                }

            ]
        });
        await writeFile({path, contents:dump(config)});
    }
</script>

{#await load_default_config()}
loading...
{:then _} 
<CoreConfigPage bind:config = {config}></CoreConfigPage>
{:catch e}
{e}
{/await}
<Button>
    <Label on:click={save_config}>保存</Label>
</Button>
<Button on:click = {startup}>
    <Label>启动</Label>
</Button>
<Button on:click={load_config}>
    <Label>载入</Label>
</Button>
{#if console_show}
    <div id = "console">
        <Console bind:lines = {console_buf} ></Console>
    </div>
{/if}

<DraggableFab bind:top={fab_top} bind:left={fab_left} onclick = {()=> {console_show = !console_show}}/>

<style>
    #console {
        height: 100%;
        width: 100%;
        position: absolute;
        top:0;
        left:0;
    }
</style>