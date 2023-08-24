use anchor_lang::prelude::*;

declare_id!("8iRs7VTbXwErab5vUjRH1tzJoUKLJpe2crYPXMZQKFpR");

#[program]
pub mod tinydancer_program_library {
    use super::*;

    pub fn push_superblock(ctx: Context<PushSuperblock>, slot_start: u64,random_hash: [u8;32], root: [u8;32], signature: [u8;64], slot_end: u64, ) -> Result<()> {
        
        ctx.accounts.superblock.validator_identity = ctx.accounts.signer.key();
        ctx.accounts.superblock.slot_start = slot_start;
        ctx.accounts.superblock.slot_end = slot_end;
      
        ctx.accounts.superblock.signature = signature;
        ctx.accounts.superblock.root = root;
        
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(slot_start: u64,random_hash: [u8;32])]
pub struct PushSuperblock<'info> {
    #[account(init,seeds=[b"superblock",signer.key().as_ref(),&slot_start.to_be_bytes(),&random_hash],bump,payer=signer,space=Superblock::LEN)]
    pub superblock: Account<'info,Superblock>,
    
    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Superblock{
 pub root: [u8;32],
 pub signature: [u8;64], // signed root
 pub validator_identity: Pubkey,
 pub slot_start: u64, // inclusive
 pub slot_end: u64 // exclusive
}

impl Superblock{
    pub const LEN: usize = 8 + 32 + 64 + 32 + 8 + 8;
}