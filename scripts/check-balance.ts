import { clusterApiUrl, Connection, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";

const conn = new Connection(clusterApiUrl("devnet"));
console.log("connected to devnet");

const publicKey = new PublicKey("Cf5yaRgQvjgySgMajURj7uoTfCv4XRs3HocwcYFDDtYA");

const balanceInLamports = await conn.getBalance(publicKey);
const balanceInSol = balanceInLamports / LAMPORTS_PER_SOL;

console.log(`The balance of ${publicKey} is ${balanceInSol} SOL`);
