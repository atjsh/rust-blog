<script lang="ts">
	import PostList from '../lib/post/post-list.svelte';
	import type { PageServerData } from './$types';

	export let data: PageServerData;
</script>

<h1>Welcome to My Blog</h1>

<ul>
	{#each data.categories as category}
		<li>
			<h2>
				Posts from <a href={`/category/${category.id}`}>{category.name}</a> category
			</h2>
			<PostList categoryWithPosts={{ ...category, posts: category.posts }} />
		</li>
	{/each}
</ul>

<svelte:head>
	<title>blog.atj.sh</title>
</svelte:head>

<style lang="scss">
	ul {
		list-style: none;
		padding: 0;
		margin: 0;

		h2 {
			color: gray;

			a {
				color: inherit;
			}
		}
	}
</style>
