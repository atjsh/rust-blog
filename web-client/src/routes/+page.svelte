<script lang="ts">
	import type { PageServerData } from './$types';

	export let data: PageServerData;
</script>

<h1>Welcome to rust-blog</h1>

<ul>
	{#each data.categories as category}
		<li>
			<a href={`/category/${category.id}`}>{category.name}</a>
		</li>
	{/each}
</ul>
