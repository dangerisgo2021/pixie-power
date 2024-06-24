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
    if (direction === "up") {
      return row >= word.length;
    }

    if (direction === "down") {
      return row + word.length < height;
    }

    if (direction === "left") {
      return column >= word.length;
    }

    if (direction === "right") {
      return column + word.length < width;
    }

    if (direction === "downRight") {
      return row + word.length < height && column + word.length < width;
    }

    if (direction === "upRight") {
      return row >= word.length && column + word.length < width;
    }

    if (direction === "upLeft") {
      return row >= word.length && column >= word.length;
    }

    if (direction === "downLeft") {
      return row + word.length < height && column + word.length < width;
    }
  });
};
