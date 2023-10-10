import { getPost } from '../../../lib/api';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params }) => {
	const { postId } = params;

	return {
		post: await getPost(Number(postId))
	};
};
