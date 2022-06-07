<script lang="ts">
    import Textfield from '@smui/textfield';
    import HelperText from '@smui/textfield/helper-text';
    import Checkbox from '@smui/checkbox';
    import FormField from '@smui/form-field';
    import Chip, { Set, LeadingIcon, TrailingIcon, TrailingAction, Text as ChipText} from '@smui/chips';
    import List, {Item, Text as ListText, Separator, Graphic} from '@smui/list';
    import IconButton from '@smui/icon-button';
    import {type CoreConfig, ip_valid, dump, parse} from '../model/core-config';
    export let config:CoreConfig;
    let ipv4_raw = '127.0.0.1';
    function ip_convert(raw:string):number[] {
        return raw.split('.').map((s)=>Number.parseInt(s, 10))
    }

</script>

<div id="config">
<h2>网络</h2>
<Textfield label="端口" bind:value={config.net.port} type='number' invalid={(!Number.isInteger(config.net.port))||config.net.port>65535||config.net.port<0}>
    <HelperText slot="helper">网络服务端口</HelperText>
</Textfield>
<Textfield label="IP" bind:value={ipv4_raw} invalid = {!ip_valid(ipv4_raw)} on:change={()=>{config.net.ipv4 = ip_convert(ipv4_raw)}}>
    <HelperText slot="helper">网络服务IP</HelperText>
</Textfield>
<h2>房间</h2>
<List>
    {#each config.room as room, index}        
    <Item>
        <Textfield label="房间号" type="number" bind:value={room.roomid} invalid = {!Number.isInteger(room.roomid)||room.roomid<0}>
        </Textfield>
        <Set chips={['json', 'bincode']} let:chip filter bind:selected={room.channel}>
            <Chip {chip} touch>
                <ChipText>{chip}</ChipText>
            </Chip>
        </Set>
        <IconButton class="material-icons" on:click={() => {
            config.room = [...config.room.slice(0, index), ...config.room.slice(index+1)]
        }} style="margin-left: auto;">close</IconButton>
    </Item>
    {/each}
    <Separator />
    <Item on:click = {()=>{config.room = [...config.room, {roomid:0, channel: ['json']}]}}>
        <Graphic class="material-icons">add</Graphic>
        <ListText>添加</ListText>
    </Item>
</List>
</div>
<style>
    #config{
        text-align: left;
    }
</style>