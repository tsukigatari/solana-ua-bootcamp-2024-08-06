import "dotenv/config";
import { clusterApiUrl, Connection, Keypair, PublicKey } from "@solana/web3.js";
import { getOrCreateAssociatedTokenAccount } from "@solana/spl-token";
import { getExplorerLink } from "@solana-developers/helpers";

const sender = Keypair.fromSecretKey(
  new Uint8Array(JSON.parse(process.env["PRIVATE_KEY"] ?? ""))
);
console.log(`Our public key is: ${sender.publicKey.toBase58()}`);

const conn = new Connection(clusterApiUrl("devnet"));

const tokenMintAccount = new PublicKey("DHNKtiQvD9aGGqyDeyNdEfbARV9TsQn9dJQUyjRh6bHx");
const recipient = new PublicKey("Cf5yaRgQvjgySgMajURj7uoTfCv4XRs3HocwcYFDDtYA");

const tokenAccount = await getOrCreateAssociatedTokenAccount(
  conn,
  sender,
  tokenMintAccount,
  recipient
);
console.log(`Token Account: ${tokenAccount.address.toBase58()}`)
const link = getExplorerLink("address", tokenAccount.address.toBase58(), "devnet");
console.log(`Created token account: ${link}`);
