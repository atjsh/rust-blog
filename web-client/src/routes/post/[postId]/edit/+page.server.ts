import { getCategories, getPost, updatePost, type PostContentType } from '$lib';
import { fail, redirect, type Actions } from '@sveltejs/kit';
import { getAccessTokenFromCookie } from '../../../../lib/access-token/utils';
import type { PageServerLoad } from './$types';

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
		const isPrivate = data.get('isPrivate');
		const categoryId = data.get('categoryId');
		const contentType = data.get('contentType');

		if (!categoryId || !title || !content || !contentType) {
			return fail(400, {
				error: 'categoryId, title, content are required'
			});
		}

		const accessToken = getAccessTokenFromCookie(cookies);

		const post = await updatePost(
			Number(postId),
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
