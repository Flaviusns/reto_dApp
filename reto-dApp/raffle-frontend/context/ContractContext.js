import { createContext, useContext } from "react";
import { raffleContract } from "../lib/RaffleContract";

const ContractContext = createContext()

export const ContractProvider = ({ children }) => {

    return (
        <ContractContext.Provider value={raffleContract}>
            {children}
        </ContractContext.Provider>
    )
}

export const useRaffleContract = () => useContext(ContractContext)