"use client";

import styles from "@/app/snake/snake.module.css";
import { useEffect } from "react";
import initGame from "@/public/out/rust";
export default function Snake() {
  //const snakeGame = useWasm("/out/rust.js");
  //console.log({ snakeGame });
  useEffect(() => {
    console.log("hook", initGame);
    async function startGame() {
      console.log({ initGame });
      try {
        const y = await initGame();
        console.log({ y });
      } catch (err) {
        console.log("err", err);
      }
    }
    console.log("starting game");
    startGame();
    console.log("started comaplted");
  }, [initGame]);
  return (
    <div className={styles.snake}>
      <h2>Snake Game</h2>
      <p className={styles.text}>Arrows - WASD - On screen controls</p>
      <canvas id="snake-canvas" width="400" height="600" />
    </div>
  );
}
