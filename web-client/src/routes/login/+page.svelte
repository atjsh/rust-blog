<script lang="ts">
	import { goto, invalidateAll } from '$app/navigation';
	import { PUBLIC_WEB_CLIENT_URL } from '$env/static/public';
	import JsCookie from 'js-cookie';
	import { getAuthed } from '../../lib/api';

	export let result = '';

	async function sendAuth(event: { currentTarget: EventTarget & HTMLFormElement }) {
		const data = new FormData(event.currentTarget);

		const authResult = await getAuthed({
			email: String(data.get('email'))
		});

		if (authResult) {
			result = 'success';
			JsCookie.set('authed', 'true', {
				domain: PUBLIC_WEB_CLIENT_URL,
				expires: 30
			});

			await invalidateAll();
			goto('/');
		} else {
			result = 'failed';
		}
	}
</script>

<main>
	<h1>Login</h1>
	<form on:submit|preventDefault={sendAuth}>
		<input type="email" id="email" name="email" required placeholder="e-mail" />
		<button type="submit">Login</button>
	</form>
</main>
