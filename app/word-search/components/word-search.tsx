import styles from "@/app/word-search/word-search.module.css";
import {WordSearchGame} from "@/app/word-search/game/create-word-search";

export const WordSearchRender = ({ wordSearch, width }: { wordSearch: WordSearchGame | undefined, width: number}) => {
    const cells = Object.values(wordSearch?.gridMap ?? {});

    return (
    // @ts-ignore
    <div className={styles.wordSearch} style={{ "--w": width }}>
      {cells.map((cell) => {
        return (
          <div key={cell.id} className={styles.wsCell}>
            {cell.letter}
          </div>
        );
      })}
    </div>
  );
};
