"use client";
import { WordSearchGrid } from "@/app/word-search/components/word-search";
import { useEffect, useState } from "react";
import {
  Cell,
  createWordSearch,
  WordSearchGame,
} from "@/app/word-search/game/create-word-search";
import { Direction, words } from "@/app/word-search/game/constants";
// setupCounter(document.querySelector('#counter'))

// const width = 10;
// const height = 10;
// const wordsearch = createWordSearch({ width, height });
// @ts-ignore

// addWordsToCrossword({ grid: wsGrid, words, width, height });
// console.log({ wsGrid });
const width = 12;
const height = 12;
export default function WordSearchPage() {
  const [wordSearch, setWordSearch] = useState<undefined | WordSearchGame>(
    undefined,
  );
  const [activeAnswer, setActiveAnswer] = useState<Cell[]>([]);
  const [answerDirection, setAnswerDirection] = useState<Direction | undefined>(
    undefined,
  );

  const [foundWords, setFoundWords] = useState<string[]>([]);
  const [selectedCells, setSelectedCells] = useState<string[]>([]);

  useEffect(() => {
    const newWordSearch = createWordSearch({ width, height });
    setWordSearch(newWordSearch);
  }, []);

  const handleCellClicked = ({ cell }: { cell: Cell }) => {
    console.log({ cell, activeAnswer });
    if (cell.selected) {
      // find index in active answer
      const index = activeAnswer.findIndex(({ id }) => id === cell.id);
      console.log({ index });

      if (index === 0) {
        console.log("empty");
        setActiveAnswer([]);
        activeAnswer.forEach((answerCell, i) => {
          // @ts-ignore
          const gridCell = wordSearch.gridMap[answerCell.id];
          gridCell.selected = false;
        });
      } else {
        activeAnswer.forEach((answerCell, i) => {
          // @ts-ignore
          const gridCell = wordSearch.gridMap[answerCell.id];
          gridCell.selected = i < index;
        });
        // remove all cells in active answers index
        setActiveAnswer(activeAnswer.slice(0, index));
      }
    } else if (activeAnswer?.length === 0) {
      cell.selected = true;
      setActiveAnswer([cell]);
      setSelectedCells([cell.id]);
    } else if (activeAnswer?.length === 1) {
      //check if spot is adjacent to first spot
      const [firstCell] = activeAnswer;

      const rowDiff = cell.row - firstCell.row;
      const colDiff = cell.col - firstCell.col;
      // distance is 1 or less
      if (Math.abs(rowDiff) <= 1 && Math.abs(colDiff) <= 1) {
        cell.selected = true;
        setActiveAnswer([...activeAnswer, cell]);
        setSelectedCells([...selectedCells, cell.id]);
        setAnswerDirection({ row: rowDiff, col: colDiff });
        console.log({ x: { row: rowDiff, col: colDiff } });
      }
    } else {
      const lastCell = activeAnswer.at(-1);

      // check if clicked cell is along the answer direction

      if (
        // @ts-ignore
        cell.col === lastCell.col + 1 * answerDirection.col &&
        // @ts-ignore
        cell.row === lastCell.row + 1 * answerDirection.row
      ) {
        cell.selected = true;
        setActiveAnswer([...activeAnswer, cell]);
        setSelectedCells([...selectedCells, cell.id]);
      }
      // calculate possible answer which are placed words filtered by found words
      // check if active answer plus new letter are in possible answers

      // if it is mark word as found

      // else
    }
  };
  const handleResetClicked = () => {
    const newWordSearch = createWordSearch({ width, height });
    setWordSearch(newWordSearch);
    setFoundWords([])
    setAnswerDirection(undefined)
    setSelectedCells([])
  }
  const hasWordsLeft = wordSearch?.placedWords?.length === foundWords.length;
  const activeText = activeAnswer?.map(({ letter }) => letter).join("");

  useEffect(() => {
    console.log({ activeText });

    const possibleWords = wordSearch?.placedWords.filter((placedWord) => {
      return !foundWords.includes(placedWord);
    });
    console.log({ possibleWords });
    const wordFound = possibleWords?.includes(activeText);
    console.log({ wordFound });
    if (wordFound) {
      setFoundWords([...foundWords, activeText]);
      activeAnswer.forEach((answerCell, i) => {
        // @ts-ignore
        const gridCell = wordSearch.gridMap[answerCell.id];
        gridCell.locked = true;
        gridCell.selected = false;
      });
      setActiveAnswer([]);
    }
  }, [activeAnswer]);
  return (
    <div className="grid justify-items-center self-start">
      <h1>Pixie Word Search</h1>
      <WordSearchGrid
        wordSearch={wordSearch}
        width={width}
        handleCellClicked={handleCellClicked}
      />
      <h2>
        {foundWords.length !== wordSearch?.placedWords.length
          ? `You have found ${foundWords.length} out of ${wordSearch?.placedWords?.length} `
            : <button className="rounded-full bg-green-600" onClick={handleResetClicked}>You Won! Click to play again</button> }
      </h2>
      {activeText && <h2>{activeText}</h2>}
      <ul className="grid grid-cols-3 gap-5">
        {wordSearch?.placedWords.map((word) => {
          const found = foundWords.includes(word);
          return (
            <li key={word} className={`${found ? "italic line-through" : ""}`}>
              {word}
            </li>
          );
        })}
      </ul>
    </div>
  );
}
