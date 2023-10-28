<script lang="ts">
	import type { LayoutServerData } from './$types';
	import { dev } from '$app/environment';
	import { inject } from '@vercel/analytics';

	inject({ mode: dev ? 'development' : 'production' });

	export let data: LayoutServerData;
</script>

<main>
	<header>
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
