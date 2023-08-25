<script lang="ts">
	import { onMount } from 'svelte';

	let canvas: HTMLCanvasElement;

	function onMagicDebugButtonPressed() {
		eval(atob('YWxlcnQoIndvdCB1IGxvb2tpbmcgYXQgdSBtZWxvbiIp'));
	}

	onMount(() => {
		const ctx = canvas.getContext('2d')!;
		let path2 = new Path2D();
		path2.rect(10, 10, 100, 100);
		ctx.stroke(path2);
	});
</script>

<html lang="en">
	<button on:click={onMagicDebugButtonPressed}>👹 Mystical Debug Button 👹</button>
	<canvas bind:this={canvas} width={512} height={512} />
</html>
