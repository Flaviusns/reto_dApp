import Head from 'next/head'
import Image from 'next/image'
import { useEffect, useState } from 'react'
import { useRaffleContract } from '../context/ContractContext'
import { useWallet } from '../context/WalletContext'
import styles from '../styles/Home.module.css'

export default function Home() {
    const { isStartingUp, isAuthenticated, signIn, signOut, userId } = useWallet()
    const { getRaffleList, createRaffle } = useRaffleContract()

    useEffect(() => {
        if (!isStartingUp) {
            getRaffleList().then((data) => {
                console.log("data",data)
            })
        }
    },[isStartingUp])

    const handleLogin = () => {
        signIn()
    }

    const handleSignout = () => {
        signOut()
    }

  return (
    <div className={styles.container}>
      <Head>
        <title>Create Next App</title>
        <meta name="description" content="Generated by create next app" />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main className={styles.main}>
        <h1 className={styles.title}>
          Welcome to <a>dRaffleApp!</a>
        </h1>

        {
            isStartingUp ? (
                <p className={styles.description} >
                    Loading ...
                </p>
            ) : !isAuthenticated ? (
                <button onClick={handleLogin}>
                    <p className={styles.description} >
                        Wallet Login
                    </p>
                </button>
            ) : (
                <p className={styles.description} >
                    Hola tu accountId es: {userId}
                </p>
            )
        }
        {
            !isStartingUp && isAuthenticated && (
                <button onClick={handleSignout}>
                    Sign out
                </button>
            )
        }
      </main>

      <footer className={styles.footer}>
        <a
          href="https://vercel.com?utm_source=create-next-app&utm_medium=default-template&utm_campaign=create-next-app"
          target="_blank"
          rel="noopener noreferrer"
        >
          Powered by{' '}
          <span className={styles.logo}>
            <Image src="/vercel.svg" alt="Vercel Logo" width={72} height={16} />
          </span>
        </a>
      </footer>
    </div>
  )
}