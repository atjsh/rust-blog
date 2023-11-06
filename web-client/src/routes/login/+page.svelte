<script lang="ts">
	import type { ActionData } from './$types';

	export let form: ActionData;
</script>

<main>
	<h1>Login</h1>
	<form method="post">
		<input
			type="email"
			id="email"
			name="email"
			required
			placeholder="username:password@example.com"
		/>
		<button type="submit">Login</button>
	</form>
	{#if form?.incorrect}
		<p style="color: red">⚠️ Failed to login</p>
	{/if}
</main>

<svelte:head>
	<title>Login | blog.atj.sh</title>
</svelte:head>

<style>
	input {
		width: 15rem;
	}
</style>
