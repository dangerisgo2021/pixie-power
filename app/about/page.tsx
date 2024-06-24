import Head from "next/head";
import styles from "@/app/about/about.module.css";
import Link from "next/link";
import Image from "next/image";

export default function About() {
  return (
    <div className={styles.about}>
      <h2>About Me</h2>
      <p className={styles.text}>Hello my name is Aliviana.</p>
      <p className={styles.text}>I am 6 years old.</p>
      <p className={styles.text}>
        I like to code, eat candy all the time and have play dates with my
        friends.
      </p>
      <p className={styles.text}>I made this website with my daddy.</p>
      <h2>Pixies & Learning</h2>
      <p className={styles.text}>Pixies are Itty bitty fairies </p>
      <h2>Bonus Fairy Story</h2>
      <p className={styles.text}>(Introduce characters)Two young pixies named Lila and Lily</p>
      <p className={styles.text}>(Where does it take place)At the park</p>
      <p className={styles.text}>(What do the characters want to )They want to go on the pink tunnel twisty slide</p>
      <p className={styles.text}>(What is stopping them)The boy fairy line for the orange slide</p>
      <p className={styles.text}>(How do they do it)</p>
      <p className={styles.text}>(What lesson did they learn)</p>
    </div>
  );
}
