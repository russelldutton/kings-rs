import { motion } from "framer-motion";
import { displayRank, CardProps } from "~/types/card";

type PlayingCardProps = {
  card: CardProps;
  index: number;
  rotation: number;
};

export const PlayingCard = ({ card, index, rotation }: PlayingCardProps) => {
  const { rank, suit } = card;
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
      key={`${rank}-${suit}-${index}`}
      className="h-3/5 w-56 bg-slate-700 absolute rounded-xl border-2 border-violet-300"
      style={{
        rotate: rotationClass(rotation),
        top: "3rem",
        left: `${index * 2.5}rem`,
        zIndex: index * 10,
      }}
    >
      <div className="p-4 h-full rounded-lg flex flex-col justify-between">
        <div className="w-full p-2 rounded-t-lg">{suit}</div>
        <div className="w-full p-2 rounded-b-lg flex justify-end items-end text-9xl">
          {displayRank(rank)}
        </div>
      </div>
    </motion.div>
  );
};

const rotationClass = (rotation: number) => {
  return rotation < 0 ? `-${Math.abs(rotation)}deg` : `${rotation}deg`;
};
