import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { TinydancerProgramLibrary } from "../target/types/tinydancer_program_library";

describe("tinydancer-program-library", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.TinydancerProgramLibrary as Program<TinydancerProgramLibrary>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
