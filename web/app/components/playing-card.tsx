import { motion } from "framer-motion";
import { displayRank, CardProps } from "~/types/card";
import { SuitIcon } from "./SuitIcon";
import { cn } from "~/lib/utils";

type PlayingCardProps = {
  card: CardProps;
  order: number;
  rotation: number;
};

export const PlayingCard = ({ card, order, rotation }: PlayingCardProps) => {
  const { rank, suit } = card;
  const isBlackCard = suit === "Clubs" || suit === "Spades";

  const textColour = isBlackCard
    ? "text-slate-800 dark:text-slate-200"
    : "text-red-600 dark:text-red-400";
  const fillColour = isBlackCard
    ? "fill-slate-800 dark:fill-slate-200"
    : "fill-red-600 dark:fill-red-400";
  const strokeColour = isBlackCard
    ? "stroke-slate-800 dark:stroke-slate-200"
    : "stroke-red-600 dark:stroke-red-400";
  const borderColour = "border-slate-300 dark:border-slate-600";

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
        left: `${order * 2.5}rem`,
        zIndex: order * 10,
      }}
    >
      <div className="p-4 h-full rounded-lg flex flex-col justify-between font-libre overflow-hidden relative z-10">
        <div className="absolute h-auto w-[170%] -bottom-16 -right-28 z-10">
          <SuitIcon
            suit={suit}
            className={`${strokeColour}`}
            fill="none"
            strokeOpacity={0.3}
          />
        </div>
        <div className={`${textColour} z-50`}>
          <div className="w-7 text-2xl font-bold text-center tracking-tighter">
            {displayRank(rank)}
          </div>
          <div className="w-7 flex justify-center">
            <SuitIcon suit={suit} className={`h-5  ${fillColour}`} />
          </div>
        </div>
        <div
          className={`mb-8 flex justify-end items-end bottom-0 text-8xl text-center z-50 tracking-tighter ${textColour}`}
        >
          {displayRank(rank)}
        </div>
      </div>
    </motion.div>
  );
};

const rotationClass = (rotation: number) => {
  return rotation < 0 ? `-${Math.abs(rotation)}deg` : `${rotation}deg`;
};
