import { useAtom } from "jotai";
import { Button } from "../ui/button";
import { darkModeAtom } from "~/state/dark-mode";
import { PropsWithChildren, ReactNode } from "react";
import { Link, UIMatch, useMatches } from "@remix-run/react";

type Matches = UIMatch<unknown, { breadcrumb: () => ReactNode }>;

export const BaseLayout = ({ children }: PropsWithChildren) => {
  const [isDarkMode, setDarkMode] = useAtom(darkModeAtom);
  const matches = useMatches() as Matches[];

  console.log({ matches });
  return (
    <div className={`${isDarkMode ? "dark" : ""} h-dvh w-dvh bg-background text-foreground`}>
      <div className="h-full w-full flex items-center">
        <div className="container bg-muted h-2/3 w-2/3 px-12 py-8 border rounded-md border-solid border-slate-500">
          <div className="flex gap-3">
            <Button onClick={() => setDarkMode((curr) => !curr)}>Toggle Dark</Button>
            <div className="flex h-full content-center my-auto">
              <ol>
                <li>
                  <Link to="/">Home</Link>
                </li>
              </ol>
            </div>
          </div>
          <div className="h-full w-full flex">{children}</div>
        </div>
      </div>
    </div>
  );
};
