import Head from "next/head";
import { useEffect, useState } from "react";
import { useRaffleContract } from "../context/ContractContext";
import { useWallet } from "../context/WalletContext";
import styles from "../styles/Home.module.css";

export default function Home() {
  const { isStartingUp, isAuthenticated, signIn, signOut, userId } =
    useWallet();
  const { getRaffleList, createRaffle } = useRaffleContract();
  const [raffleList, setRaffleList] = useState({});
  const [formData, setFormData] = useState({
    min_entry_price: 1,
    min_participants: 1,
    prize: "test_hash",
    description: "test_hash",
    nft_account: "test_hash",
    open_days: 1,
  });

  useEffect(() => {
    if (!isStartingUp) {
      getRaffleList()
        .then((data) => {
          setRaffleList(data);
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
                Log out
              </button>
              <p className={styles.description}>Welcome {userId}</p>
              <form id={styles.createRaffleForm}>
                <leyend className={styles.subtitle}>
                  Create your Charitaffle
                </leyend>
                <div>
                  <label>
                    min_entry_price:
                    <input
                      type="number"
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
                    min_participants:
                    <input
                      type="number"
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
                    prize (nft hash):
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
                    description:
                    <input
                      type="text"
                      placeholder="description"
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
                    nft_account:
                    <input
                      type="text"
                      placeholder="nft_account"
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
                    open_days:
                    <input
                      type="number"
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

              <div id={styles.raffleList}>
                <h2 className={styles.subtitle}>Charitaffles in progress</h2>
                <p>
                  Find here the cause or prize you want to participate for and
                  buy yout ticket.
                </p>
                <ul>
                  <li>Raffle Name 1</li>
                  <li>Raffle Name 2</li>
                  {/*raffleList.map((name) => <li>{name}</li>)*/}
                </ul>
              </div>
            </>
          )
        )}
      </main>

      <footer className={styles.footer}>
        <p>
          Powered by the<a
            href="https://platzi.com/blog/animate-a-aprender-de-desarrollo-blockchain-y-aplica-a-una-de-las-100-becas-con-near-hispano/"
            target="_blank"
          >NEAR Developer Program</a> of Platzi
          
          </p>
          <p>
          More about this project on
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
