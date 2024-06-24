import styles from "@/app/word-search/word-search.module.css";
import {
  Cell,
  WordSearchGame,
} from "@/app/word-search/game/create-word-search";

export const WordSearchGrid = ({
  wordSearch,
  width,
  handleCellClicked,
}: {
  wordSearch: WordSearchGame | undefined;
  width: number;
  handleCellClicked: ({ cell }: { cell: Cell }) => void;
}) => {
  const cells = Object.values(wordSearch?.gridMap ?? {});
  return (
    // @ts-ignore
    <div className={styles.wordSearchGrid} style={{ "--w": width }}>
      {cells.map((cell) => {
        return (
          <div
            key={cell.id}
            className={`${styles.wsCell} ${cell.selected ? styles.selectedCell : ""} ${cell.locked ? "text-amber-600" : ""}`}
            onClick={() => handleCellClicked({ cell })}
          >
            {cell.letter}
          </div>
        );
      })}
    </div>
  );
};
