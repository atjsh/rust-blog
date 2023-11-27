import { getCategories, getCategoryPosts } from '$lib/api';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
	const categories = await getCategories();

	return {
		categories: await Promise.all(
			categories.map(async (category) => ({
				...category,
				posts: await getCategoryPosts(category.id)
			}))
		)
	};
};
