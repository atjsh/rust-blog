<script lang="ts">
	import type { PageData } from './$types';

	export let data: PageData;
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
	</ul>
</div>

<div style="border: 1px solid gray; padding: 2em">
	{@html data.post.content}
</div>
