import { COOKIE_SECRET } from '$env/static/private';
import {
	getCurrentAuthedWriterId,
	getHTMLFromMarkdown,
	getPost,
	type GetPostResponseData
} from '$lib';
import { error } from '@sveltejs/kit';
import CryptoJS from 'crypto-js';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ cookies, params }) => {
	const { postId } = params;

	let post: GetPostResponseData;

	try {
		post = await getPost(Number(postId));
	} catch (e) {
		throw error(404, {
			message: '글을 찾을 수 없습니다.'
		});
	}

	let isWriter = false;

	const accessToken = cookies.get('accessToken', {
		decode: (value) => CryptoJS.AES.decrypt(value, COOKIE_SECRET).toString(CryptoJS.enc.Utf8)
	});

	if (accessToken) {
		const currentAuthedWriterId = await getCurrentAuthedWriterId(accessToken);

		isWriter = post.written_by.id === currentAuthedWriterId;
	}

	if (post.private && !isWriter) {
		throw error(403, {
			message: '글을 조회할 권한이 없습니다.'
		});
	}

	return {
		post: {
			...post,
			renderedContent: post.content_type === 'md' ? getHTMLFromMarkdown(post.content) : post.content
		},
		isWriter
	};
};
