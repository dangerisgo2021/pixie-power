import Image from "next/image";
import Head from "next/head";
import Link from "next/link";
import styles from "./page.module.css";
export default function Home() {
  return (
    <div>
      <Head>
        <title>Pixie Power</title>
        <link rel="icon" href="/public/favicon.svg" />
      </Head>

      <main>
        <Image
          className={styles.heroImage}
          src="/hero.webp"
          alt="Hero Image"
          width={1792}
          height={1024}
        />
        <div
          className={`${styles.heroCard} grid grid-cols-2 gap-4 items-center justify-items-center`}
        >
          <div>
            <h1>Welcome to Pixie Power!</h1>
            <h2>Discover the Magic of Learning!</h2>
            <p>
              Join our Pixie Power adventures and make learning fun with
              interactive word searches and more!
            </p>
          </div>
          <Link className={styles.btn} href="/wordsearch">
            Play Word Search
          </Link>
        </div>
      </main>

      <footer>
        <p>&copy; 2024 Pixie Power. All rights reserved.</p>
      </footer>
    </div>
  );
}
