import { PUBLIC_SERVER_URL } from '$env/static/public';
import returnFetch from 'return-fetch';

export type PostContentType = 'html' | 'md';

export const PostAd = {
	NoAd: 0,
	CoupangAd: 1
} as const;
export type PostAd = (typeof PostAd)[keyof typeof PostAd];

export function isPostAd(value: unknown): value is PostAd {
	return typeof value === 'number' && Object.values(PostAd).map(Number).includes(value);
}

const serverFetch = returnFetch({
	baseUrl: PUBLIC_SERVER_URL
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
	content_type: PostContentType;
	private: boolean;
	ad: PostAd;
	created_at: string;

	written_by: {
		id: number;
		email: string;
	};
	category: {
		id: number;
		name: string;
	};
};

export type GetWriterPostResponseData = Array<{
	id: number;

	title: string;
	private: boolean;
	created_at: string;

	written_by: {
		id: number;
		email: string;
	};
	category: {
		id: number;
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
	password: string;
};

export type CreatePostAttachmentResponseData = {
	id: number;
	created_at: string;
};

export async function getAccessToken(payload: GetAuthedPayload): Promise<string> {
	const response = await serverFetch(`/auth/access-token`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(payload)
	});

	if (response.status !== 200) {
		throw Error('Failed to get access token');
	}

	return response.text();
}

export async function getCurrentAuthedWriterId(accessToken: string): Promise<number> {
	const response = await serverFetch('/profile', {
		method: 'GET',
		headers: {
			Authorization: `Bearer ${accessToken}`
		}
	});

	return Number(await response.text());
}

export async function getCurrentAuthedWriterPosts(
	accessToken: string
): Promise<GetWriterPostResponseData> {
	const response = await serverFetch('/profile/posts', {
		method: 'GET',
		headers: {
			Authorization: `Bearer ${accessToken}`
		}
	});

	return (await response.json()) as GetWriterPostResponseData;
}

export async function logout() {
	await serverFetch(`/auth`, {
		method: 'DELETE'
	});
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
	const response = await serverFetch(`/post/${postId}`, {
		method: 'GET'
	});

	if (response.status !== 200) {
		throw Error('Failed to get post');
	}

	return (await response.json()) as GetPostResponseData;
}

export async function createPostAttachment(
	attachment: FormData,
	accessToken: string
): Promise<CreatePostAttachmentResponseData> {
	const response = await serverFetch(`/post-attachment`, {
		method: 'POST',
		body: attachment,
		headers: {
			Authorization: `Bearer ${accessToken}`
		}
	});

	if (response.status !== 200) {
		throw Error(`Failed to create post attachment: ${await response.text()}`);
	}

	return (await response.json()) as CreatePostAttachmentResponseData;
}

export async function getPostAttachment(attachmentId: string): Promise<{
	attachment: Blob;
	contentDisposition: string;
}> {
	const response = await serverFetch(`/post-attachment/${attachmentId}`, {
		method: 'GET'
	});

	if (response.status !== 200) {
		throw Error('Failed to get post attachment');
	}

	return {
		attachment: await response.blob(),
		contentDisposition: response.headers.get('Content-Disposition') || ''
	};
}

export async function createPost(
	categoryId: number,
	title: string,
	content: string,
	contentType: PostContentType,
	isPrivate: boolean,
	ad: PostAd,
	accessToken: string
): Promise<GetPostResponseData> {
	return (await (
		await serverFetch(`/post`, {
			method: 'POST',
			body: JSON.stringify({
				title,
				content,
				content_type: contentType,
				is_private: isPrivate,
				ad,
				category_id: categoryId
			}),
			headers: {
				Authorization: `Bearer ${accessToken}`,
				'Content-Type': 'application/json'
			}
		})
	).json()) as GetPostResponseData;
}

export async function updatePost(
	postId: number,
	categoryId: number,
	title: string,
	content: string,
	contenyType: PostContentType,
	isPrivate: boolean,
	ad: PostAd,
	accessToken: string
): Promise<GetPostResponseData> {
	const response = await serverFetch(`/post/${postId}`, {
		method: 'PATCH',
		body: JSON.stringify({
			title,
			content,
			content_type: contenyType,
			is_private: isPrivate,
			category_id: categoryId,
			ad
		}),
		headers: {
			Authorization: `Bearer ${accessToken}`,
			'Content-Type': 'application/json'
		}
	});

	return (await response.json()) as GetPostResponseData;
}

export async function deletePost(postId: number, accessToken: string) {
	await serverFetch(`/post/${postId}`, {
		method: 'DELETE',
		headers: {
			Authorization: `Bearer ${accessToken}`
		}
	});
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
