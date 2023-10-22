import { COOKIE_SECRET } from '$env/static/private';
import { getCurrentAuthedWriterId, getPost } from '$lib';
import CryptoJS from 'crypto-js';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ cookies, params }) => {
	const { postId } = params;

	const post = await getPost(Number(postId));

	let isWriter = false;

	const accessToken = cookies.get('accessToken', {
		decode: (value) => CryptoJS.AES.decrypt(value, COOKIE_SECRET).toString(CryptoJS.enc.Utf8)
	});

	if (accessToken) {
		const currentAuthedWriterId = await getCurrentAuthedWriterId(accessToken);

		isWriter = post.written_by.id === currentAuthedWriterId;
	}

	return {
		post,
		isWriter
	};
};
