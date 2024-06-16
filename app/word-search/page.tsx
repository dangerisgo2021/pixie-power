"use client";
import { WordSearchRender } from "@/app/word-search/components/word-search";
import { useEffect, useState } from "react";
import {
  createWordSearch,
  WordSearchGame,
} from "@/app/word-search/game/create-word-search";
// setupCounter(document.querySelector('#counter'))

// const width = 10;
// const height = 10;
// const wordsearch = createWordSearch({ width, height });
// @ts-ignore

// addWordsToCrossword({ grid: wsGrid, words, width, height });
// console.log({ wsGrid });
const width = 12;
const height = 12;
export default function WordSearch() {
  const [wordSearch, setWordSearch] = useState<undefined | WordSearchGame>(
    undefined,
  );

  useEffect(() => {
    const newWordSearch = createWordSearch({ width, height });
    setWordSearch(newWordSearch);
  }, []);

    return (
    <div className="grid justify-items-center self-start">
      <WordSearchRender wordSearch={wordSearch} width={width} />
      <ul className="grid grid-cols-3 gap-5">
        {wordSearch?.placedWords.map((word) => {
          return <li key={word}>{word}</li>;
        })}
      </ul>
    </div>
  );
}
