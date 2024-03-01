<script lang="ts">
	import { getContentTypeLabel } from '../../../lib';
	import { PUBLIC_WEB_URL } from '$env/static/public';

	import type { PageServerData } from './$types';

	export let data: PageServerData;
	export let webUrl = PUBLIC_WEB_URL;
</script>

<article>
	<div class="post-metadata">
		<h1>{data.post.title}</h1>
		<div class="category-and-date">
			<div>
				{data.post.category.name}
			</div>
			<div>
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
		<div class="post-content">
			{@html data.post.renderedContent}
		</div>
	</div>
</article>

<svelte:head>
	<title>{data.post.title} | blog.atj.sh</title>
</svelte:head>

<style lang="scss">
	article {
		padding: 2em 3em;
		max-width: 42rem;
		display: flex;
		flex-direction: column;
		gap: 2em;

		@media screen and (max-width: 800px) {
			border: none;
			padding: 1em 1em;
		}

		.post-metadata {
			display: flex;
			flex-direction: column;
			gap: 1em;

			h1 {
				word-break: keep-all;
				word-wrap: break-word;
				color: #2c5e96;
				font-size: 2.4em;
			}

			.category-and-date {
				display: flex;
				gap: 0.3em;
				color: #2c5e96;
				font-size: 1.3em;
			}

			.links {
				margin-top: 0.4em;

				.links-title {
					font-weight: bold;
					cursor: pointer;
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
			}
		}
	}

	:global(
			.post-content h1,
			.post-content h2,
			.post-content h3,
			.post-content h4,
			.post-content h5,
			.post-content h6
		) {
		color: #005a9c;
		font-weight: 600;
	}

	:global(.post-content h1) {
		display: block;
		font-size: 2em;
		margin-top: 0.67em;
		margin-bottom: 0.67em;
		margin-left: 0;
		margin-right: 0;
	}

	:global(.post-content h2) {
		display: block;
		font-size: 1.5em;
		margin-top: 0.83em;
		margin-bottom: 0.83em;
		margin-left: 0;
		margin-right: 0;
	}

	:global(.post-content h3) {
		display: block;
		font-size: 1.17em;
		margin-top: 1em;
		margin-bottom: 1em;
		margin-left: 0;
		margin-right: 0;
	}

	:global(.post-content h4) {
		display: block;
		font-size: 1em;
		margin-top: 1.33em;
		margin-bottom: 1.33em;
		margin-left: 0;
		margin-right: 0;
	}

	:global(.post-content h5) {
		display: block;
		font-size: 0.83em;
		margin-top: 1.67em;
		margin-bottom: 1.67em;
		margin-left: 0;
		margin-right: 0;
	}

	:global(.post-content h6) {
		display: block;
		font-size: 0.67em;
		margin-top: 2.33em;
		margin-bottom: 2.33em;
		margin-left: 0;
		margin-right: 0;
	}

	:global(.post-content p) {
		margin-top: 0;
		margin-bottom: 1em;
		line-height: 1.6;
	}

	:global(li) {
		list-style-position: inside;
		margin-left: 1em;
	}
</style>
