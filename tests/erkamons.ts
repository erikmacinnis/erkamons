import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Erkamons } from "../target/types/erkamons";

describe("erkamons", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Erkamons as Program<Erkamons>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
