import React from "react";
import { Suit } from "~/types/card";

interface SuitIconProps extends React.SVGProps<SVGSVGElement> {
  suit: Suit;
}

const svgContentLookup = {
  Hearts: (
    <path
      transform="rotate(180, 50, 50)"
      d="M50,10 A250,250,-45,0,1,20,40 A1,1,-45,0,0,50,75 A1,1,-45,0,0,80,40 A250,250,-45,0,1,50,10"
      // d="M50,25 A1,1.2,-30,0,0,20,60 L50,90 L80,60 A1,1.2,30,0,0,50,25"
    ></path>
  ),
  Clubs: (
    <path
      d="
          M 33,40
          A 21 21, 0, 1, 0 50,70
          Q 50,80 40,90 H60 Q50,80 50,70
          A 21 21, 0, 1, 0 68,40
          A 21 21, 0, 1, 0 33,40"
    />
  ),
  Diamonds: (
    <path d="M50,10 A200,200,-45,0,1,15,50 A200,200,-45,0,1,50,90 A200,200,-45,0,1,85,50 A200,200,-45,0,1,50,10"></path>
  ),
  Spades: (
    <path d="M50,10 A250,250,-45,0,1,20,40 A1,1,-45,0,0,49,70 A50,50,-45,0,1,40,90 L60,90 A50,50,-45,0,1,51,70 A1,1,-45,0,0,80,40 A250,250,-45,0,1,50,10"></path>
  ),
} satisfies Record<Suit, JSX.Element>;

export const SuitIcon = ({ suit, ...props }: SuitIconProps) => {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100" {...props}>
      {svgContentLookup[suit]}
    </svg>
  );
};
