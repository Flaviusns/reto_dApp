import nearWallet from "../lib/near-wallet";

const CONTRACT_ID = "dev-1664448823156-51379824713049"

class RaffleContract {
    contractId;
    wallet;

    constructor({ contractId, walletToUse }) {
      this.contractId = contractId;
      this.wallet = walletToUse;    
    }
  
    getRaffleList = async () => {
      return await this.wallet.viewMethod({ contractId: this.contractId, method: 'get_list_raffle' });
    }
  
    createRaffle = async (min_entry_price, min_participants, prize) => {
        const methodArgs = {
            min_entry_price, 
            min_participants, 
            prize
        }
      return await this.wallet.callMethod({ contractId: this.contractId, method: 'create_raffle', args: methodArgs });
    }
}

export const raffleContract = new RaffleContract({contractId: CONTRACT_ID, walletToUse: nearWallet});