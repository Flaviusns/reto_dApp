import { ContractProvider } from '../context/ContractContext'
import { WalletProvider } from '../context/WalletContext'
import '../styles/globals.css'

function MyApp({ Component, pageProps }) {
    return (
        <WalletProvider>
            <ContractProvider>
                <Component {...pageProps} />
            </ContractProvider>
        </WalletProvider>
    )
}

export default MyApp
