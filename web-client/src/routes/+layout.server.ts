import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ cookies }) => {
	const authed = cookies.get('accessToken') !== undefined;

	return {
		authed: authed
	};
};
