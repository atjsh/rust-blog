<script lang="ts">
	import type { LayoutData } from './$types';

	export let data: LayoutData;
</script>

<main>
	<header>
		<a href="/">This is rust-blog</a><br />
		{#if data.authed}
			<a href="/logout" data-sveltekit-preload-data="tap">Logout</a> /
			<a href="/profile">Profile</a>
		{:else}
			<a href="/login">Login</a>
		{/if}
	</header>
	<slot />
	<footer>2023 ATJSH. All rights reserved</footer>
</main>
