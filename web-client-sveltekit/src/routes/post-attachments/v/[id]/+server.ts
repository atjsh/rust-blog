import { error } from '@sveltejs/kit';
import { getPostAttachment } from '../../../../lib';
import type { RequestHandler } from './$types';
import type { Response as SFResponse } from '@cloudflare/workers-types';

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

		const response = new Response(attachment.attachment, {
			headers: {
				'Content-Disposition': attachment.contentDisposition,
				'Cache-Control': 'public, max-age=2630000; immutable',
				Expires: new Date(Date.now() + 2630000).toUTCString(),
				'x-test': new Date().toISOString()
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
