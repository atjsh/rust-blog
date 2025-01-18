<script lang="ts">
	import PostForm from '../../../lib/post/post-form.svelte';
	import type { ActionData, PageData } from './$types';

	export let data: PageData;

	export let form: ActionData;
</script>

<div>
	<h1>New Post</h1>

	<PostForm availableCategories={data.categories} errorMessage={form?.error} />
</div>

<svelte:head>
	<title>New Post | Personal Blog from atjsh</title>
</svelte:head>

<style>
	div {
		padding: 2em;
		display: flex;
		flex-direction: column;
		gap: 2em;
	}
</style>
