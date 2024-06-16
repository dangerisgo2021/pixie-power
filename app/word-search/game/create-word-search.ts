import {
  directionsMap,
  letters,
  words,
} from "@/app/word-search/game/constants";
import { getOpenDirection } from "@/app/word-search/game/get-open-directions";

type Cell = {
  letter: string;
  open: number;
  row: number;
  col: number;
  id: string;
};
export type WordSearchGame = {
  gridMap: {
    [key: string]: Cell;
  };
  placedWords: string[]
};
function shuffle(array: any[]) {
  array.sort(() => Math.random() - 0.5);
}

const getRandomCharacter = () => {
  return letters.charAt(Math.floor(Math.random() * letters.length));
};

export const createWordSearch = ({
  width,
  height,
}: {
  width: number;
  height: number;
}) => {
  const wordSearch: WordSearchGame = {
    gridMap: {},
    placedWords: []
  };

  console.log("creating grid", { width, height });
  for (let rowIndex = 0; rowIndex < height; rowIndex++) {

    for (let columnIndex = 0; columnIndex < width; columnIndex++) {
      const cellId = `cell_${rowIndex}_${columnIndex}`;
      wordSearch.gridMap[cellId] = {
        open: 0,
        row: rowIndex,
        col: columnIndex,
        id: cellId,
        letter: getRandomCharacter(),
      };
    }
  }

  addWordsToCrossword({ grid: wordSearch, words, width, height });
  return wordSearch;
};

// @ts-ignore
const addWordsToCrossword = ({ grid, words, width, height }) => {
  shuffle(words);
  console.log("placing words", words);
  words.forEach((word: string) => {
    const randomRow = Math.floor(Math.random() * width);
    const randomColumn = Math.floor(Math.random() * height);
    const directionOptions = getOpenDirection({
      word,
      row: randomRow,
      column: randomColumn,
      width,
      height,
    });
    shuffle(directionOptions);
    console.log({ word, randomRow, randomColumn, directionOptions });

    //for each direction until on passes
    dirLoop: for (let i = 0; i < directionOptions.length; i++) {
      // @ts-ignore
      const currentDirection = directionsMap[directionOptions[i]];
      const cell = grid.gridMap[`cell_${randomRow}_${randomColumn}`];
      if (cell.open === 0) {
        let currentRow = randomRow;
        let currentColumn = randomColumn;

        for (let j = 0; j < word.length; j++) {
          const currentLetter = word.charAt(j);
          const currentCell = grid.gridMap[`cell_${currentRow}_${currentColumn}`];
          if (currentCell.open === 0 || currentCell.letter === currentLetter) {
            currentCell.letter = currentLetter;
            //currentCell.className = "full";
            currentRow = currentRow + 1 * currentDirection.row;
            currentColumn = currentColumn + 1 * currentDirection.col;
            currentCell.open += 1;
          } else {
            //backtrack and continue with next direction
            let backtrackRow = currentRow;
            let backtrackColumn = currentColumn;

            for (let x = 0; x < j; x++) {
              backtrackRow = backtrackRow - 1 * currentDirection.row;
              backtrackColumn = backtrackColumn - 1 * currentDirection.col;
              const backtrackCell =
                grid.gridMap[`cell_${backtrackRow}_${backtrackColumn}`];
              const open = backtrackCell.open - 1;
            }
            continue dirLoop;
          }
        }
        grid.placedWords.push(word);
        break;
      }
    }
  });
};
