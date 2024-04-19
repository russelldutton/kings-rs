import type { MetaFunction } from "@remix-run/node";
import { Link, Outlet } from "@remix-run/react";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "~/components/ui/select";

export const meta: MetaFunction = () => {
  return [
    { title: "New Remix App" },
    { name: "description", content: "Welcome to Remix!" },
  ];
};

export default function Index() {
  return (
    <div>
      <Link to={"card-editor"}>Cards</Link>
    </div>
  );
}
