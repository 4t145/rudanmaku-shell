<script lang="ts">
    import { appWindow, LogicalSize} from '@tauri-apps/api/window'
    import { emit} from '@tauri-apps/api/event'
    import { Icon } from '@smui/common';

    function unlock() {
      emit('WEBAPP_UNLOCK')
    }
</script>
<div class="titlebar">
    <div class="titlebar-button" id="titlebar-unlock" on:click={unlock}>
      <Icon class="material-icons">lock</Icon>
    </div>
</div>
<style>
    .titlebar {
        height: 30px;
        /* backgrounrgb(92, 45, 45)66); */
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

</style>