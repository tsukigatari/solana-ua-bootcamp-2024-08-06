import { Keypair } from "@solana/web3.js";
import "dotenv/config";

console.log("keypair in env: ", process.env["PRIVATE_KEY"]);

const keypair = Keypair.fromSecretKey(
  new Uint8Array(JSON.parse(process.env["PRIVATE_KEY"] ?? ""))
);

console.log("public key: ", keypair.publicKey.toBase58());
