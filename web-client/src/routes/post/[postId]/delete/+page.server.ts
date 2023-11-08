import { deletePost, getCurrentAuthedWriterId, getPost } from '$lib';
import { redirect } from '@sveltejs/kit';
import { getAccessTokenFromCookie } from '../../../../lib/access-token/utils';
import type { Actions, PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ cookies, params }) => {
	const { postId } = params;

	const post = await getPost(Number(postId));

	const accessToken = getAccessTokenFromCookie(cookies);

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

		const accessToken = getAccessTokenFromCookie(cookies);

		await deletePost(Number(postId), accessToken);

		throw redirect(301, '/');
	}
} satisfies Actions;
