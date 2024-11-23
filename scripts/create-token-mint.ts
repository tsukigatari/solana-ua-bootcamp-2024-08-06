import "dotenv/config";
import { getExplorerLink } from "@solana-developers/helpers";
import { createMint } from "@solana/spl-token";
import { clusterApiUrl, Connection, Keypair } from "@solana/web3.js";

const sender = Keypair.fromSecretKey(
  new Uint8Array(JSON.parse(process.env["PRIVATE_KEY"] ?? "")),
);
console.log(`Our public key is: ${sender.publicKey.toBase58()}`);

const conn = new Connection(clusterApiUrl("devnet"));

const tokenMint = await createMint(conn, sender, sender.publicKey, null, 2);
const link = getExplorerLink("address", tokenMint.toString(), "devnet");
console.log(`Token Mint: ${link}`);
