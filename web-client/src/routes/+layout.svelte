<script lang="ts">
	import type { LayoutServerData } from './$types';
	import { dev } from '$app/environment';
	import { inject } from '@vercel/analytics';
	import { navigating } from '$app/stores';

	inject({ mode: dev ? 'development' : 'production' });

	export let data: LayoutServerData;
</script>

<main>
	<header>
		{#if $navigating}
			<div class="loading">
				<div class="spin">⏳</div>
				<div>loading...</div>
			</div>
		{/if}
		<a href="/">This is rust-blog</a><br />
		{#if data.authed}
			<a href="/profile">Profile</a> /
			<a href="/logout" data-sveltekit-preload-data="tap">Logout</a> /
			<a href="/post/new">New Post</a>
		{:else}
			<a href="/login">Login</a>
		{/if}
	</header>
	<slot />
	<footer>2023 ATJSH. All rights reserved</footer>
</main>

<style>
	.loading {
		position: fixed;
		top: 0;
		left: 0;
		width: 100%;
		background: rgba(0, 0, 0, 0.8);
		color: white;
		padding: 1em;
		display: flex;
		gap: 0.5em;
	}

	.spin {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		0% {
			transform: rotate(0deg);
		}

		100% {
			transform: rotate(360deg);
		}
	}
</style>
