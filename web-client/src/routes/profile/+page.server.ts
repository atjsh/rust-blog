import { getCurrentAuthedWriterId, getWriter, getWriterPosts } from '$lib';
import { getAccessTokenFromCookie } from '../../lib/access-token/utils';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ cookies }) => {
	const accessToken = getAccessTokenFromCookie(cookies);

	const writerId = await getCurrentAuthedWriterId(accessToken);

	return {
		writer: await getWriter(writerId),
		posts: await getWriterPosts(writerId)
	};
};
