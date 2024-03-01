<script lang="ts">
	import { PUBLIC_WEB_URL } from '$env/static/public';
	import { getContentTypeLabel } from '../../../../lib';

	import type { PageServerData } from './$types';

	export let data: PageServerData;
	export let webUrl = PUBLIC_WEB_URL;
</script>

<article>
	<div class="post-metadata">
		<h1>{data.post.title} (Raw {getContentTypeLabel(data.post.content_type)})</h1>
		<div class="category-and-date">
			<div class="category">
				{data.post.category.name}
			</div>
			<div class="date">
				{new Date(data.post.created_at).toLocaleDateString('ko-KR', { timeZone: 'UTC' })}
			</div>
		</div>
		<details class="links" open>
			<summary class="links-title">이 문서에 대한 자세한 정보</summary>
			<ul>
				<li>
					<div class="links-subtitle">문서의 판:</div>
					<a class="link" href={`${webUrl}/post/${data.post.id}`}
						>{`${webUrl}/post/${data.post.id}`}</a
					>
				</li>

				<li>
					<div class="links-subtitle">문서의 소스코드:</div>
					<a class="link" href={`${webUrl}/post/${data.post.id}/raw`}
						>{`${webUrl}/post/${data.post.id}/raw`}</a
					>
				</li>

				<li>
					<div class="links-subtitle">렌더링된 버전:</div>
					<a class="link" href={`${webUrl}/post/${data.post.id}`}
						>{`${webUrl}/post/${data.post.id}`}</a
					>
				</li>

				<li>
					<div class="links-subtitle">리포지토리:</div>
					<div class="content">제공되지 않음</div>
				</li>

				<li>
					<div class="links-subtitle">문서의 형식:</div>
					<div class="content">
						{getContentTypeLabel(data.post.content_type)}
					</div>
				</li>

				<li>
					<div class="links-subtitle">작성자:</div>
					<a class="link" href="https://atj.sh">전성훈</a>
				</li>
			</ul>
		</details>
		<div class="copyright">Copyright © 2024 atjsh.</div>
		{#if data.isWriter}
			<div>
				<ul>
					<li>
						<b>당신</b>이 이 게시글을 작성했습니다. 가능한 동작:
						<a href="/post/{data.post.id}/edit">수정</a>
						<a href="/post/{data.post.id}/delete">삭제</a>
					</li>
					{#if data.post.private}
						<li>이 게시글은 비공개 상태입니다.</li>
					{/if}
				</ul>
			</div>
		{/if}
	</div>

	<hr />

	<div class="post-container">
		<pre class="post-content">{data.post.content}</pre>
	</div>
</article>

<svelte:head>
	<title>{data.post.title} (raw) | blog.atj.sh</title>
</svelte:head>

<style lang="scss">
	article {
		padding: 2em 2em 6em 2em;
		max-width: 42rem;
		display: flex;
		flex-direction: column;
		gap: 2em;

		@media screen and (max-width: 800px) {
			border: none;
			padding: 1em 1em 7em 1em;
		}

		.post-metadata {
			display: flex;
			flex-direction: column;
			gap: 1em;

			h1 {
				word-break: break-all;
				font-family: monospace;
				word-wrap: break-word;
				color: #2c5e96;
				font-size: 2.4em;
			}

			.category-and-date {
				color: #2c5e96;
				font-size: 1.3em;

				.category {
					display: inline-block;

					&::after {
						content: ',';
					}
				}

				.date {
					display: inline-block;
				}
			}

			.links {
				margin-top: 0.4em;

				.links-title {
					font-weight: bold;
					cursor: pointer;

					&:hover {
						text-decoration: underline;
					}

					@media screen and (max-width: 800px) {
						text-decoration: underline;
					}
				}

				ul {
					list-style: none;
					padding: 0;
					margin: 0;

					li {
						margin-top: 1em;

						.links-subtitle {
							font-weight: bold;
						}

						.link {
							color: #2c5e96;
						}

						.link,
						.content {
							padding-left: 3em;
						}
					}
				}
			}
		}

		.post-container {
			.post-content {
				word-break: keep-all;
				word-wrap: break-word;
				margin: auto;
				overflow-x: auto;
			}
		}
	}
</style>
