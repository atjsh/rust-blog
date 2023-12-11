import { useLoaderData } from "react-router-dom";

export function Component() {
  const categories = useLoaderData();
  return <div>{JSON.stringify(categories)}</div>;
}
