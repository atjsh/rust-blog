import { COOKIE_SECRET } from '$env/static/private';
import CryptoJS from 'crypto-js';
import { getAccessToken } from '../../lib';
import type { Actions } from './$types';

export const actions: Actions = {
	default: async ({ request, cookies }) => {
		const data = await request.formData();
		const email = data.get('email');

		let accessToken: string | undefined;

		try {
			accessToken = await getAccessToken({ email: email?.toString() ?? '' });
		} catch (error) {
			console.error(error);

			return {
				success: false
			};
		}

		cookies.set('accessToken', accessToken, {
			encode: (value) => CryptoJS.AES.encrypt(value, COOKIE_SECRET).toString(),
			httpOnly: true,
			secure: true,
			path: '/'
		});

		return {
			success: true
		};
	}
} satisfies Actions;
