import type { Response as SFResponse } from '@cloudflare/workers-types';
import { error } from '@sveltejs/kit';
import { getPostAttachment } from '../../../../lib';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = async ({ platform, params, request }) => {
	const { id: attachmentId } = params;

	if (!attachmentId) {
		return error(404);
	}

	const cachedResponse = await platform?.caches?.default.match(request.url);

	if (cachedResponse) {
		return cachedResponse as unknown as Response; // TODO: Fix this type cast
	}

	try {
		const attachment = await getPostAttachment(attachmentId);

		const maxAge = 60 * 60 * 24 * 365; // 1 year

		const response = new Response(attachment.attachment, {
			headers: {
				'Content-Disposition': attachment.contentDisposition,
				'Cache-Control': `public, max-age=${maxAge}; immutable`,
				Expires: new Date(Date.now() + maxAge * 1000).toUTCString()
			}
		});

		await platform?.caches?.default.put(
			request.url,
			response.clone() as unknown as SFResponse // TODO: Fix this type cast
		);

		return response;
	} catch (e) {
		console.error(e);
		return error(404);
	}
};
