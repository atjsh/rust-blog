<script lang="ts">
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<h1>Welcome to rust-blog</h1>

<ul>
	{#each data.categories as category}
		<li>
			<a href={`/category/${category.id}`}>{category.name}</a>
		</li>
	{/each}
</ul>
