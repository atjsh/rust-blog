import { Outlet, ScrollRestoration, Link } from "react-router-dom";

export const AppLayout: React.FC = () => {
  return (
    <>
      <header>
        <Link to="/" className="title-a">
          <div className="title-content">
            <div className="icon">🫠</div>
            <code>blog.atj.sh</code>
          </div>
        </Link>
      </header>
      <ScrollRestoration />
      <Outlet />
      <footer>
        <p>
          2023 <a href="https://atj.sh/">atjsh</a>. All rights reserved. <br />
          <br />
        </p>
        <Link to="/login">Login</Link>
      </footer>
    </>
  );
};
