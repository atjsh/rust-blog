<script lang="ts">
	import { goto, invalidateAll } from '$app/navigation';
	import { logout } from '../../lib/api';
	import JsCookie from 'js-cookie';

	async function afterLogout() {
		JsCookie.remove('authed');
		await logout();
		await invalidateAll();
		goto('/');
	}
</script>

{#await afterLogout()}{/await}
