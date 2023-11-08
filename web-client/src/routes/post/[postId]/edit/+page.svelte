<script lang="ts">
	import PostForm from '../../../../lib/post/post-form.svelte';
	import type { ActionData, PageData } from './$types';

	export let data: PageData;

	export let form: ActionData;
</script>

<h1>Edit Post '{data.post.title}'</h1>

<PostForm
	availableCategories={data.categories}
	defaultPostValues={{
		categoryId: data.post.category.id,
		title: data.post.title,
		content: data.post.content
	}}
	errorMessage={form?.error}
/>

<svelte:head>
	<title>edit {data.post.title} | blog.atj.sh</title>
</svelte:head>
