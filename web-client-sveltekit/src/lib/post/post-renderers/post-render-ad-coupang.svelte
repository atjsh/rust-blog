<script lang="ts">
	import { PUBLIC_POST_AD_COUPANG_URL } from '$env/static/public';
	import type { PostContentType } from '../../api';
	import PostRenderBase from './post-render-base.svelte';

	export let renderedContent: string;
	export let contentType: PostContentType;
</script>

<div class="disclaimer">
	이 포스팅은 쿠팡 파트너스 활동의 일환으로, 이에 따른 일정액의 수수료를 제공받습니다.
</div>

<hr />

<PostRenderBase {renderedContent} {contentType} />

<div id="CoupangPartnersBannerContainer">
	<iframe
		id="CoupangPartnersBanner"
		title="Coupang Partners Ad Banner"
		src={PUBLIC_POST_AD_COUPANG_URL}
		width="100%"
		height="150"
		frameborder="0"
		scrolling="no"
		referrerpolicy="unsafe-url"
		browsingtopics="true"
	></iframe>
</div>

<style lang="scss">
	.disclaimer {
		font-size: 0.8em;
		word-break: keep-all;
	}
</style>
