<script lang="ts">
	import { page } from '$app/stores';
	import { PUBLIC_TMDB_API_KEY } from '$env/static/public';

	const items = $page.url.searchParams.get('items');
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
