<script lang="ts">
	import { onMount } from 'svelte';
	import { Canvas, Layer, t, type Render } from 'svelte-canvas';
	import { page } from '$app/stores';

	const items = $page.url.searchParams.get('items');

	let render: Render;
	let x: any;

	let text_input: string = $page.url.searchParams.get('items') || '';
	let films: string[] = [];
	$: films = getTextString(text_input);

	// Parses a string of films separated by commas into an array of films
	function getTextString(text: string) {
		console.log(text_input);
		return text.split(',').map((film) => film.trim());
	}

	function onMagicDebugButtonPressed() {
		console.log(x.getCanvas());
		eval(atob('YWxlcnQoIndvdCB1IGxvb2tpbmcgYXQgdSBtZWxvbiIp'));
	}

	$: render = ({ context, width, height }) => {
		/**
		 * Time to work this shit out
		 * what do we need everytime we are entering a film?
		 * we ideally wouldnt change every single colour in the circle. would be better to just change the newest one
		 * but to do that then we wouldnt want to be looping through the number of films 
		 * or we would need some sort of way of identifying if we have already added a section for that film/state
		 * i dont wanna get too deep into state rn so fuck that 
		 * 
		 * ok other things
		 * we need the text on each segment of the circle - lets work on this one first
		 * we want the circle to actually rotate
		 * we want an arrow that will select a segment of the circle once it has stopped spinning 
		 * 
		 */
		let off = 10,
			horizontalCoord = (width - off) / 2,
			verticalCoord = (height - off) / 2,
			pieAngle = (2 * Math.PI) / films.length,
			segmentDepth = 150;

		function createWheelOfDoom(radius: number) {
			for (let i = 0; i < films.length; i++) {
				context.beginPath();
				context.moveTo(horizontalCoord, verticalCoord);
				context.arc(horizontalCoord, verticalCoord, radius, i * pieAngle, (i + 1) * pieAngle);
				context.lineWidth = 2;
				// context.fillText(films[i], 2, 2);
				context.fillText("eldar", 2,2);
				
				// context.fillStyle = '#' + ((Math.random() * 0xffffff) << 0).toString(16);
				context.fill();
				context.closePath();
			}
		}

		createWheelOfDoom(segmentDepth);
	};
</script>

<html lang="en">
	<h1>WHEEL OF PAIN</h1>
	<h2>By Harry and Eldar</h2>

	<button on:click={onMagicDebugButtonPressed}>👹 Mystical Debug Button 👹</button>

	<br />

	<img
		width="200"
		src="https://ih1.redbubble.net/image.3623098288.8604/raf,360x360,075,t,fafafa:ca443f4786.jpg"
		alt="WHEEL OF PAIN"
	/>

	<br />

	<input bind:value={text_input} type="text" placeholder="film1, film2, film3" />
	<Canvas bind:this={x} width={512} height={512}>
		<Layer {render} />
	</Canvas>
</html>

<style>
	:root {
		font-family: sans-serif;
		text-align: center;
	}
</style>
