<script lang="ts">
	import type { PageServerData } from './$types';

	export let data: PageServerData;
</script>

<h1>Delete {data.post.title}?</h1>
<p>this action cannot be undone.</p>

<form method="post">
	<button type="submit">Delete</button>
</form>
