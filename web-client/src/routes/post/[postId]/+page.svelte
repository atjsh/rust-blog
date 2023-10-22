<script lang="ts">
	import type { PageServerData } from './$types';

	export let data: PageServerData;
</script>

<hr />

<a href="/category/{data.post.category.id}">
	back to category: {data.post.category.name}
</a>

<h1>Title: {data.post.title}</h1>
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

<div style="border: 1px solid gray; padding: 2em">
	{@html data.post.content}
</div>
