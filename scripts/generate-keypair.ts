import { Keypair } from "@solana/web3.js";
import "dotenv/config";

const keypair = Keypair.generate();
console.log("public key: ", keypair.publicKey.toBase58());
console.log("private key: ", keypair.secretKey);
