import {
  Route,
  createBrowserRouter,
  createRoutesFromElements
} from "react-router-dom";
import { AppLayout } from "./layouts/AppLayout";
// import { IndexPage } from "./pages/IndexPage";
import { getCategories, getCategoryPosts } from "./api";

export const router = createBrowserRouter(
  createRoutesFromElements(
    <Route element={<AppLayout />}>
      <Route
        path="/"
        lazy={() => import("./pages/IndexPage")}
        loader={async () => {
          const categories = await getCategories();

          return {
            categories: await Promise.all(
              categories.map(async (category) => ({
                ...category,
                posts: await getCategoryPosts(category.id)
              }))
            )
          };
        }}
      ></Route>
      <Route path="/login" lazy={() => import("./pages/LoginPage")}></Route>
    </Route>
  )
);
