import { Suit } from "~/types/card";

type SuitIconProps = {
  suit: Suit;
  className: string;
};

const svgContentLookup = {
  Hearts: (
    <g transform="rotate(45,30,30)">
      <rect x="15" y="15" height="35" width="35" />
      <circle cx="15" cy="32.5" r="17.5" />
      <circle cx="32.5" cy="15" r="17.5" />
    </g>
  ),
  Clubs: (
    <>
      <circle cx="18" cy="35" r="14" />
      <circle cx="30" cy="15" r="14" />
      <circle cx="42" cy="35" r="14" />
      <path d="M30,30 Q 30,50 20,60 H40 Q30,50 30,30" />
    </>
  ),
  Diamonds: <rect x="10" y="10" width="40" height="40" transform="rotate(45,30,30)" />,
  Spades: (
    <>
      <g transform="rotate(225,30,30)">
        <rect width="30" height="30" x="20" y="20" />
        <circle cx="20" cy="35" r="15" />
        <circle cx="35" cy="20" r="15" />
      </g>
      <path d="M30,30 Q30,50 20,60 H40 Q30,50 30,30" />
    </>
  ),
} satisfies Record<Suit, JSX.Element>;

export const SuitIcon = ({ suit, className }: SuitIconProps) => {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 60 60" className={className}>
      {svgContentLookup[suit]}
    </svg>
  );
};
