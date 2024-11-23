import "dotenv/config";
import {
  clusterApiUrl,
  Connection,
  Keypair,
  PublicKey,
  sendAndConfirmTransaction,
  Transaction,
} from "@solana/web3.js";
import { createCreateMetadataAccountV3Instruction } from "@metaplex-foundation/mpl-token-metadata";
import { getExplorerLink } from "@solana-developers/helpers";

const user = Keypair.fromSecretKey(
  new Uint8Array(JSON.parse(process.env["PRIVATE_KEY"] ?? "")),
);
console.log(`Our public key is: ${user.publicKey.toBase58()}`);

const conn = new Connection(clusterApiUrl("devnet"));

const TOKEN_METADATA_PROGRAM_ID = new PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s",
);
const tokenMintAccount = new PublicKey(
  "DHNKtiQvD9aGGqyDeyNdEfbARV9TsQn9dJQUyjRh6bHx",
);

const metadataData = {
  name: "CoolCoin",
  symbol: "CCoin",
  uri: "https://gateway.pinata.cloud/ipfs/mycoolcoin",
  sellerFeeBasisPoints: 0,
  creators: null,
  collection: null,
  uses: null,
};

const [metadataPDA, _metadataBump] = PublicKey.findProgramAddressSync(
  [
    Buffer.from("metadata"),
    TOKEN_METADATA_PROGRAM_ID.toBuffer(),
    tokenMintAccount.toBuffer(),
  ],
  TOKEN_METADATA_PROGRAM_ID,
);

const transaction = new Transaction();
const createMetadataAccountInstruction =
  createCreateMetadataAccountV3Instruction(
    {
      metadata: metadataPDA,
      mint: tokenMintAccount,
      mintAuthority: user.publicKey,
      payer: user.publicKey,
      updateAuthority: user.publicKey,
    },
    {
      createMetadataAccountArgsV3: {
        collectionDetails: null,
        data: metadataData,
        isMutable: true,
      },
    },
  );
transaction.add(createMetadataAccountInstruction);

await sendAndConfirmTransaction(conn, transaction, [user]);

const tokenMintLink = getExplorerLink(
  "address",
  tokenMintAccount.toString(),
  "devnet",
);
console.log(`Look at the token mint again: ${tokenMintLink}!`);
