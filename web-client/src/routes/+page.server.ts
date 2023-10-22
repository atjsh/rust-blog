import { getCategories } from '$lib/api';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
	return {
		categories: await getCategories()
	};
};
