<script lang="ts">
	import type { PostContentType } from '../../api';
	import PostRenderBase from './post-render-base.svelte';

	export let renderedContent: string;
	export let contentType: PostContentType;
</script>

<hr />

<PostRenderBase {renderedContent} {contentType} />
