<script lang="ts">
    import { parse } from 'toml';
    import {readTextFile, readBinaryFile, readDir} from '@tauri-apps/api/fs';
    import type { WebappPkg} from '../model/webapp-pkg';
    import List, { Item, Text, SecondaryText } from '@smui/list';
    import Checkbox from '@smui/checkbox';
    import FormField from '@smui/form-field';
    import Textfield from '@smui/textfield';
    import HelperText from '@smui/textfield/helper-text';
    import Card, {    
        Content,
    } from '@smui/card'
    import IconButton from '@smui/icon-button';
    import { emit } from '@tauri-apps/api/event'
    import { Icon, Label } from '@smui/common';
    export let path:string;
    import { writeText } from '@tauri-apps/api/clipboard';
    import { invoke } from '@tauri-apps/api/tauri';

    enum OpenContent {
        None,
        Info,
        Settings
    }
    let open_content:OpenContent = OpenContent.None;
    let search: {
        string: {[key: string]:string},
        number: {[key: string]:number},
        boolean: {[key: string]:boolean}
    } = {
        string: {},
        number: {},
        boolean: {}
    };
    async function read_package_info(path:string):Promise<{icon:Blob, pkginfo: WebappPkg, port:number}> {
        let entries = await readDir(path);
        let icon;
        let pkginfo:WebappPkg;
        for(const file of entries) {
            if (file.name==='favicon.ico') {
                icon = new Blob([await readBinaryFile(file.path)]);
            } else if (file.name==='package.toml') {
                pkginfo = parse(await readTextFile(file.path));
            } else {
                continue;
            }
        }
        if(pkginfo.args.boolean) {
            for(const arg of pkginfo.args.boolean) {
                search.boolean[arg.key] = arg.default
            }
        } else {
            pkginfo.args.boolean = [];
        }

        if(pkginfo.args.string) {
            for(const arg of pkginfo.args.string) {
                search.string[arg.key] = arg.default
            }
        } else {
            pkginfo.args.string = [];
        }

        if(pkginfo.args.number) {
            for(const arg of pkginfo.args.number) {
                search.number[arg.key] = arg.default
            }
        } else {
            pkginfo.args.number = [];
        }

        let port = await invoke('get_sws_port') as number;
        return ({
            port,
            icon,
            pkginfo
        })
    }

    async function generate_url() {
        let query = [];
        for(const key of Object.keys(search.number)) {
            query.push(`${key}=${search.number[key]}`)
        }
        for(const key of Object.keys(search.string)) {
            query.push(`${key}=${search.string[key]}`)
        }
        for(const key of Object.keys(search.boolean)) {
            query.push(`${key}=${search.boolean[key]}`)
        }
        query.join()
    }
</script>

<Card>
    <Content>
    <div>
        {#await read_package_info(path)}
        loading...
        {:then {icon, pkginfo, port}}
        <div class="webapp-item">
            <div id="icon">
                <img src={URL.createObjectURL(icon)} alt="" height="48px">
            </div>
            <div id="app-name">
                <h3>{pkginfo.package.appname}</h3>
            </div>
            <div id="app-brief">
                {pkginfo.package.brief}
            </div>
            <div id="buttons">
                <IconButton on:click={()=>{
                    console.log(open_content);
                    if(open_content===OpenContent.Info) {
                        open_content = OpenContent.None;
                    } else {
                        open_content = OpenContent.Info;
                    }
                }}><Icon class="material-icons" >info</Icon></IconButton>
                <IconButton on:click={()=>{
                    if(open_content===OpenContent.Settings) {
                        open_content = OpenContent.None;
                    } else {
                        open_content = OpenContent.Settings;
                    }
                }}><Icon class="material-icons" >settings</Icon></IconButton>
                <IconButton on:click={()=>{
                    let query = [];
                    for(const key of Object.keys(search.number)) {
                        query.push(`${key}=${search.number[key]}`)
                    }
                    for(const key of Object.keys(search.string)) {
                        query.push(`${key}=${search.string[key]}`)
                    }
                    for(const key of Object.keys(search.boolean)) {
                        query.push(`${key}=${search.boolean[key]}`)
                    }
                    const search_part = query.join('&');
                    const url = `http://localhost:${port}/${pkginfo.package.name}/${pkginfo.package.entry}?${search_part}`
                    writeText(url)
                }}><Icon class="material-icons">content_copy</Icon></IconButton>
                <IconButton on:click={()=>{
                    let query = [];
                    for(const key of Object.keys(search.number)) {
                        query.push(`${key}=${search.number[key]}`)
                    }
                    for(const key of Object.keys(search.string)) {
                        query.push(`${key}=${search.string[key]}`)
                    }
                    for(const key of Object.keys(search.boolean)) {
                        query.push(`${key}=${search.boolean[key]}`)
                    }
                    const search_part = query.join('&');
                    const url = `http://localhost:${port}/${pkginfo.package.name}/${pkginfo.package.entry}?${search_part}`
                    emit('OPEN_WEBAPP', {
                        url: encodeURI(url),
                        name: pkginfo.package.name
                    });
                }}><Icon class="material-icons">launch</Icon></IconButton>
            </div>
        </div>
        <div class="optional-content">
            {#if open_content === OpenContent.Info}
            <div id="info">
                <List nonInteractive>
                    <Item>
                        <Text>
                            <SecondaryText>app名称：</SecondaryText>
                            {pkginfo.package.appname}
                        </Text>
                    </Item>
                    <Item>
                        <Text>
                            <SecondaryText>签名：</SecondaryText>
                            {pkginfo.package.name}
                        </Text>
                    </Item>
                    <Item>
                        <Text>
                            <SecondaryText>作者：</SecondaryText>
                            {pkginfo.package.authors.join('; ')}
                        </Text>
                    </Item>
                    <Item>
                        <Text>
                            <SecondaryText>版本：</SecondaryText>
                            {pkginfo.package.version}
                        </Text>
                    </Item>
                    <Item>
                        <Text>
                            <SecondaryText>简介：</SecondaryText>
                            {pkginfo.package.brief}
                        </Text>
                    </Item>
                    <Item>
                        <Text>
                            <SecondaryText>联系方式: </SecondaryText>
                            {pkginfo.package.contact.join('; ')}
                        </Text>
                    </Item>
                    <Item>
                        <Text>
                            <SecondaryText>入口: </SecondaryText>
                            <code>{pkginfo.package.entry}</code>
                        </Text>
                    </Item>
                    <Item>
                        <Text>
                            <SecondaryText>仓库: </SecondaryText>
                            {#if pkginfo.package.repo}
                            <a href={pkginfo.package.repo}>{pkginfo.package.repo}</a>
                            {:else}
                            无
                            {/if}
                        </Text>
                    </Item>
                </List>
            </div>
            {:else if open_content===OpenContent.Settings}
            <div id="settings" >
                <List nonInteractive>
                    {#each pkginfo.args.number as arg}
                        <Item>
                            <Textfield label={arg.key} type="number" bind:value={search.number[arg.key]}>
                                <HelperText slot="helper">{arg.description}</HelperText>
                            </Textfield>
                        </Item>
                    {/each}
                    {#each pkginfo.args.string as arg}
                    <Item>
                        <Textfield label={arg.key} type="text" bind:value={search.string[arg.key]}>
                            <HelperText slot="helper">{arg.description}</HelperText>
                        </Textfield>
                    </Item>
                    {/each}
                    {#each pkginfo.args.boolean as arg}
                    <Item>
                        <FormField align="end" >
                            <Checkbox bind:checked={search.boolean[arg.key]}></Checkbox>
                            <span slot="label">{arg.description}</span>
                        </FormField>
                    </Item>
                    {/each}
                </List>
            </div>
            {:else}
            <div></div>
            {/if}
        </div>
        {:catch}
        <Icon class="material-icons">error</Icon>
        {/await}
    </div>
</Content>
</Card>

<style>
    .webapp-item {
        display: flex;
        padding: 4px;
    }
    .webapp-item>div {
        margin-left: 8px;
        margin-right: 8px;
    }
    #icon {
        align-self: center;
        width: 48px;
        height: 48px;
        border: none;
        border-radius: 4px 4px;
        box-shadow: gray 1px 1px 3px;
    }
    #buttons {
        align-self: center;
        margin-left: auto;
    }
    #app-brief{
        align-self:center;
    }
    .optional-content>#info {
        text-align: left;
    }
</style>