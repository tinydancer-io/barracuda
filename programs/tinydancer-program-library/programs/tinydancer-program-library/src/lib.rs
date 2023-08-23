use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod tinydancer_program_library {
    use super::*;

    pub fn push_superblock(ctx: Context<PushSuperblock>, root: [u8;32], signature: [u8;64],  slot_start: u64, slot_end: u64 ) -> Result<()> {
        
        ctx.accounts.superblock.validator_identity = ctx.accounts.signer.key();
        ctx.accounts.superblock.slot_start = slot_start;
        ctx.accounts.superblock.slot_end = slot_end;

      
        let publickey = ed25519_dalek::PublicKey::from_bytes(&ctx.accounts.signer.key().as_ref()).unwrap();
        match publickey.verify_strict(&root,&signature.into()){
            Ok(_) => msg!("Signature Valid"),
            Err(e) => panic!("Error: {:?}",e)
        }
      
        ctx.accounts.superblock.signature = signature;
        ctx.accounts.superblock.root = root;
        
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(slot:u64,random_hash: [u8;32])]
pub struct PushSuperblock<'info> {
    #[account(init,seeds=[b"superblock",signer.key().as_ref(),&slot.to_be_bytes(),&random_hash],bump,payer=signer,space=Superblock::LEN)]
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