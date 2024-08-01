<script>
	import { PUBLIC_TMDB_API_KEY } from '$env/static/public';

	let search_contents = '';

    function on_search_type() {
        if (search_contents.length < 2) return;
        search();
    }

    function search() {
        const url = `https://api.themoviedb.org/3/search/movie?query=${search_contents}&include_adult=false&language=en-US&page=1`;

        const options = {
            method: 'GET',
            headers: {
                accept: 'application/json',
                Authorization:
                    `Bearer ${PUBLIC_TMDB_API_KEY}`
            }
        };

        fetch(url, options)
            .then((res) => res.json())
            .then((json) => {
                console.log(json)
            })
            .catch((err) => console.error('error:' + err));
    }
</script>

<input
	type="text"
	name="search_box"
	id="search_box"
	placeholder="Search Film"
    on:input={() => on_search_type()}
	bind:value={search_contents}
/>
<button on:click={() => search()}>Search</button>
