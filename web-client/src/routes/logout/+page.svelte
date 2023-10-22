<script lang="ts">
	import { goto, invalidateAll } from '$app/navigation';

	async function afterLogout() {
		await invalidateAll();
		goto('/');
	}
</script>

<main>
	{#await afterLogout()}{/await}
</main>
