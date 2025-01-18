<script lang="ts">
	import PostForm from '$lib/post/post-form.svelte';
	import type { ActionData, PageData } from './$types';

	export let data: PageData;

	export let form: ActionData;
</script>

<div>
	<h1>Edit Post '{data.post.title}'</h1>

	<PostForm
		availableCategories={data.categories}
		defaultPostValues={{
			categoryId: data.post.category.id,
			title: data.post.title,
			private: data.post.private,
			content: data.post.content,
			contentType: data.post.content_type
		}}
		errorMessage={form?.error}
	/>
</div>

<svelte:head>
	<title>edit {data.post.title}| Personal Blog from atjsh</title>
</svelte:head>

<style>
	div {
		padding: 2em;
		display: flex;
		flex-direction: column;
		gap: 2em;
	}
</style>
