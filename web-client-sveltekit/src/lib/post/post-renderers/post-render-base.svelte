<script lang="ts">
	import type { PostContentType } from '../../api';

	export let renderedContent: string;
	export let contentType: PostContentType;
</script>

<div class="post-container">
	<div class="post-content" class:md={contentType === 'md'} class:html={contentType === 'html'}>
		{@html renderedContent}
	</div>
</div>

<style lang="scss">
	.post-container {
		.post-content {
			word-break: keep-all;
			word-wrap: break-word;
			margin: auto;
		}
	}

	:global(
		.post-content h1,
		.post-content h2,
		.post-content h3,
		.post-content h4,
		.post-content h5,
		.post-content h6
	) {
		color: #005a9c;
		font-weight: 600;
	}

	:global(.post-content h1) {
		display: block;
		font-size: 2em;
		margin-top: 0.67em;
		margin-bottom: 0.67em;
		margin-left: 0;
		margin-right: 0;
	}

	:global(.post-content h2) {
		display: block;
		font-size: 1.5em;
		margin-top: 0.83em;
		margin-bottom: 0.83em;
		margin-left: 0;
		margin-right: 0;
	}

	:global(.post-content h3) {
		display: block;
		font-size: 1.17em;
		margin-top: 1em;
		margin-bottom: 1em;
		margin-left: 0;
		margin-right: 0;
	}

	:global(.post-content h4) {
		display: block;
		font-size: 1em;
		margin-top: 1.33em;
		margin-bottom: 1.33em;
		margin-left: 0;
		margin-right: 0;
	}

	:global(.post-content h5) {
		display: block;
		font-size: 0.83em;
		margin-top: 1.67em;
		margin-bottom: 1.67em;
		margin-left: 0;
		margin-right: 0;
	}

	:global(.post-content h6) {
		display: block;
		font-size: 0.67em;
		margin-top: 2.33em;
		margin-bottom: 2.33em;
		margin-left: 0;
		margin-right: 0;
	}

	:global(.post-content p) {
		margin-top: 1em;
		margin-bottom: 1em;
		line-height: 1.6;
	}

	:global(.post-content pre) {
		max-width: 100%;
		overflow-x: auto;
		padding-left: 2em;
		padding-right: 2em;
	}

	:global(.post-content.md img) {
		max-width: 100%;
	}

	:global(.post-content li) {
		list-style-position: inside;
		margin-left: 1em;
	}

	:global(.post-content img.half) {
		width: 50%;
	}

	:global(.post-content img.full) {
		width: 100%;
	}

	:global(.post-content img.center) {
		display: block;
		margin-left: auto;
		margin-right: auto;
	}

	:global(.post-content img.size-2-3) {
		width: 66.66666667%;
	}

	:global(.post-content img.size-1-3) {
		width: 33.33333333%;
	}
</style>
