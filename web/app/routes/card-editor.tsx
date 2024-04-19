import { useCallback, useState } from "react";
import { Button } from "~/components/ui/button";
import { Card } from "~/components/ui/card";

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

export default function CardEditor() {
  const [card, setCard] = useState<PlayingCard>({ rank: "Ace", suit: "Clubs" });

  const setRank = (rank: Rank) => setCard({ ...card, rank });

  const setSuit = (suit: Suit) => setCard({ ...card, suit });

  const displayRank = useCallback((rank: Rank) => {
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
  }, []);

  return (
    <>
      <div className="w-1/4 h-full pr-8">
        <h4 className="scroll-m-20 text-xl font-semibold tracking-tight">
          Rank
        </h4>
        <div className="grid grid-cols-3 gap-2">
          {ranks.map((rank) => (
            <Button
              key={rank}
              onClick={() => setRank(rank)}
              className="leading-3 px-2 py-1"
            >
              {rank}
            </Button>
          ))}
        </div>
        <h4 className="scroll-m-20 mt-4 text-xl font-semibold tracking-tight">
          Suit
        </h4>
        <div className="grid grid-cols-2 gap-2">
          {suits.map((suit) => (
            <Button key={suit} onClick={() => setSuit(suit)}>
              {suit}
            </Button>
          ))}
        </div>
      </div>
      <div className="w-3/4 h-full pl-8 border-l border-solid border-slate-500">
        <Card className="h-3/5 w-56 bg-slate-700">
          <div className="p-4 h-full rounded-lg">
            <div className="w-full h-1/5 p-2 rounded-t-lg">{card.suit}</div>
            <div className="w-full h-4/5 p-2 rounded-b-lg flex justify-end items-end text-9xl font-serif">
              {displayRank(card.rank)}
            </div>
          </div>
        </Card>
      </div>
    </>
  );
}
