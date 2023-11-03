import { getWriter, getWriterPosts } from '$lib/api';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ params }) => {
	const { writerId } = params;

	return {
		writer: await getWriter(Number(writerId)),
		posts: await getWriterPosts(Number(writerId))
	};
};
