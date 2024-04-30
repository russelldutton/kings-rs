export const ranks = [
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
export type Rank = (typeof ranks)[number];

export const suits = ["Hearts", "Diamonds", "Spades", "Clubs"] as const;
export type Suit = (typeof suits)[number];

export type CardProps = {
  rank: Rank;
  suit: Suit;
};

export const displayRank = (rank: Rank) => {
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
