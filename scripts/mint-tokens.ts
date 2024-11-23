import "dotenv/config";
import { clusterApiUrl, Connection, Keypair, PublicKey } from "@solana/web3.js";
import { mintTo } from "@solana/spl-token";
import { getExplorerLink } from "@solana-developers/helpers";

const sender = Keypair.fromSecretKey(
  new Uint8Array(JSON.parse(process.env["PRIVATE_KEY"] ?? ""))
);
console.log(`Our public key is: ${sender.publicKey.toBase58()}`);

const conn = new Connection(clusterApiUrl("devnet"));

const MINOR_UNITS_PER_MAJOR_UNITS = Math.pow(10, 2);

const tokenMintAccount = new PublicKey("DHNKtiQvD9aGGqyDeyNdEfbARV9TsQn9dJQUyjRh6bHx");
const recipientAssociatedTokenAccount = new PublicKey("FyqrZ1KCk5LUXznHAdTQeqsgHtsd7utyD1zhGe5MhyT6");

const transactionSignature = await mintTo(
  conn,
  sender,
  tokenMintAccount,
  recipientAssociatedTokenAccount,
  sender,
  647 * MINOR_UNITS_PER_MAJOR_UNITS
);
const link = getExplorerLink("transaction", transactionSignature, "devnet");
console.log(`Success! Mint Token Transaction: ${link}`);
