<script lang="ts">
	import type { ActionData } from './$types';

	export let form: ActionData;
</script>

<main>
	<h1>Login</h1>
	<form method="post">
		<input type="email" id="email" name="email" required placeholder="e-mail" />
		<button type="submit">Login</button>
	</form>
	{#if form?.incorrect}
		<p style="color: red">⚠️ Failed to login; Check E-mail address</p>
	{/if}
</main>
