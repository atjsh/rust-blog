<script lang="ts">
	import type { PageServerData } from './$types';

	export let data: PageServerData;
</script>

<h1>Profile</h1>

<p>email: {data.writer.email}</p>
<p>self introduce: {data.writer.description}</p>

<h2>Posts</h2>

<ul>
	{#each data.posts as post}
		<li>
			<a href={`/post/${post.id}`}>{post.title}</a>
		</li>
	{/each}
</ul>
