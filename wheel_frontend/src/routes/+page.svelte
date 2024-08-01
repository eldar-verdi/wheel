<script lang="ts">
	import { Canvas, Layer, t, type Render } from 'svelte-canvas';
	import { page } from '$app/stores';

	const items = $page.url.searchParams.get('items');

	let render: Render;
	let x: any;
</script>

<html lang="en">
	<h1>Yardley Movie Wheel</h1>
	<h2>By Harry and Eldar</h2>

</html>

<style>
	:root {
		font-family: sans-serif;
		text-align: center;
	}
</style>
