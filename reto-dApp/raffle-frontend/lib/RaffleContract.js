import { utils } from 'near-api-js';

export class RaffleContract {
    contractId;
    wallet;
    min_entry_prize;

    constructor({ contractId, walletToUse, min_entry_prize }) {
      this.contractId = contractId;
      this.wallet = walletToUse;
      this.min_entry_prize = min_entry_prize;
    }
  
    participate = async () => {
      const amountInYocto = utils.format.parseNearAmount(this.min_entry_prize);
      return await this.wallet.callMethod({ contractId: this.contractId, method: 'participate', args: {}, deposit: amountInYocto });
    }
}

// export const raffleContract = new RaffleContract({contractId: CONTRACT_ID, walletToUse: nearWallet});