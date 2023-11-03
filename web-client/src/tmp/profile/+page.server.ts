import { COOKIE_SECRET } from '$env/static/private';
import { getCurrentAuthedWriterId, getWriter, getWriterPosts } from '$lib';
import { redirect } from '@sveltejs/kit';
import CryptoJS from 'crypto-js';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ cookies }) => {
	const accessToken = cookies.get('accessToken', {
		decode: (value) => CryptoJS.AES.decrypt(value, COOKIE_SECRET).toString(CryptoJS.enc.Utf8)
	});

	if (!accessToken) {
		throw redirect(301, '/login');
	}

	const writerId = await getCurrentAuthedWriterId(accessToken);

	return {
		writer: await getWriter(writerId),
		posts: await getWriterPosts(writerId)
	};
};
