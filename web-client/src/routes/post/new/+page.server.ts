import { fail, redirect, type Actions } from '@sveltejs/kit';
import { createPost, getCategories } from '../../../lib';
import { getAccessTokenFromCookie } from '../../../lib/access-token/utils';
import type { PageServerLoad } from './$types';

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

		const accessToken = getAccessTokenFromCookie(cookies);

		const post = await createPost(
			Number(categoryId),
			title.toString(),
			content.toString(),
			accessToken
		);

		throw redirect(301, `/post/${post.id}`);
	}
};
