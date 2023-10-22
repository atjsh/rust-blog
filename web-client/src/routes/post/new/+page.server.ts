import { COOKIE_SECRET } from '$env/static/private';
import { fail, redirect, type Actions } from '@sveltejs/kit';
import { createPost, getCategories } from '../../../lib';
import type { PageServerLoad } from './$types';
import CryptoJS from 'crypto-js';

export const load: PageServerLoad = async () => {
	const categories = await getCategories();

	return {
		categories
	};
};

export const actions: Actions = {
	default: async ({ request, cookies }) => {
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

		const post = await createPost(
			Number(categoryId),
			title.toString(),
			content.toString(),
			accessToken
		);

		throw redirect(301, `/post/${post.id}`);
	}
};
