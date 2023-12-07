import "./App.css";

export function App() {
  return (
    <>
      <header>
        <a href="/" className="title-a">
          <div className="title-content">
            <div className="icon">🫠</div>
            <code>blog.atj.sh</code>
          </div>
        </a>
      </header>
      <main>
        <h1>전성훈의 블로그</h1>
        <ul>
          <li>
            <h2>
              <a href={`/category`}>ㅇㅇ</a> 카테고리의 글 목록
            </h2>
          </li>
        </ul>
      </main>
      <footer></footer>
    </>
  );
}
