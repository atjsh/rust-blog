<script lang="ts">
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	let latestPostsFromEachCategory = data.categories
		.map((category) => {
			const orderedPosts = category.posts.sort((a, b) => {
				const aDate = new Date(a.created_at);
				const bDate = new Date(b.created_at);
				return bDate.getTime() - aDate.getTime();
			});

			return orderedPosts[0];
		})
		.sort((a, b) => {
			const aDate = new Date(a.created_at);
			const bDate = new Date(b.created_at);
			return bDate.getTime() - aDate.getTime();
		});
</script>

<div class="container">
	<div class="texts">
		<p>
			<b>최근에 작성된 글</b>
		</p>
		<br />
		<p>
			{#each latestPostsFromEachCategory as post}
				<a href={`/post/${post.id}`}
					>{post.title} ({new Date(post.created_at).toLocaleDateString('ko-KR', {
						timeZone: 'UTC'
					})})</a
				><br />
			{/each}
		</p>
	</div>
</div>

<svelte:head>
	<title>Personal Blog from atjsh</title>
</svelte:head>

<style>
	.container {
		background-color: #b6b6b6;
		background-image: url('data:image/svg+xml;utf8,<svg xmlns="http://www.w3.org/2000/svg" width="300" height="300" viewBox="0 0 300 300"><text x="150" y="210" text-anchor="middle" transform="rotate(45 150 150)" font-size="180" font-family="sans-serif" fill="black" fill-opacity="0.03">PB</text></svg>');
		background-repeat: repeat;
		background-size: 4em;
		height: 100vh;
		width: 100%;

		display: flex;

		@media (prefers-color-scheme: dark) {
			background-color: #0a0a0a;
		}

		.texts {
			background-color: #b6b6b6;
			padding: 3em;
			margin: auto;
			line-height: 130%;

			@media (prefers-color-scheme: dark) {
				background-color: #0a0a0a;
			}
		}
	}
</style>
