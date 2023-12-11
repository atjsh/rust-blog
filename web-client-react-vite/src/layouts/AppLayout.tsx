import { Outlet, ScrollRestoration } from "react-router-dom";

export const AppLayout: React.FC = () => {
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
      <ScrollRestoration />
      <Outlet />
      <footer></footer>
    </>
  );
};
