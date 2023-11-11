<script lang="ts">
	import PostList from '../../../lib/post/post-list.svelte';
	import type { PageServerData } from './$types';

	export let data: PageServerData;
</script>

<h1>{data.category.name} 카테고리</h1>
<p>
	{data.category.name} 카테고리 글 모음.
</p>

<PostList
	categoryWithPosts={{
		...data.category,
		posts: data.posts
	}}
/>

<svelte:head>
	<title>Category {data.category.name} | blog.atj.sh</title>
</svelte:head>
