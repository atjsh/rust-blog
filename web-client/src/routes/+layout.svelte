<script lang="ts">
	import type { LayoutServerData } from './$types';

	export let data: LayoutServerData;
</script>

<main>
	<header>
		<a href="/" class="title-a">
			<div>
				<div class="icon">🫠</div>
				<code>blog.atj.sh</code>
			</div>
		</a>
	</header>
	<slot />
	<footer>
		<p>
			2023 <a href="https://atj.sh/">atjsh</a>. All rights reserved
		</p>
		{#if data.authed}
			<a href="/profile">Profile</a> /
			<a href="/logout" data-sveltekit-preload-data="tap">Logout</a> /
			<a href="/post/new">New Post</a>
		{/if}
	</footer>
</main>

<style>
	.icon {
		font-size: 3em;
		margin-top: 0.5em;
		margin-bottom: 0.1em;
	}

	.title-a {
		text-decoration: none;

		&:hover {
			text-decoration: underline;
		}
	}
</style>
