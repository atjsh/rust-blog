import { error, json } from '@sveltejs/kit';
import { createPostAttachment } from '../../lib';
import { getAccessTokenFromCookie } from '../../lib/access-token/utils';
import type { RequestHandler } from './$types';
import { PUBLIC_WEB_URL } from '$env/static/public';

export const POST: RequestHandler = async ({ request, cookies }) => {
	const form = await request.formData();
	const file = form.get('attachment');
	const fileExtension = form.get('fileExtension');

	const accessToken = getAccessTokenFromCookie(cookies);

	if (!file) {
		return error(400, { message: 'attachment is empty; attachment is required' });
	}
	if (!(file instanceof File)) {
		return error(400, { message: 'attachment is not a file; attachment must be a file' });
	}
	if (!fileExtension) {
		return error(400, { message: 'fileExtension is empty; fileExtension is required' });
	}
	if (typeof fileExtension !== 'string') {
		return error(400, { message: 'fileExtension is not a string; fileExtension must be a string' });
	}

	const requestForm = new FormData();
	requestForm.append('attachment', file);
	requestForm.append('file_extension', fileExtension);

	try {
		const creationResult = await createPostAttachment(requestForm, accessToken);

		const responseJson = {
			id: creationResult.id,
			url: `${PUBLIC_WEB_URL}/post-attachments/v/${creationResult.id}`
		};

		return json(responseJson);
	} catch (e) {
		console.error(e);
		return error(500, { message: 'Failed to create post attachment' });
	}
};
