<script lang="ts">
	import type { PageServerData } from './$types';

	export let data: PageServerData;
</script>

<h1>Welcome to My Blog</h1>

<ul>
	{#each data.categories as category}
		<li>
			<a href={`/category/${category.id}`}>{category.name}</a>
			<ul>
				{#each category.posts as post}
					<li>
						<a href={`/post/${post.id}`}>{post.title}</a>
					</li>
				{/each}
			</ul>
		</li>
	{/each}
</ul>

<svelte:head>
	<title>blog.atj.sh</title>
</svelte:head>
