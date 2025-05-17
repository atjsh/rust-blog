import { getCategories, getCategoryPosts } from '../lib';
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ cookies }) => {
	const authed = cookies.get('accessToken') !== undefined;
	const categories = await getCategories();

	const sortedCategories = categories.sort((a, b) => {
		if (new Date(a.created_at) < new Date(b.created_at)) {
			return 1;
		} else if (new Date(a.created_at) > new Date(b.created_at)) {
			return -1;
		} else {
			return 0;
		}
	});

	return {
		authed: authed,
		categories: await Promise.all(
			sortedCategories.map(async (category) => ({
				...category,
				posts: await getCategoryPosts(category.id)
			}))
		)
	};
};
