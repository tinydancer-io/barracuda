use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod tinydancer_program_library {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub signer: Signer<'info>
}

#[account]
pub struct Superblock{
 pub root: [u8;32]
 pub signature: [u8;64] // signed root
 pub validator_identity: Pubkey,
 pub slot_start: u64 // inclusive
 pub slot_end: u64 // exclusive
}