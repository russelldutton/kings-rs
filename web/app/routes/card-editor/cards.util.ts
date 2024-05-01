import { CardProps, Rank, Suit } from "~/types/card";

type CardReducerAction =
  | {
      type: "rankUpdate";
      rank: Rank;
    }
  | {
      type: "suitUpdate";
      suit: Suit;
    };

export const cardReducer = (state: CardProps, action: CardReducerAction) => {
  switch (action.type) {
    case "rankUpdate":
      return { ...state, rank: action.rank };
    case "suitUpdate":
      return { ...state, suit: action.suit };
  }
};
