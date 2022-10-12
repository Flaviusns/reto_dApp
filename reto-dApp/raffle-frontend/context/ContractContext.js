import { createContext, useContext } from "react";
import { raffleManagerContract } from "../lib/RaffleManagerContract";

const ContractContext = createContext()

export const ContractProvider = ({ children }) => {

    return (
        <ContractContext.Provider value={raffleManagerContract}>
            {children}
        </ContractContext.Provider>
    )
}

export const useRaffleContract = () => useContext(ContractContext)