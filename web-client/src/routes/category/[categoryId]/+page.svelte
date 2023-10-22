<script lang="ts">
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<h1>Category: {data.category.name}</h1>
<ul>
	{#each data.posts as post}
		<li>
			<a href={`/post/${post.id}`}>{post.title}</a>
		</li>
	{:else}
		<li>No posts</li>
	{/each}
</ul>
