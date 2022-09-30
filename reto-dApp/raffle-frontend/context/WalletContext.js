import { createContext, useContext, useEffect, useState } from "react";
import nearWallet from "../lib/near-wallet";

const WalletContext = createContext()

export const WalletProvider = ({ children }) => {
    const [isStartingUp, setisLoading] = useState(true)
    const [isAuthenticated, setisAuthenticated] = useState(false)

    useEffect(() => {
        nearWallet.startUp().then((isSignedIn) => {
            setisAuthenticated(isSignedIn)
            setisLoading(false)
        })
    },[])
    const value = {
        isAuthenticated: isAuthenticated,
        isStartingUp: isStartingUp,
        signIn: () => nearWallet.signIn(),
        signOut: () => nearWallet.signOut(),
    }
    return (
        <WalletContext.Provider value={value}>
            {children}
        </WalletContext.Provider>
    )
}

export const useWallet = () => useContext(WalletContext)