<script lang="ts">
	import type { PageData } from './$types';

	export let data: PageData;
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
