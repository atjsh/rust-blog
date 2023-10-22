<script lang="ts">
	import { goto, invalidateAll } from '$app/navigation';

	invalidateAll().then(() => goto('/'));
</script>
