![](https://i.imgur.com/7kz7VH2.jpg)

# Charitaffle || NFT Raffle dApp for Charitable Causes

Charitaffle is a decentralized platform that allows you to transparently and securely organize fundraising for a charitable cause through the NFTs prize draw.

It is very common to find people or organizations that seek to raise funds for charitable causes through raffles with physical prizes, however these systems are not entirely transparent when it comes to establishing a winner and can be easily manipulated.

With Charitaffle we offer the possibility of organizing a raffle tat exist in a smart contract within the Blockchain of the NEAR protocol, ensuring total transparency and security of the participation, award process and the transfer of the prize to its respective winner. As well as the possibility of rewarding with NFTs, which support digital assets as well as unique representation of physical assets.

![](https://i.imgur.com/ECitB7u.jpg)

## Smart Contracts

Charitaffle uses two contract models. A contract that is created for each raffle and an administrator contract for all raffle contracts management.

### Raffle Manager Smart Contract

This contract is in charge of creating all the raffle contracts and also of view their respective states.

### Raffle Smart Contract

These contracts are created as [Lock Contract](https://docs.near.org/develop/upgrade-and-lock#locking-a-contract). It means they can manage themselves once they receive the NFT that is stablished as a prize.

This is the smart contract that is in charge of managing user participation in this own raffle, choosing a random winner, transferring the NFT to the winner or returning it to the raffle organizer in case the minimum number of participants is not met and transferring the entire amount collected in the raffle to the organizer once it finished.

This contract requires funds to operate all these functions through transactions. It's for this reason that in order to provide enough funds for operation, as well as to establish a liability barrier for those who use this platform, it is required to make a payment of 10 NEAR when creating a new raffle.

## Initialization

This app was initialized with [create-near-app](https://github.com/near/create-near-app)

### Quick start

If you haven't installed dependencies:

    npm install

The Smart Contract was made in Rust. So add the target to build using:

    rustup target add wasm32-unknown-unknown

Build and deploy your contract to TestNet with a temporary dev account:

    npm run deploy

To compile the Smart Contract and get the `.wasm` use:

    cargo build --target wasm32-unknown-unknown --release

The FrontEnd was made in Next. To load it in local server, go to raffle-frontend folder and use:

    npm run dev

### Exploring The Code

1. The smart-contract code lives in the `/contract/raffle_smart_contract` folder. See the README there for
   more info. In blockchain apps the smart contract is the "backend" of your app and this one was made in Rust.
2. The frontend code lives in the `/raffle-frontend` folder. `/raffle-frontend/pages/index.js` is your entrypoint to learn how the frontend connects to the NEAR blockchain.

## About NEAR Developer Program 2022 of Platzi

The [Blockchain Development Program with NEAR](https://platzi.com/cursos/near-program/) is an initiative by the Spanish-speaking community of NEAR Protocol (NEAR Hispano), under the support of the NEAR Foundation for the development of skills around the creation of applications and platforms that make use of the NEAR Protocol blockchain technology.

## Team Nº4 members

Those responsible for the design and development of the Charitaffle dApp were [Flavius Stan](https://github.com/Flaviusns), [Joaquín Ramos](https://github.com/JoarDev) and [Felipe Huerta](https://github.com/Hueeerta).

![](https://i.imgur.com/1WTMBjC.jpg)
