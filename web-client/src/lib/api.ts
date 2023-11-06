import { PUBLIC_SERVER_URL } from '$env/static/public';
import axios from 'axios';

const serverAxios = axios.create({
	baseURL: PUBLIC_SERVER_URL
});

export type GetCategoryResponseData = {
	id: number;
	name: string;
	created_at: string;
};

export type GetCategoryPostsResponseData = Array<{
	id: number;

	title: string;
	created_at: string;

	written_by: {
		id: number;
		email: string;
	};
	category: {
		id: string;
		name: string;
	};
}>;

export type GetPostResponseData = {
	id: number;

	title: string;
	content: string;
	created_at: string;

	written_by: {
		id: number;
		email: string;
	};
	category: {
		id: string;
		name: string;
	};
};

export type GetWriterPostResponseData = Array<{
	id: number;

	title: string;
	created_at: string;

	written_by: {
		id: number;
		email: string;
	};
	category: {
		id: string;
		name: string;
	};
}>;

export type GetWriterResponseData = {
	id: number;
	email: string;
	description: string;
};

export type GetAuthedPayload = {
	email: string;
};

export async function getAccessToken(payload: GetAuthedPayload): Promise<string> {
	const response = await serverAxios.post(`/auth/access-token`, payload);

	if (response.status !== 200) {
		throw Error('Failed to get access token');
	}

	return response.data;
}

export async function getCurrentAuthedWriterId(accessToken: string): Promise<number> {
	const response = await serverAxios.get('/profile', {
		headers: {
			Authorization: `Bearer ${accessToken}`
		}
	});

	return Number(await response.data);
}

export async function logout() {
	await serverAxios.delete(`/auth`);
}

export async function getCategories(): Promise<GetCategoryResponseData[]> {
	return (await (
		await serverAxios.get(`/category`)
	).data) as GetCategoryResponseData[];
}

export async function getCategory(categoryId: number): Promise<GetCategoryResponseData> {
	return (await (
		await serverAxios.get(`/category/${categoryId}`)
	).data) as GetCategoryResponseData;
}

export async function getCategoryPosts(categoryId: number): Promise<GetCategoryPostsResponseData> {
	return (await (
		await serverAxios.get(`/category/${categoryId}/posts`)
	).data) as GetCategoryPostsResponseData;
}

export async function getPost(postId: number): Promise<GetPostResponseData> {
	return (await (
		await serverAxios.get(`/post/${postId}`)
	).data) as GetPostResponseData;
}

export async function createPost(
	categoryId: number,
	title: string,
	content: string,
	accessToken: string
): Promise<GetPostResponseData> {
	return (await (
		await serverAxios.post(
			`/post`,
			{
				title,
				content,
				category_id: categoryId
			},
			{
				headers: {
					Authorization: `Bearer ${accessToken}`
				}
			}
		)
	).data) as GetPostResponseData;
}

export async function updatePost(
	postId: number,
	categoryId: number,
	title: string,
	content: string,
	accessToken: string
): Promise<GetPostResponseData> {
	const response = await serverAxios.patch(
		`/post/${postId}`,
		{
			title,
			content,
			category_id: categoryId
		},
		{
			headers: {
				Authorization: `Bearer ${accessToken}`
			}
		}
	);

	return response.data as GetPostResponseData;
}

export async function deletePost(postId: number, accessToken: string) {
	await serverAxios.delete(`/post/${postId}`, {
		headers: {
			Authorization: `Bearer ${accessToken}`
		}
	});
}

export async function getWriter(writerId: number): Promise<GetWriterResponseData> {
	return (await serverAxios.get(`/writer/${writerId}`)).data as GetWriterResponseData;
}

export async function getWriterPosts(writerId: number): Promise<GetWriterPostResponseData> {
	return (await serverAxios.get(`/writer/${writerId}/posts`)).data as GetWriterPostResponseData;
}

export async function updateProfile(email: string | null, description: string | null) {
	await serverAxios.put(`/writer/profile`, {
		email,
		description
	});
}
