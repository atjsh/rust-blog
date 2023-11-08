<script lang="ts">
	import type { PageServerData } from './$types';

	export let data: PageServerData;
</script>

<article>
	<div>
		<h1>{data.post.title}</h1>
		<div>
			{new Date(data.post.created_at).toLocaleDateString('ko-KR', { timeZone: 'UTC' })}
		</div>
	</div>
	<div>
		<ul>
			{#if data.isWriter}
				<li>
					<b>You</b> wrote this article. You can:
					<a href="/post/{data.post.id}/edit">edit</a>
					<a href="/post/{data.post.id}/delete">delete</a>
				</li>
			{/if}
		</ul>
	</div>

	<div class="post-container">
		<div class="post-content">
			{@html data.post.content}
		</div>
	</div>

	<div class="bottom-menu">
		<div>
			<a href="/category/{data.post.category.id}">
				See other posts from {data.post.category.name} category
			</a>
		</div>
		<div>
			<a href="/writer/{data.post.written_by.id}">See other posts from this post's writer</a>
		</div>
	</div>
</article>

<svelte:head>
	<title>{data.post.title} | blog.atj.sh</title>
</svelte:head>

<style lang="scss">
	h1 {
		word-break: keep-all;
	}

	.post-container {
		border: 1px solid rgb(198, 198, 198);
		border-radius: 0.8rem;
		padding: 2em 0.7em;

		@media screen and (max-width: 800px) {
			border: none;
			padding: 0 0.2em;
		}

		.post-content {
			word-break: keep-all;
			word-wrap: break-word;
			max-width: 60rem;
			margin: auto;
		}
	}

	.bottom-menu {
		margin: 2em 0;
		display: flex;
		gap: 1em;
		flex-direction: column;

		@media screen and (max-width: 800px) {
			border-top: 1px solid gray;
			padding-top: 1em;
		}
	}
</style>
