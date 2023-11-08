<script lang="ts">
	import PostList from '../../../lib/post/post-list.svelte';
	import type { PageServerData } from './$types';

	export let data: PageServerData;
</script>

<h1>Category: {data.category.name}</h1>
<p>
	Posts from {data.category.name} category
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
