import type { MetaFunction } from "@remix-run/node";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "~/components/ui/select";

export const meta: MetaFunction = () => {
  return [{ title: "New Remix App" }, { name: "description", content: "Welcome to Remix!" }];
};

const ranks = [
  "Ace",
  "Two",
  "Three",
  "Four",
  "Five",
  "Six",
  "Seven",
  "Eight",
  "Nine",
  "Ten",
  "Jack",
  "Queen",
  "King",
];

const suits = ["Hearts", "Diamonds", "Spades", "Clubs"];

export default function Index() {
  return (
    <div className="h-dvh w-dvh bg-background text-foreground">
      <div className="h-full w-full flex items-center">
        <div className="container bg-muted h-2/3 w-2/3 px-12 py-8 border rounded-md border-solid border-slate-500">
          <div className="h-full w-full flex">
            <div className="w-1/4 h-full pr-8">
              <h4 className="scroll-m-20 text-xl font-semibold tracking-tight">Rank</h4>
              <Select>
                <SelectTrigger className="mt-2 border-violet-800 bg-muted">
                  <SelectValue placeholder="Select a rank" />
                </SelectTrigger>
                <SelectContent>
                  <SelectGroup>
                    {ranks.map((rank) => (
                      <SelectItem key={rank} value={rank}>
                        {rank}
                      </SelectItem>
                    ))}
                  </SelectGroup>
                </SelectContent>
              </Select>
              <h4 className="scroll-m-20 mt-4 text-xl font-semibold tracking-tight">Suit</h4>
              <Select>
                <SelectTrigger className="mt-2 border-violet-800 bg-muted">
                  <SelectValue placeholder="Select a suit" />
                </SelectTrigger>
                <SelectContent>
                  <SelectGroup>
                    {suits.map((suit) => (
                      <SelectItem key={suit} value={suit}>
                        {suit}
                      </SelectItem>
                    ))}
                  </SelectGroup>
                </SelectContent>
              </Select>
            </div>
            <div className="w-3/4 h-full pl-8 border-l border-solid border-slate-500"></div>
          </div>
        </div>
      </div>
    </div>
  );
}
