# Solana_Bootcamp-Final_Project

# My Digital Art NFT Contract

This is a custom Non-Fungible Token (NFT) contract on the Solana blockchain that represents unique pieces of digital art. Users can mint, transfer, and burn these NFTs. This README provides an overview of the contract's purpose and instructions on how to deploy and interact with it.

## Purpose

The purpose of this NFT contract is to tokenize digital art and enable collectors to own, trade, and showcase these unique pieces of art as NFTs. Each NFT has metadata, including a title, artist, description, and an image URL that points to the artwork's location.

## Functionalities

1. **Minting**: Users can mint new NFTs by providing the required metadata, including title, artist, description, and image URL. Each minted NFT is a unique piece of digital art.

2. **Transferring**: NFT owners can transfer their NFTs to other users. This functionality allows the ownership and trading of NFTs.

3. **Burning**: Owners have the option to burn (destroy) their NFTs, removing them from circulation.

## Deployment

To deploy and interact with the contract, follow these steps:

1. **Install Solana CLI**: Ensure you have the Solana Command Line Interface (CLI) installed. You can install it by following the instructions [here](https://docs.solana.com/cli/install).

2. **Clone the Repository**: Clone this repository to your local environment.

   ```bash
   git clone https://github.com/your-repo/your-nft-contract.git
Navigate to the Project Directory:

bash
Copy code
cd your-nft-contract
Compile the Program:

bash
Copy code
cargo build-bpf
Deploy the Program: Deploy the compiled program to the Solana blockchain. This will give you a program ID that you will use to interact with the contract.

bash
Copy code
solana program deploy target/deploy/your_nft_program.so
Interact with the Contract: You can interact with the contract using the Solana CLI or by building a custom front-end for users to mint, transfer, and burn NFTs.

Usage
Minting NFTs
To mint a new NFT, use the mint method with the required metadata:

bash
Copy code
solana call <Your Program ID> mint \
    --input <Mint Account> \
    --input <Your NFT Account (to receive the NFT)> \
    --input <Your Wallet (for authorization)> \
    --input <Title> \
    --input <Artist> \
    --input <Description> \
    --input <Image URL>
Transferring NFTs
To transfer an NFT, use the transfer method with the NFT account and the recipient's account:

bash
Copy code
solana call <Your Program ID> transfer \
    --input <Mint Account> \
    --input <Your NFT Account (to transfer)> \
    --input <Recipient Account> \
    --input <Your Wallet (for authorization)>
Burning NFTs
To burn (destroy) an NFT, use the burn method with the NFT account:

bash
Copy code
solana call <Your Program ID> burn \
    --input <Mint Account> \
    --input <Your NFT Account (to burn)> \
    --input <Your Wallet (for authorization)>
