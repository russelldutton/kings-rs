import { PropsWithChildren, useState } from "react";
import { Button } from "~/components/ui/button";
import { cn } from "~/lib/utils";
import { AnimatePresence, LayoutGroup } from "framer-motion";
import { type CardProps, Rank, Suit, ranks, suits } from "~/types/card";
import { PlayingCard } from "~/components/playing-card";

const range = (target: number) =>
  Array.from(Array(target).keys()).map((i) => i + 1);

const rotationClass = (rotation: number) => {
  return rotation < 0 ? `-${Math.abs(rotation)}deg` : `${rotation}deg`;
};

const rotateConfig = [[0], [-2, 1], [-4, 0, 4], [-5, 0, 5, 10]];

export const CardEditor = () => {
  const [card, setCard] = useState<CardProps>({ rank: "Ace", suit: "Clubs" });
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
              <PlayingCard
                card={card}
                index={index}
                rotation={rotateConfig[count - 1][i]}
              />
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
};

const Header = ({ children }: PropsWithChildren) => (
  <h4 className="scroll-m-20 mt-4 text-xl font-semibold tracking-tight">
    {children}
  </h4>
);

export default CardEditor;
