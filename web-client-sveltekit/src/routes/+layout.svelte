<script lang="ts">
	import 'normalize.css';

	import type { LayoutServerData } from './$types';

	export let data: LayoutServerData;
</script>

<main>
	<header>
		<a href="/" class="title-a">
			<div class="title-content">
				<div class="icon">🫠</div>
				<code>blog.atj.sh</code>
			</div>
		</a>
	</header>
	<slot />
	<footer>
		<p>
			2023 <a href="https://atj.sh/">atjsh</a>. All rights reserved. <br /><br />
			이 블로그의 소스코드는
			<a href="https://github.com/atjsh/rust-blog" target="_blank">github.com/atjsh/rust-blog</a
			>에서 확인 가능합니다.
		</p>
		{#if data.authed}
			<a href="/profile">Profile</a> /
			<a href="/logout" data-sveltekit-preload-data="tap">Logout</a> /
			<a href="/post/new">New Post</a>
		{/if}
	</footer>
</main>

<style lang="scss">
	:root {
		--theme-bg-color: 255, 255, 255;
		--theme-text-color: 0, 0, 0;
		--theme-a-color: 0, 0, 255;
	}

	:global(body) {
		font-family: -apple-system, BlinkMacSystemFont, avenir next, avenir, segoe ui, helvetica neue,
			helvetica, Cantarell, Ubuntu, roboto, noto, arial, sans-serif;
		color: rgb(var(--theme-text-color));
		background-color: rgb(var(--theme-bg-color));

		@media (prefers-color-scheme: dark) {
			--theme-bg-color: 15, 15, 15;
			--theme-text-color: 255, 255, 255;
			--theme-a-color: 236, 159, 25;
		}
	}

	:global(a) {
		color: rgb(var(--theme-a-color));
	}

	main {
		max-width: 80rem;
		padding: 0 1rem;
		margin: 0 auto 2rem;
	}

	.icon {
		font-size: 3em;
		margin-top: 0.5em;
		margin-bottom: 0.1em;
	}

	.title-a {
		text-decoration: none;
		color: rgb(1, 149, 50);
		font-weight: bold;

		.title-content {
			display: inline-block;

			&:hover {
				text-decoration: underline;
			}
		}
	}

	footer,
	footer a {
		margin-top: 3em;
		text-align: center;
		color: gray;
	}
</style>
