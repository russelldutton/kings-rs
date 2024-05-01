import { motion } from "framer-motion";
import { displayRank, CardProps } from "~/types/card";
import { SuitIcon } from "./SuitIcon";
import { cn } from "~/lib/utils";

type PlayingCardProps = {
  card: CardProps;
  index: number;
  rotation: number;
};

export const PlayingCard = ({ card, index, rotation }: PlayingCardProps) => {
  const { rank, suit } = card;
  const isBlackCard = suit === "Clubs" || suit === "Spades";

  const textColour = isBlackCard
    ? "text-slate-800 dark:text-slate-200"
    : "text-red-600 dark:text-red-400";
  const fillColour = isBlackCard
    ? "fill-slate-800 dark:fill-slate-200"
    : "fill-red-600 dark:fill-red-400";
  const borderColour = isBlackCard
    ? "border-slate-800 dark:border-slate-200"
    : "border-red-600 dark:border-red-400";
  return (
    <motion.div
      whileHover={{ scale: 1.1, y: -25 }}
      initial={{ opacity: 0, y: 40, rotate: "0deg" }}
      animate={{
        opacity: 1,
        y: 0,
        rotate: rotationClass(rotation),
      }}
      exit={{ opacity: 0, y: 40, rotate: "0deg" }}
      className={cn(
        "h-3/5 w-56 bg-slate-200 dark:bg-slate-700 absolute rounded-xl border-2",
        borderColour
      )}
      style={{
        rotate: rotationClass(rotation),
        top: "3rem",
        left: `${index * 2.5}rem`,
        zIndex: index * 10,
      }}
    >
      <div className="p-4 h-full rounded-lg flex flex-col justify-between font-libre">
        <div className={textColour}>
          <div className="pl-1.5 text-2xl font-bold">{displayRank(rank)}</div>
          <SuitIcon suit={suit} className={`pl-1 h-5 ${fillColour}`} />
        </div>
        <div className={`flex justify-end items-end bottom-0 text-8xl text-center ${textColour}`}>
          {displayRank(rank)}
        </div>
      </div>
    </motion.div>
  );
};

const rotationClass = (rotation: number) => {
  return rotation < 0 ? `-${Math.abs(rotation)}deg` : `${rotation}deg`;
};
