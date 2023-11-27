import { COOKIE_SECRET } from '$env/static/private';
import { redirect, type Cookies } from '@sveltejs/kit';
import CryptoJS from 'crypto-js';

export function getAccessTokenFromCookie(cookies: Cookies) {
	const accessToken = cookies.get('accessToken', {
		decode: (value) => CryptoJS.AES.decrypt(value, COOKIE_SECRET).toString(CryptoJS.enc.Utf8)
	});

	if (!accessToken) {
		throw redirect(301, '/login');
	}

	return accessToken;
}
