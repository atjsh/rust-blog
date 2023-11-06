<script lang="ts">
	import type { PageServerData } from './$types';

	export let data: PageServerData;
</script>

<h1>{data.post.title}</h1>
<article>
	<div>
		<ul>
			<li>
				<a href="/writer/{data.post.written_by.id}"> writer: {data.post.written_by.email}</a>
			</li>
			<li>
				published at: {new Date(data.post.created_at).toLocaleString('ko-KR')}
			</li>
			{#if data.isWriter}
				<li>
					<b>You</b> wrote this article. You can:
					<a href="/post/{data.post.id}/edit">edit</a>
					<a href="/post/{data.post.id}/delete">delete</a>
				</li>
			{/if}
		</ul>
	</div>

	<div class="post-content">
		{@html data.post.content}
	</div>

	<br />

	<a href="/category/{data.post.category.id}">
		See other posts from {data.post.category.name} category
	</a>
</article>

<svelte:head>
	<title>{data.post.title} | blog.atj.sh</title>
</svelte:head>

<style>
	h1 {
		word-break: keep-all;
	}

	.post-content {
		border: 1px solid gray;
		padding: 2em;
		word-break: keep-all;
		word-wrap: break-word;
	}
</style>
