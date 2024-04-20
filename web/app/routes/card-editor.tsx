import { PropsWithChildren, useState } from "react";
import { Button } from "~/components/ui/button";
import { Card } from "~/components/ui/card";
import { cn } from "~/lib/utils";
import { AnimatePresence, LayoutGroup, motion } from "framer-motion";

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
] as const;
type Rank = (typeof ranks)[number];

const suits = ["Hearts", "Diamonds", "Spades", "Clubs"] as const;
type Suit = (typeof suits)[number];

type PlayingCard = {
  rank: Rank;
  suit: Suit;
};

const displayRank = (rank: Rank) => {
  switch (rank) {
    case "Two":
      return "2";
    case "Three":
      return "3";
    case "Four":
      return "4";
    case "Five":
      return "5";
    case "Six":
      return "6";
    case "Seven":
      return "7";
    case "Eight":
      return "8";
    case "Nine":
      return "9";
    case "Ten":
      return "10";
    default:
      return rank[0];
  }
};

const range = (target: number) => Array.from(Array(target).keys()).map((i) => i + 1);

const rotationClass = (rotation: number) => {
  return rotation < 0 ? `-${Math.abs(rotation)}deg` : `${rotation}deg`;
};

const rotateConfig = [[0], [-2, 1], [-4, 0, 4], [-5, 0, 5, 10]];

export default function CardEditor() {
  const [card, setCard] = useState<PlayingCard>({ rank: "Ace", suit: "Clubs" });
  const [count, setCount] = useState(1);

  const setRank = (rank: Rank) => setCard({ ...card, rank });

  const setSuit = (suit: Suit) => setCard({ ...card, suit });

  return (
    <>
      <div className="w-1/4 h-full pr-8">
        <Header>Rank</Header>
        <div className="grid grid-cols-3 gap-2">
          {ranks.map((rank) => (
            <Button
              key={rank}
              onClick={() => setRank(rank)}
              className={cn("leading-3 px-2 py-1", {
                "border-2 border-solid border-violet-400": rank === card.rank,
              })}
            >
              {rank}
            </Button>
          ))}
        </div>
        <Header>Suit</Header>
        <div className="grid grid-cols-2 gap-2">
          {suits.map((suit) => (
            <Button
              key={suit}
              onClick={() => setSuit(suit)}
              className={cn("leading-3 px-2 py-1", {
                "border-2 border-solid border-violet-400": suit === card.suit,
              })}
            >
              {suit}
            </Button>
          ))}
        </div>
      </div>
      <div className="w-2/4 h-full px-8 border-x border-solid border-slate-500 relative">
        <LayoutGroup>
          <AnimatePresence>
            {range(count).map((index, i) => (
              <motion.div
                whileHover={{ scale: 1.1, y: -50 }}
                initial={{ opacity: 0, y: 40, rotate: "0deg" }}
                animate={{ opacity: 1, y: 0, rotate: rotationClass(rotateConfig[count - 1][i]) }}
                exit={{ opacity: 0, y: 40, rotate: "0deg" }}
                key={index}
                className="h-3/5 w-56 bg-slate-700 absolute rounded-xl border-2 border-violet-300"
                style={{
                  rotate: rotationClass(rotateConfig[count - 1][i]),
                  top: "3rem",
                  left: `${index * 2.5}rem`,
                  zIndex: index * 10,
                }}
              >
                <div className="p-4 h-full rounded-lg flex flex-col justify-between">
                  <div className="w-full p-2 rounded-t-lg">{card.suit}</div>
                  <div className="w-full p-2 rounded-b-lg flex justify-end items-end text-9xl">
                    {displayRank(card.rank)}
                  </div>
                </div>
              </motion.div>
            ))}
          </AnimatePresence>
        </LayoutGroup>
      </div>
      <div className="w-1/4 h-full pl-8">
        <Header>Num Cards</Header>
        <div className="pt-2 grid grid-cols-4 gap-1">
          {range(4).map((i) => (
            <Button
              key={i}
              onClick={() => setCount(i)}
              className={cn("leading-3 px-2 py-1", {
                "border-2 border-solid border-violet-400": i === count,
              })}
            >
              {i}
            </Button>
          ))}
        </div>
      </div>
    </>
  );
}

const Header = ({ children }: PropsWithChildren) => (
  <h4 className="scroll-m-20 mt-4 text-xl font-semibold tracking-tight">{children}</h4>
);
