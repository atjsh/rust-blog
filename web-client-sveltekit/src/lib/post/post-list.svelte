<script lang="ts">
	export let categoryWithPosts: {
		posts: Array<{
			id: number;

			title: string;
			created_at: string;

			written_by: {
				id: number;
				email: string;
			};
		}>;
		id: number;
		name: string;
		created_at: string;
	};
</script>

<ul>
	{#each categoryWithPosts.posts as post}
		<li>
			<a href={`/post/${post.id}`}>
				<div class="thumbnail">
					<span>📝</span>
				</div>
				<div class="title-container">
					<div class="created-at">
						{new Date(post.created_at).toLocaleDateString('ko-KR', { timeZone: 'UTC' })}
					</div>
					<div class="title">
						{post.title}
					</div>
				</div>
			</a>
		</li>
	{/each}
</ul>

<style lang="scss">
	ul {
		list-style: none;
		padding: 0;
		margin: 0;

		display: flex;
		flex-direction: column;
		gap: 2em;
		width: 100%;

		li {
			width: 100%;

			width: 100%;

			a {
				color: rgb(var(--theme-text-color));
				text-decoration: none;
				width: 100%;
				display: flex;
				vertical-align: middle;
				border: 1px solid rgba(var(--theme-text-color), 0.3);
				border-radius: 1rem;
				padding: 0.2rem;

				&:hover {
					border: 1px solid rgba(128, 128, 128, 0.171);
				}

				.thumbnail {
					border-radius: 1rem;
					overflow: hidden;
					background: rgb(234, 234, 234);
					width: 5em;
					height: 5em;
					display: flex;
					padding: 0;
					margin: 0;
					flex-shrink: 0;
					font-size: 1.2em;

					@media (prefers-color-scheme: dark) {
						background: rgb(50, 50, 50);
					}

					span {
						margin: auto;
					}
				}

				.title-container {
					width: 100%;
					height: 5em;
					display: flex;
					word-break: keep-all;
					word-wrap: break-word;
					padding-right: 1em;
					box-sizing: border-box;
					flex-direction: column;
					justify-content: center;

					& > div {
						margin-left: 1rem;
					}

					.title {
						font-size: 1.2em;
					}

					.created-at {
						font-size: 0.7em;
						color: rgb(100, 100, 100);
						font-weight: bold;
					}
				}
			}
		}
	}
</style>
