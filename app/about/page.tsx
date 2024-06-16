import Head from "next/head";
import styles from "@/app/about/about.module.css";
import Link from "next/link";
import Image from "next/image";

export default function About() {
  return (
    <div className={styles.about}>
      <h2>About Me</h2>
      <p className={styles.aboutParagraph}>Hello my name is Aliviana.</p>
      <p className={styles.aboutParagraph}>I am 6 years old.</p>
      <p className={styles.aboutParagraph}>
        I like to code, eat candy all the time and have play dates with my
        friends.
      </p>
      <p className={styles.aboutParagraph}>
        I made this website with my daddy.
      </p>
      <h2>Pixies & Learning</h2>
      <p className={styles.aboutParagraph}>Pixies are Itty bitty fairies </p>
    </div>
  );
}
