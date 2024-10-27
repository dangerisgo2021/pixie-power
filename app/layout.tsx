import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";
import styles from "@/app/page.module.css";
import Link from "next/link";

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "Pixie",
  description: "Pixies + Learning",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={inter.className}>
        <header className={styles.header}>
          <nav>
            <ul className="flex">
              <li className="mr-6">
                <Link href="/">Home</Link>
              </li>
              <li className="mr-6">
                <Link href="/word-search">Word Search</Link>
              </li>
              <li className="mr-6">
                <Link href="/snake">Snake</Link>
              </li>
              <li className="mr-6">
                <Link href="/about">About</Link>
              </li>
            </ul>
          </nav>
        </header>
        {children}
        <footer>
          <p>&copy; 2024 Pixie Power. All rights reserved.</p>
        </footer>
      </body>
    </html>
  );
}
