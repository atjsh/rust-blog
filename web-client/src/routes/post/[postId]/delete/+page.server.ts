import { COOKIE_SECRET } from '$env/static/private';
import { deletePost, getCurrentAuthedWriterId, getPost } from '$lib';
import { redirect } from '@sveltejs/kit';
import CryptoJS from 'crypto-js';
import type { Actions, PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ cookies, params }) => {
	const { postId } = params;

	const post = await getPost(Number(postId));

	const accessToken = cookies.get('accessToken', {
		decode: (value) => CryptoJS.AES.decrypt(value, COOKIE_SECRET).toString(CryptoJS.enc.Utf8)
	});

	if (!accessToken) {
		throw redirect(301, '/login');
	}

	const currentAuthedWriterId = await getCurrentAuthedWriterId(accessToken);

	if (post.written_by.id !== currentAuthedWriterId) {
		throw redirect(301, `/post/${post.id}`);
	}

	return {
		post
	};
};

export const actions: Actions = {
	default: async ({ params, cookies }) => {
		const { postId } = params;

		if (!postId) {
			return {
				status: 400,
				body: {
					error: 'postId is required'
				}
			};
		}

		const accessToken = cookies.get('accessToken', {
			decode: (value) => CryptoJS.AES.decrypt(value, COOKIE_SECRET).toString(CryptoJS.enc.Utf8)
		});

		if (!accessToken) {
			throw redirect(301, '/login');
		}

		await deletePost(Number(postId), accessToken);

		throw redirect(301, '/');
	}
} satisfies Actions;
