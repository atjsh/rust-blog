export function PostListGrid({
  posts
}: {
  posts: Array<{
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
}) {
  return (
    <ul className="postlist-grid-container">
      {posts.map((post) => {
        return (
          <li key={post.id}>
            <div className="postlist-grid-item">
              <div className="postlist-grid-item-header">
                <div className="postlist-grid-item-header-title">
                  {post.title}
                </div>
                <div className="postlist-grid-item-header-date">
                  {post.created_at}
                </div>
              </div>
              <div className="postlist-grid-item-body">
                <div className="postlist-grid-item-body-written-by">
                  {post.written_by.email}
                </div>
                <div className="postlist-grid-item-body-category">
                  {post.category.name}
                </div>
              </div>
            </div>
          </li>
        );
      })}
    </ul>
  );
}
