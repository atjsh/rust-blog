<script lang="ts">
	import type { ActionData } from './$types';

	export let form: ActionData;
</script>

<div>
	<h1>WebMaster Login</h1>
	<form method="post">
		<input type="email" id="email" name="email" required placeholder="email" />
		<input type="password" name="password" id="password" required placeholder="password" />
		<button type="submit">Login</button>
	</form>
	{#if form?.incorrect}
		<p style="color: red">⚠️ Failed to login</p>
	{/if}
</div>

<svelte:head>
	<title>WebMaster Login | Personal Blog from atjsh</title>
</svelte:head>

<style lang="scss">
	input {
		width: 15rem;
	}

	div {
		display: flex;
		flex-direction: column;
		padding: 2em;
		gap: 2em;

		h1 {
			color: #2c5e96;
		}

		form {
			display: flex;
			flex-direction: column;
			gap: 0.4em;

			button {
				margin-top: 1em;
				width: 15rem;
				padding: 0.5em;
			}
		}
	}
</style>
