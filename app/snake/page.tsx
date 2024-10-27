"use client";

import styles from "@/app/snake/snake.module.css";
import useWasm from "@/app/snake/use-wasm";

export default function Snake() {
  const snakeGame = useWasm("/out/rust.js");
  console.log({ snakeGame });
  useEffect(() => {
    console.log("hook");

    if (snakeGame) {
      console.log("initSync");

      snakeGame.initSync();
    }
  }, [snakeGame]);
  return (
    <div className={styles.snake}>
      <h2>Snake Game</h2>
      <p className={styles.text}>Arrows - WASD - On screen controls</p>
    </div>
  );
}
