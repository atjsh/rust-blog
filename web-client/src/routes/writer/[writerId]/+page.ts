import { getWriter, getWriterPosts } from '../../../lib/api';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params }) => {
	const { writerId } = params;

	return {
		writer: await getWriter(Number(writerId)),
		posts: await getWriterPosts(Number(writerId))
	};
};
