import "dotenv/config";
import { clusterApiUrl, Connection, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";

const keypair = Keypair.fromSecretKey(
  new Uint8Array(JSON.parse(process.env["PRIVATE_KEY"] ?? ""))
);
console.log(`Our public key is: ${keypair.publicKey.toBase58()}`);

const conn = new Connection(clusterApiUrl("devnet"));

const signature = await conn.requestAirdrop(
  keypair.publicKey,
  1 * LAMPORTS_PER_SOL
);
await conn.confirmTransaction(signature);
console.log(`Success! 1 SOL: ${keypair.publicKey}`);
