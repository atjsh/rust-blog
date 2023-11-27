<script lang="ts">
	import PostForm from '../../../lib/post/post-form.svelte';
	import type { ActionData, PageData } from './$types';

	export let data: PageData;

	export let form: ActionData;
</script>

<h1>New Post</h1>

<PostForm availableCategories={data.categories} errorMessage={form?.error} />

<svelte:head>
	<title>New Post | blog.atj.sh</title>
</svelte:head>
