<script lang="ts">
	import { PostAd, type PostContentType } from '../api';
	import PostRenderAdCoupang from './post-renderers/post-render-ad-coupang.svelte';
	import PostRenderAdFree from './post-renderers/post-render-ad-free.svelte';

	export let renderedContent: string;
	export let contentType: PostContentType;
	export let ad: PostAd;
</script>

{#if ad === PostAd.CoupangAd}
	<PostRenderAdCoupang {renderedContent} {contentType} />
{:else}
	<PostRenderAdFree {renderedContent} {contentType} />
{/if}
