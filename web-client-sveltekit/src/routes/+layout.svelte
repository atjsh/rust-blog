<script lang="ts">
	import { navigating, page } from '$app/stores';
	import type { LayoutServerData } from './$types';

	export let isAsideShown = false;
	export let isFloatingButtonShown = true;

	$: if ($navigating) {
		if ($page.url.pathname === '/') {
			isAsideShown = true;
			isFloatingButtonShown = false;
		} else {
			isAsideShown = false;
			isFloatingButtonShown = true;
		}
	}

	if ($page.url.pathname === '/') {
		isFloatingButtonShown = false;
		isAsideShown = true;
	}

	export let data: LayoutServerData;
</script>

<div class={`content ${isAsideShown ? 'aside-show' : ''}`}>
	<aside>
		<div class="ribbon">Personal Blog from atjsh (전성훈)</div>
		<nav>
			<div class="links-container">
				<ul>
					<li>
						<a href="https://atj.sh">atjsh (전성훈)</a>
					</li>
					<li>
						<a href="https://resume.atj.sh/resume.kor.pdf">이력서 PDF</a>
					</li>
				</ul>
			</div>
			{#each data.categories as category}
				<div class="links-container">
					<div class="description">{category.name}:</div>
					<ul>
						{#each category.posts as post}
							<li><a href={`/post/${post.id}`}>{post.title}</a></li>
						{/each}
					</ul>
				</div>
			{/each}
			{#if data.authed}
				<div class="links-container">
					<div class="description">PB Administration:</div>
					<ul>
						<li><a href="/profile">Profile</a></li>
						<li><a href="/logout" data-sveltekit-preload-data="tap">Logout</a></li>
						<li><a href="/post/new">New Post</a></li>
					</ul>
				</div>
			{:else}
				<div class="links-container">
					<div class="description">PB Administration:</div>
					<ul>
						<li><a href="/login">WebMaster Login</a></li>
					</ul>
				</div>
			{/if}
		</nav>
	</aside>
	<main>
		<slot />
	</main>
</div>
{#if isFloatingButtonShown}
	<div class="floating-button">
		<button on:click={() => (isAsideShown = !isAsideShown)}>
			웹사이트 목차 {isAsideShown ? '가리기' : '보기'}
		</button>
	</div>
{/if}

<style lang="scss">
	:root {
		--theme-bg-color: 255, 255, 255;
		--theme-text-color: 0, 0, 0;
		color-scheme: light dark;

		@media (prefers-color-scheme: dark) {
			--theme-bg-color: 15, 15, 15;
			--theme-text-color: 255, 255, 255;
		}

		@media screen and (max-width: 800px) and (prefers-color-scheme: dark) {
			--theme-bg-color: 0, 0, 0;
		}
	}

	:global(body) {
		font-family:
			-apple-system,
			BlinkMacSystemFont,
			avenir next,
			avenir,
			segoe ui,
			helvetica neue,
			helvetica,
			Cantarell,
			Ubuntu,
			roboto,
			noto,
			arial,
			sans-serif;

		background-color: rgb(var(--theme-bg-color));
		color: rgb(var(--theme-text-color));
	}

	:global(*) {
		margin: 0;
		padding: 0;
	}

	.floating-button {
		display: none;
		position: fixed;
		bottom: 2rem;
		right: 2rem;
		z-index: 1000;

		button {
			padding: 1em;
			border: none;
			border-radius: 0.5em;
			background-color: #2c5e96;
			color: white;
			cursor: pointer;
		}

		@media screen and (max-width: 800px) {
			display: block;
		}
	}

	aside {
		position: fixed;
		background-color: #d9d9d9;
		background-image: url('data:image/svg+xml;utf8,<svg xmlns="http://www.w3.org/2000/svg" width="300" height="300" viewBox="0 0 300 300"><text x="150" y="210" text-anchor="middle" transform="rotate(45 150 150)" font-size="180" font-family="sans-serif" fill="black" fill-opacity="0.03">PB</text></svg>');
		background-repeat: repeat;
		width: 30rem;
		height: 100vh;
		height: 100dvh;
		box-sizing: border-box;
		overflow-y: auto;

		@media (prefers-color-scheme: dark) {
			background-color: #000000;
			background-image: url('data:image/svg+xml;utf8,<svg xmlns="http://www.w3.org/2000/svg" width="300" height="300" viewBox="0 0 300 300"><text x="150" y="210" text-anchor="middle" transform="rotate(45 150 150)" font-size="180" font-family="sans-serif" fill="white" fill-opacity="0.08">PB</text></svg>');
		}

		.ribbon {
			width: 30rem;
			height: 2rem;
			background-color: #2c5e96;
			padding: 0.7em 2rem 0.7em 10em;
			box-sizing: border-box;
			transform-origin: 15rem 15rem;
			transform: rotate(-90deg);
			text-align: right;
			color: white;
			font-size: 0.8em;
			cursor: default;
			position: fixed;
		}

		nav {
			margin-top: 0rem;
			padding: 2em;
			padding-left: calc(2em + 2rem);
			width: 100%;
			box-sizing: border-box;

			.links-container {
				margin-bottom: 2em;

				.description {
					font-weight: bold;
					margin-bottom: 0.5em;
					text-transform: uppercase;
					opacity: 0.5;
					cursor: default;
				}

				ul {
					li {
						list-style-position: inside;
						margin-left: 1em;
						margin-bottom: 0.3em;
					}
				}
			}
		}
	}

	main {
		padding-left: 30rem;
		box-sizing: border-box;
		flex: 1;
		width: 100%;
	}

	.content {
		display: flex;

		@media screen and (max-width: 800px) {
			aside {
				display: none;
				width: 100%;
			}

			main {
				display: block;
				padding-left: 0px;
			}

			&.aside-show {
				aside {
					display: block;
				}

				main {
					display: none;
				}
			}
		}
	}
</style>
