import { PropsWithChildren, useReducer, useState } from "react";
import { Button } from "~/components/ui/button";
import { cn } from "~/lib/utils";
import { AnimatePresence, LayoutGroup } from "framer-motion";
import { CardProps, ranks, suits } from "~/types/card";
import { PlayingCard } from "~/components/playing-card";
import { cardReducer } from "./cards.util";

const range = (target: number) =>
  Array.from(Array(target).keys()).map((i) => i + 1);

const rotateConfig = [[0], [-2, 1], [-4, 0, 4], [-5, 0, 5, 10]];

const initCards = () => {
  const newCards: CardProps[] = range(20)
    .map(() => ({
      suit: suits.at(Math.floor(Math.random() * suits.length - 1))!,
      rank: ranks.at(Math.floor(Math.random() * ranks.length - 1))!,
    }))
    .sort(() => Math.random() - 0.5);

  return newCards;
};

export const CardEditor = () => {
  const [count, setCount] = useState(3);
  const [card, dispatch] = useReducer(cardReducer, {
    rank: "Ace",
    suit: "Clubs",
  });

  const [cards, setCards] = useState<CardProps[]>(initCards);

  const cardsToShow = cards.filter((_, index) => index < count);

  const randomiseCards = () => {
    const newCards = initCards();
    setCards(newCards);
  };

  return (
    <>
      <div className="w-1/4 h-full pr-8">
        <Header>Rank</Header>
        <div className="grid grid-cols-3 gap-2">
          {ranks.map((rank) => (
            <Button
              key={rank}
              onClick={() => dispatch({ type: "rankUpdate", rank })}
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
              onClick={() => dispatch({ type: "suitUpdate", suit })}
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
            {cardsToShow.map((card, i) => (
              <PlayingCard
                key={`${card.rank}-${card.suit}-${i}`}
                card={card}
                order={i + 1}
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
        <div className="pt-2 grid grid-cols-4 gap-1">
          <Button
            onClick={() => randomiseCards()}
            className={cn("leading-3 px-2 py-1 col-span-4")}
          >
            Randomize
          </Button>
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
