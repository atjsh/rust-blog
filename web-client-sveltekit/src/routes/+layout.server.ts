import { getCategories, getCategoryPosts } from '../lib';
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ cookies }) => {
	const authed = cookies.get('accessToken') !== undefined;
	const categories = await getCategories();

	return {
		authed: authed,
		categories: await Promise.all(
			categories.map(async (category) => ({
				...category,
				posts: await getCategoryPosts(category.id)
			}))
		)
	};
};
