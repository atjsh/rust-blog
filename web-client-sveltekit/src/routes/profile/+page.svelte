<script lang="ts">
	import WriterPosts from '$lib/writer/writer-posts.svelte';
	import WriterProfile from '$lib/writer/writer-profile.svelte';
	import type { PageServerData } from './$types';

	export let data: PageServerData;
</script>

<h1>Profile</h1>

<WriterProfile writer={data.writer} />

<h2>Posts</h2>

<WriterPosts posts={data.posts} />

<svelte:head>
	<title>Writer {data.writer.email} | Personal Blog from atjsh</title>
</svelte:head>
