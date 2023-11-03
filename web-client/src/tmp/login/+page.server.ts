import { COOKIE_SECRET } from '$env/static/private';
import { getAccessToken } from '$lib';
import { Temporal } from '@js-temporal/polyfill';
import { fail, redirect } from '@sveltejs/kit';
import CryptoJS from 'crypto-js';
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

			return fail(403, { email, incorrect: true });
		}

		cookies.set('accessToken', accessToken, {
			encode: (value) => CryptoJS.AES.encrypt(value, COOKIE_SECRET).toString(),
			httpOnly: true,
			secure: true,
			path: '/',
			expires: new Date(Temporal.Now.instant().add({ hours: 24 * 365 }).epochMilliseconds)
		});

		throw redirect(301, '/');
	}
} satisfies Actions;
