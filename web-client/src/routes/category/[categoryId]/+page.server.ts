import { getCategory, getCategoryPosts } from '$lib/api';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ params }) => {
	const { categoryId } = params;

	return {
		category: await getCategory(Number(categoryId)),
		posts: await getCategoryPosts(Number(categoryId))
	};
};
