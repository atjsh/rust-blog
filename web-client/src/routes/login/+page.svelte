<script lang="ts">
	import { goto, invalidateAll } from '$app/navigation';
	import { PUBLIC_WEB_CLIENT_URL } from '$env/static/public';
	import JsCookie from 'js-cookie';
	import { getAccessToken } from '$lib/api';

	let loginAttemptResult = '';

	async function sendAuth(event: { currentTarget: EventTarget & HTMLFormElement }) {
		const data = new FormData(event.currentTarget);

		const authResult = await getAccessToken({
			email: String(data.get('email'))
		});

		if (authResult) {
			JsCookie.set('authed', 'true', {
				domain: PUBLIC_WEB_CLIENT_URL,
				expires: 30
			});

			await invalidateAll();
			goto('/');
		} else {
			loginAttemptResult = 'failed';
		}
	}
</script>

<main>
	<h1>Login</h1>
	<form on:submit|preventDefault={sendAuth}>
		<input
			type="email"
			id="email"
			name="email"
			required
			placeholder="e-mail"
			on:input={() => {
				loginAttemptResult = '';
			}}
		/>
		<button type="submit">Login</button>
	</form>
	{#if loginAttemptResult === 'failed'}
		<p style="color: red">⚠️ Failed to login; Check E-mail address</p>
	{/if}
</main>
