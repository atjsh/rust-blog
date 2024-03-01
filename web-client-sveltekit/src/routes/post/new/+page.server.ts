import { fail, redirect, type Actions } from '@sveltejs/kit';
import { createPost, getCategories, type PostContentType } from '../../../lib';
import { getAccessTokenFromCookie } from '../../../lib/access-token/utils';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ cookies }) => {
	getAccessTokenFromCookie(cookies);

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
		const isPrivate = data.get('isPrivate');
		const categoryId = data.get('categoryId');
		const contentType = data.get('contentType');

		if (!categoryId || !title || !content || !contentType) {
			return fail(400, {
				error: 'categoryId, title, and content are required'
			});
		}

		const accessToken = getAccessTokenFromCookie(cookies);

		const post = await createPost(
			Number(categoryId),
			title.toString(),
			content.toString(),
			contentType.toString() as PostContentType,
			isPrivate === 'on',
			accessToken
		);

		throw redirect(301, `/post/${post.id}`);
	}
};
