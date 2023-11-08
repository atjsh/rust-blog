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
				<div class="title">
					<span>
						{post.title}
					</span>
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

			font-size: 1.2em;

			width: 100%;

			a {
				color: black;
				text-decoration: none;
				width: 100%;
				display: flex;
				vertical-align: middle;

				&:hover {
					text-decoration: underline;
				}

				.thumbnail {
					border-radius: 1em;
					overflow: hidden;
					background: rgb(234, 234, 234);
					width: 5em;
					height: 5em;
					display: flex;
					padding: 0;
					margin: 0;
					flex-shrink: 0;

					span {
						margin: auto;
					}
				}

				.title {
					width: 100%;
					height: 5em;
					display: flex;
					word-break: keep-all;
					word-wrap: break-word;
					padding-right: 1em;
					box-sizing: border-box;

					span {
						margin: auto;
						margin-left: 1em;
					}
				}
			}
		}
	}
</style>
