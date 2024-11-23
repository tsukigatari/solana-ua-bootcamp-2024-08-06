import "dotenv/config";
import {
  clusterApiUrl,
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  sendAndConfirmTransaction,
  SystemProgram,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";

const sender = Keypair.fromSecretKey(
  new Uint8Array(JSON.parse(process.env["PRIVATE_KEY"] ?? "")),
);
console.log("public key: ", sender.publicKey.toBase58());

const conn = new Connection(clusterApiUrl("devnet"));
console.log("connected to devnet");

const recipient = new PublicKey("Cf5yaRgQvjgySgMajURj7uoTfCv4XRs3HocwcYFDDtYA");

const transaction = new Transaction();

const sendSolInstruction = SystemProgram.transfer({
  fromPubkey: sender.publicKey,
  toPubkey: recipient,
  lamports: 0.01 * LAMPORTS_PER_SOL,
});

transaction.add(sendSolInstruction);

transaction.add(
  new TransactionInstruction({
    keys: [{ pubkey: sender.publicKey, isSigner: true, isWritable: true }],
    data: Buffer.from(">_<", "utf-8"),
    programId: new PublicKey("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"),
  }),
);

const signature = await sendAndConfirmTransaction(conn, transaction, [sender]);
console.log(`Transaction confirmed, signature: ${signature}!`);
