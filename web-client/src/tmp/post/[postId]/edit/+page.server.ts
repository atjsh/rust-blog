import { COOKIE_SECRET } from '$env/static/private';
import { getCategories, getPost, updatePost } from '$lib';
import { fail, redirect, type Actions } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import CryptoJS from 'crypto-js';

export const load: PageServerLoad = async ({ params }) => {
	const { postId } = params;

	const post = await getPost(Number(postId));
	const categories = await getCategories();

	return {
		categories,
		post
	};
};

export const actions: Actions = {
	default: async ({ request, cookies, params }) => {
		const { postId } = params;

		if (!postId) {
			return fail(400, {
				error: 'postId is required'
			});
		}

		const data = await request.formData();
		const title = data.get('title');
		const content = data.get('content');
		const categoryId = data.get('categoryId');

		if (!categoryId || !title || !content) {
			return fail(400, {
				error: 'categoryId, title, and content are required'
			});
		}

		const accessToken = cookies.get('accessToken', {
			decode: (value) => CryptoJS.AES.decrypt(value, COOKIE_SECRET).toString(CryptoJS.enc.Utf8)
		});

		if (!accessToken) {
			throw redirect(301, '/login');
		}

		const post = await updatePost(
			Number(postId),
			Number(categoryId),
			title.toString(),
			content.toString(),
			accessToken
		);

		throw redirect(301, `/post/${post.id}`);
	}
};
