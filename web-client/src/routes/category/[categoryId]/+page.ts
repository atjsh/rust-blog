import { getCategory, getCategoryPosts } from '../../../lib/api';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params }) => {
	const { categoryId } = params;

	return {
		category: await getCategory(Number(categoryId)),
		posts: await getCategoryPosts(Number(categoryId))
	};
};
