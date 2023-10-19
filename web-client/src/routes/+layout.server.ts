import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ cookies }) => {
	const authed = cookies.get('authed') === 'true';

	return {
		authed
	};
};
