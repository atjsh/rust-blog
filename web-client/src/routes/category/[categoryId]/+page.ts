import { getCategoryPosts } from '../../../lib/api';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params }) => {
	const { categoryId } = params;

	return {
		posts: await getCategoryPosts(Number(categoryId))
	};
};
