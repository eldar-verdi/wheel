<script lang="ts">
	import { PUBLIC_TMDB_API_KEY } from '$env/static/public';

	let search_contents = '';

    type Film = {
        title: string,
        release_date: string,
        overview: string
    };

	let films: Film[] = [];

	function on_search_type() {
        if (search_contents === "") films = [];
		if (search_contents.length < 2) return;
		search();
	}

	function search() {
		const url = `https://api.themoviedb.org/3/search/movie?query=${search_contents}&include_adult=false&language=en-US&page=1`;

		const options = {
			method: 'GET',
			headers: {
				accept: 'application/json',
				Authorization: `Bearer ${PUBLIC_TMDB_API_KEY}`
			}
		};

		fetch(url, options)
			.then((res) => res.json())
			.then((json) => {
				console.log(json['results']);
				films = json['results'].slice(0, 6);
			})
			.catch((err) => console.error('error:' + err));
	}
</script>

<div class="container">
	<input
		type="text"
		name="search_box"
		id="search_box"
		placeholder="Search Film"
		bind:value={search_contents}
		on:input={() => on_search_type()}
	/>
	<button on:click={() => search()}>Search</button>
	{#each films as film}
		<div class="film_item">
			<span class="title">{film.title}</span>
			<span>({film.release_date.slice(0, 4)})</span>
			<br />
			<span class="overview">{film.overview}</span>
		</div>
	{/each}
</div>

<style>
	.container {
		margin: 0 auto;
		width: 600px;
		border: 1px solid red;
	}
	.film_item {
		border: 1px solid blue;
        text-align: left;
	}
    .film_item .overview {
        font-size: 0.8em;
    }
	.film_item .title {
		font-size: 1.1em;
		font-weight: bold;
	}
</style>
