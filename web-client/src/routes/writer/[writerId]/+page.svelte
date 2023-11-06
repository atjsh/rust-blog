<script lang="ts">
	import type { PageServerData } from './$types';

	export let data: PageServerData;
</script>

<h1>Writer: {data.writer.email}</h1>

<p>self introduce: {data.writer.description}</p>

<h2>Posts</h2>

<ul>
	{#each data.posts as post}
		<li>
			<a href={`/post/${post.id}`}>{post.title}</a>
		</li>
	{/each}
</ul>

<svelte:head>
	<title>Writer {data.writer.email} | blog.atj.sh</title>
</svelte:head>
