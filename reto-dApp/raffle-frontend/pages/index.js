import Head from "next/head";
import { useEffect, useState } from "react";
import { useRaffleContract } from "../context/ContractContext";
import { useWallet } from "../context/WalletContext";
import { RaffleContract } from "../lib/RaffleContract";
import styles from "../styles/Home.module.css";
import { useRouter } from "next/router";

export default function Home() {
  const router = useRouter();
  const {
    isStartingUp,
    isAuthenticated,
    signIn,
    signOut,
    userId,
    wallet,
    getTransactionResult,
  } = useWallet();
  const { getRaffleList, createRaffle } = useRaffleContract();
  const [transactionResult, setTransactionResult] = useState(false);
  const [raffleList, setRaffleList] = useState([]);
  const [formData, setFormData] = useState({
    min_entry_price: 1,
    min_participants: 1,
    prize: "test_hash",
    description: "test_hash",
    nft_account: "test_hash",
    open_days: 1,
  });

  useEffect(() => {
    const { query } = router;
    if (query.transactionHashes) {
      getTransactionResult(query.transactionHashes)
        .then((value) => setTransactionResult({ value: value }))
        .catch((error) => console.error("error", error));
    }
  }, []);

  useEffect(() => {
    if (!isStartingUp) {
      getRaffleList()
        .then((data) => {
          const raffleData = data.map(([hash, item]) => item);
          setRaffleList(raffleData);
        })
        .catch((error) => {
          console.error(error);
        });
    }
  }, [isStartingUp]);

  const handleLogin = () => {
    signIn();
  };

  const handleSignout = () => {
    signOut();
  };

  const handleCreateRaffle = (e) => {
    e.preventDefault();
    const createRaffleArgs = {
      description: "test raffle",
      min_entry_price: formData.min_entry_price,
      min_participants: formData.min_participants,
      prize: formData.prize,
      nft_account: "",
      open_days: 1,
    };
    createRaffle(createRaffleArgs)
      .then((data) => {
        console.log(data);
      })
      .catch((error) => {
        console.error(error);
      });
  };

  const handleParticipate = (contractId, min_entry_prize) => {
    const raffleContract = new RaffleContract({
      contractId: contractId,
      walletToUse: wallet,
      min_entry_prize: min_entry_prize.toString(),
    });
    return raffleContract.participate();
  };

  return (
    <div className={styles.container}>
      <Head>
        <title>Charitaffle || NFT raffle dApp for charity</title>
        <meta
          name="description"
          content="Give away NFTs and raise the funds you need for your cause in a transparent and decentralized way. This dApp was developed by the Team Nº4 of the NEAR Developer Program 2022 of Platzi."
        />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main className={styles.main}>
        {isStartingUp ? (
          <p className={styles.description}>Loading ...</p>
        ) : !isAuthenticated ? (
          <>
            <h1 className={styles.title}>Charitaffle</h1>
            <p className={styles.slogan}>NFT raffle dApp for charity</p>
            <p className={styles.description}>
              Give away NFTs and raise the funds you need for your cause in a
              transparent and decentralized way.
            </p>
            <button onClick={handleLogin}>Login with your Wallet</button>
          </>
        ) : (
          !isStartingUp &&
          isAuthenticated && (
            <>
              <div className={styles.logo}>Charitaffle</div>
              <button onClick={handleSignout} className={styles.logout}>
                Log out {userId}
              </button>

              <div id={styles.createRaffleForm}>
                <form>
                  <h2 className={styles.subtitle}>
                    Create your own Charitaffle
                  </h2>
                  <div>
                    <label>
                      <span>
                        Description:
                        <div className={styles.tooltip}>
                          i
                          <span className={styles.tiptext}>
                            Brief description of your cause and/or the prize you
                            are raffling off.
                          </span>
                        </div>
                      </span>
                      <input
                        type="text"
                        placeholder="Raffle's description"
                        value={formData.description}
                        onChange={(e) =>
                          setFormData((prev) => ({
                            ...prev,
                            description: e.target.value,
                          }))
                        }
                      />
                    </label>
                  </div>
                  <div>
                    <label>
                      <span>
                        Minimum entry price:
                        <div className={styles.tooltip}>
                          i
                          <span className={styles.tiptext}>
                            How much is the minimun price that each participant
                            must pay for one ticket.
                          </span>
                        </div>
                      </span>
                      <input
                        type="number"
                        placeholder="1 NEAR"
                        value={formData.min_entry_price}
                        onChange={(e) =>
                          setFormData((prev) => ({
                            ...prev,
                            min_entry_price: e.target.value,
                          }))
                        }
                      />
                    </label>
                  </div>
                  <div>
                    <label>
                      <span>
                        Minimum participants:
                        <div className={styles.tooltip}>
                          i
                          <span className={styles.tiptext}>
                            How many participant must buy a ticket in order to
                            give away the NFT prize.
                          </span>
                        </div>
                      </span>
                      <input
                        type="number"
                        placeholder="10"
                        value={formData.min_participants}
                        onChange={(e) =>
                          setFormData((prev) => ({
                            ...prev,
                            min_participants: e.target.value,
                          }))
                        }
                      />
                    </label>
                  </div>
                  <div>
                    <label>
                      <span>
                        NFT Prize:
                        <div className={styles.tooltip}>
                          i
                          <span className={styles.tiptext}>
                            Enter here the NFT's hash that you're givin away.
                          </span>
                        </div>
                      </span>
                      <input
                        type="text"
                        placeholder="NFT hash"
                        value={formData.prize}
                        onChange={(e) =>
                          setFormData((prev) => ({
                            ...prev,
                            prize: e.target.value,
                          }))
                        }
                      />
                    </label>
                  </div>
                  <div>
                    <label>
                      <span>
                        NFT's owner account:
                        <div className={styles.tooltip}>
                          i
                          <span className={styles.tiptext}>
                            Enter here the account that owns the NFT.
                          </span>
                        </div>
                      </span>
                      <input
                        type="text"
                        placeholder="nftOwner.tesnet"
                        value={formData.nft_account}
                        onChange={(e) =>
                          setFormData((prev) => ({
                            ...prev,
                            nft_account: e.target.value,
                          }))
                        }
                      />
                    </label>
                  </div>
                  <div>
                    <label>
                      <span>
                        Open days:
                        <div className={styles.tooltip}>
                          i
                          <span className={styles.tiptext}>
                            Enter here how many days your raffle will be open
                            for participation.
                          </span>
                        </div>
                      </span>
                      <input
                        type="number"
                        placeholder="30 days"
                        value={formData.open_days}
                        onChange={(e) =>
                          setFormData((prev) => ({
                            ...prev,
                            open_days: e.target.value,
                          }))
                        }
                      />
                    </label>
                  </div>
                  <button onClick={handleCreateRaffle}>
                    Launch my Charitaffle
                  </button>
                </form>
              </div>

              <div id={styles.raffleList}>
                <h2 className={styles.subtitle}>Explore active Charitaffles</h2>
                <p>
                  Search for the cause/prize you want to participate for and buy
                  yout ticket.
                </p>
                <ul>
                  {raffleList.map((item, i) => (
                    <li key={i}>
                      <div>{item.description}</div>
                      <div>
                        <span>Minimum of participants:</span>
                        {item.min_participants}
                      </div>
                      <button
                        onClick={() =>
                          handleParticipate(item.account, item.min_entry_price)
                        }
                      >
                        <span>Buy</span>
                        <span>Ticket</span>
                      </button>
                    </li>
                  ))}
                  {/* List item template
                  <li>
                    <div>Raffle Description</div>
                    <div>
                      <span>Total participant:</span>10
                    </div>
                    <button>
                      <span>Buy</span>
                      <span>Ticket</span>
                    </button>
                  </li>
                  */}
                </ul>
              </div>
              {/*
              <div className={styles.code}>
                {
                  JSON.stringify(raffleList)
                }
              </div>
              */}
            </>
          )
        )}
      </main>

      <footer className={styles.footer}>
        <p>
          Powered by the
          <a
            href="https://platzi.com/blog/animate-a-aprender-de-desarrollo-blockchain-y-aplica-a-una-de-las-100-becas-con-near-hispano/"
            target="_blank"
          >
            NEAR Developer Program
          </a>{" "}
          of Platzi
        </p>
        <p>
          More info about this project on
          <a
            className={styles.github}
            href="https://github.com/Flaviusns/reto_dApp"
            target="_blank"
          >
            Github
          </a>
        </p>
      </footer>
    </div>
  );
}
