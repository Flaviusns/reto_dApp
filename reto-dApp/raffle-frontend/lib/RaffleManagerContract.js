import nearWallet from "./near-wallet";
import { utils } from 'near-api-js';

const CONTRACT_ID = "subcuenta.flaviusstan.testnet"
const nearRequiredForContractCall = "10" //NEAR
const amountInYocto = utils.format.parseNearAmount(nearRequiredForContractCall);

class RaffleManagerContract {
    contractId;
    wallet;

    constructor({ contractId, walletToUse }) {
      this.contractId = contractId;
      this.wallet = walletToUse;    
    }
  
    getRaffleList = async () => {
      return await this.wallet.viewMethod({ contractId: this.contractId, method: 'get_list_raffle' });
    }
  
    createRaffle = async (methodArgs) => {
      return await this.wallet.callMethod({ contractId: this.contractId, method: 'create_raffle', args: methodArgs, deposit: amountInYocto });
    }
}

export const raffleManagerContract = new RaffleManagerContract({contractId: CONTRACT_ID, walletToUse: nearWallet});