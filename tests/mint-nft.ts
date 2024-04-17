import * as anchor from "@coral-xyz/anchor";
import * as metaplex_token_metadata from "@metaplex-foundation/mpl-token-metadata"
import { Program } from "@coral-xyz/anchor";
import { MintNft } from "../target/types/mint_nft";

describe("mint-nft", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  const wallet = provider.wallet as anchor.Wallet;
  anchor.setProvider(provider);

  const program = anchor.workspace.MintNft as Program<MintNft>;

  const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(metaplex_token_metadata.MPL_TOKEN_METADATA_PROGRAM_ID);

  const name = "Teleport NFT";
  const symbol = "TPNFT";
  const uri = "https://raw.githubusercontent.com/notdanilo/mint-nft/main/assets/nft.json";

  it("Mint!", async () => {

    const mintKeypair: anchor.web3.Keypair = await anchor.web3.Keypair.generate();
    console.log(`Token created: ${mintKeypair.publicKey}`)

    const tokenAddress = await anchor.utils.token.associatedAddress({
      mint: mintKeypair.publicKey,
      owner: wallet.publicKey
    });
    console.log(`Token account ${tokenAddress} for ${wallet.publicKey}`);

    const metadataAddress = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mintKeypair.publicKey.toBuffer()  
      ],
      TOKEN_METADATA_PROGRAM_ID
    )[0];
    console.log(`Metadata account: ${metadataAddress}`);

    const masterEditionAddress = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mintKeypair.publicKey.toBuffer(),
        Buffer.from("edition")
      ],
      TOKEN_METADATA_PROGRAM_ID
    )[0];
    console.log(`Edition account: ${masterEditionAddress}`);

    let signature = await program.methods.mint(name, symbol, uri)
      .accounts({
        masterEdition: masterEditionAddress,
        metadata: metadataAddress,
        mint: mintKeypair.publicKey,
        tokenAccount: tokenAddress,
        mintAuthority: wallet.publicKey,
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
      })
      .signers([mintKeypair])
      .rpc({
        skipPreflight:true
      });
    console.log(`Transaction signature: ${signature}`);
  });
});
