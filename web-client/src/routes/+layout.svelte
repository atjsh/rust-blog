<script lang="ts">
	import type { LayoutData } from './$types';

	export let data: LayoutData;
</script>

<main>
	<header>
		<a href="/">This is rust-blog</a>
	</header>
	<slot />
	<footer>2023 ATJSH. All rights reserved</footer>
</main>
