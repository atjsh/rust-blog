import returnFetch from "return-fetch";

export type PostContentType = "html" | "md";

const serverFetch = returnFetch({
  baseUrl: import.meta.env.VITE_SERVER_URL
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
};

export async function getAccessToken(
  payload: GetAuthedPayload
): Promise<string> {
  const response = await serverFetch(`/auth/access-token`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json"
    },
    body: JSON.stringify(payload)
  });

  if (response.status !== 200) {
    throw Error("Failed to get access token");
  }

  return response.text();
}

export async function getCurrentAuthedWriterId(
  accessToken: string
): Promise<number> {
  const response = await serverFetch("/profile", {
    method: "GET",
    headers: {
      Authorization: `Bearer ${accessToken}`
    }
  });

  return Number(await response.text());
}

export async function getCurrentAuthedWriterPosts(
  accessToken: string
): Promise<GetWriterPostResponseData> {
  const response = await serverFetch("/profile/posts", {
    method: "GET",
    headers: {
      Authorization: `Bearer ${accessToken}`
    }
  });

  return (await response.json()) as GetWriterPostResponseData;
}

export async function getCategories(): Promise<GetCategoryResponseData[]> {
  return (await (
    await serverFetch(`/category`, {
      method: "GET"
    })
  ).json()) as GetCategoryResponseData[];
}

export async function getCategory(
  categoryId: number
): Promise<GetCategoryResponseData> {
  return (await (
    await serverFetch(`/category/${categoryId}`, {
      method: "GET"
    })
  ).json()) as GetCategoryResponseData;
}

export async function getCategoryPosts(
  categoryId: number
): Promise<GetCategoryPostsResponseData> {
  return (await (
    await serverFetch(`/category/${categoryId}/posts`, {
      method: "GET"
    })
  ).json()) as GetCategoryPostsResponseData;
}

export async function getPost(postId: number): Promise<GetPostResponseData> {
  const response = await serverFetch(`/post/${postId}`, {
    method: "GET"
  });

  if (response.status !== 200) {
    throw Error("Failed to get post");
  }

  return (await response.json()) as GetPostResponseData;
}

export async function getWriter(
  writerId: number
): Promise<GetWriterResponseData> {
  return (await (
    await serverFetch(`/writer/${writerId}`, {
      method: "GET"
    })
  ).json()) as GetWriterResponseData;
}

export async function getWriterPosts(
  writerId: number
): Promise<GetWriterPostResponseData> {
  return (await (
    await serverFetch(`/writer/${writerId}/posts`, {
      method: "GET"
    })
  ).json()) as GetWriterPostResponseData;
}
