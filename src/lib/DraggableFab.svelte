<script lang="ts">
    import Fab, { Icon as FabIcon}  from '@smui/fab';
    export let left;
	export let top;
    export let onclick:(MouseEvent) => void;
    let moving = false;
	
	function onMouseDown() {
		moving = true;
	}
	
	function onMouseMove(e) {
		if (moving) {
			left += e.movementX;
			top += e.movementY;
		}
	}
	
	function onMouseUp() {
		moving = false;
	}
</script>

<div class="draggable-fab" on:mousedown={onMouseDown} style="left: {left}px; top: {top}px;">
    <Fab on:click={onclick}>
        <FabIcon class="material-icons">terminal</FabIcon>
    </Fab>
</div>

<style>
	.draggable-fab {
		user-select: none;
        z-index: 999;
		position: absolute;
	}
</style>



<svelte:window on:mouseup={onMouseUp} on:mousemove={onMouseMove} />