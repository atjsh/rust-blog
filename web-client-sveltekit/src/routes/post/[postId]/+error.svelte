<script lang="ts">
	import { page } from '$app/stores';
</script>

<div>
	<h1>{$page.error?.message}</h1>
	<a href="/">홈으로 가기</a>
</div>

<style>
	div {
		padding: 2em;
		display: flex;
		flex-direction: column;
		gap: 1em;
	}
</style>
