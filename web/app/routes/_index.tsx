import type { MetaFunction } from "@remix-run/node";
import { Link } from "@remix-run/react";
import { Button } from "~/components/ui/button";

export const meta: MetaFunction = () => {
  return [{ title: "New Remix App" }, { name: "description", content: "Welcome to Remix!" }];
};

export const handle = {
  breadcrumb: () => (
    <Button asChild>
      <Link to="/">Home</Link>
    </Button>
  ),
};

export default function Index() {
  return (
    <div>
      <ul>
        <li>
          <Link to={"card-editor"}>Cards</Link>
        </li>
        <li>
          <Link to={"nesting"}>Testing Nesting</Link>
        </li>
      </ul>
    </div>
  );
}
