<script lang="ts">
	import PostList from '../lib/post/post-list.svelte';
	import type { PageServerData } from './$types';

	export let data: PageServerData;
</script>

<h1>전성훈의 블로그</h1>

<ul>
	{#each data.categories as category}
		<li>
			<h2>
				<a href={`/category/${category.id}`}>{category.name}</a> 카테고리의 글 목록
			</h2>
			<PostList categoryWithPosts={{ ...category, posts: category.posts }} />
		</li>
	{/each}
</ul>

<svelte:head>
	<title>전성훈의 블로그 | blog.atj.sh</title>
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
