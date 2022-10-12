import { createContext, useContext, useEffect, useState } from "react";
import nearWallet from "../lib/near-wallet";

const WalletContext = createContext()

export const WalletProvider = ({ children }) => {
    const [isStartingUp, setisLoading] = useState(true)
    const [isAuthenticated, setisAuthenticated] = useState(false)
    const [userId, setUserId] = useState("")

    useEffect(() => {
        nearWallet.startUp().then((isSignedIn) => {
            setisAuthenticated(isSignedIn)
            setisLoading(false)
            if (isSignedIn) {
                const userId = nearWallet.getUserId()
                setUserId(userId)
            }
        })
    },[])
    const value = {
        isAuthenticated: isAuthenticated,
        isStartingUp: isStartingUp,
        signIn: () => nearWallet.signIn(),
        signOut: () => nearWallet.signOut(),
        getTransactionResult: nearWallet.getTransactionResult,
        userId: userId,
        wallet: nearWallet,
    }
    return (
        <WalletContext.Provider value={value}>
            {children}
        </WalletContext.Provider>
    )
}

export const useWallet = () => useContext(WalletContext)