import { directionsList } from "@/app/word-search/game/constants";

type GetDirectionListParams = {
  word: string;
  row: number;
  column: number;
  width: number;
  height: number;
};
export const getOpenDirection = ({
  word,
  row,
  column,
  width,
  height,
}: GetDirectionListParams) => {
  return directionsList.filter((direction) => {
    //check up fits
    if (direction === "up") {
      return row >= word.length;
    }

    //check down fits
    if (direction === "down") {
      return row + word.length < height;
    }
    //check left fits
    if (direction === "left") {
      return column >= word.length;
    }
    //check right fits
    if (direction === "right") {
      return column + word.length < width;
    }
  });
};
