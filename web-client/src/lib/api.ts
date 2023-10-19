import returnFetch from 'return-fetch';
import { PUBLIC_SERVER_URL } from '$env/static/public';

const serverFetch = returnFetch({
	baseUrl: PUBLIC_SERVER_URL,
	interceptors: {
		request: async (requestArgs) => {
			return [
				requestArgs[0],
				{
					...requestArgs[1],
					credentials: 'include'
				}
			];
		}
	}
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

export async function getAuthed(): Promise<boolean> {
	try {
		await serverFetch(`/auth`, {
			method: 'PUT'
		});

		return true;
	} catch (error) {
		console.error(error);
		return false;
	}
}

export async function getCategories(): Promise<GetCategoryResponseData[]> {
	return (await (
		await serverFetch(`/category`, {
			method: 'GET'
		})
	).json()) as GetCategoryResponseData[];
}

export async function getCategory(categoryId: number): Promise<GetCategoryResponseData> {
	return (await (
		await serverFetch(`/category/${categoryId}`, {
			method: 'GET'
		})
	).json()) as GetCategoryResponseData;
}

export async function getCategoryPosts(categoryId: number): Promise<GetCategoryPostsResponseData> {
	return (await (
		await serverFetch(`/category/${categoryId}/posts`, {
			method: 'GET'
		})
	).json()) as GetCategoryPostsResponseData;
}

export async function getPost(postId: number): Promise<GetPostResponseData> {
	return (await (
		await serverFetch(`/post/${postId}`, {
			method: 'GET'
		})
	).json()) as GetPostResponseData;
}

export async function createPost(
	categoryId: number,
	title: string,
	content: string
): Promise<GetPostResponseData> {
	return (await (
		await serverFetch(`/category/${categoryId}/posts`, {
			method: 'POST',
			body: JSON.stringify({
				title,
				content
			})
		})
	).json()) as GetPostResponseData;
}

export async function getWriter(writerId: number): Promise<GetWriterResponseData> {
	return (await (
		await serverFetch(`/writer/${writerId}`, {
			method: 'GET'
		})
	).json()) as GetWriterResponseData;
}

export async function getWriterPosts(writerId: number): Promise<GetWriterPostResponseData> {
	return (await (
		await serverFetch(`/writer/${writerId}/posts`, {
			method: 'GET'
		})
	).json()) as GetWriterPostResponseData;
}

export async function updateProfile(email: string | null, description: string | null) {
	await serverFetch(`/writer/profile`, {
		method: 'PUT',
		body: JSON.stringify({
			email,
			description
		})
	});
}
