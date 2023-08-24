use std::{fs, rc::Rc};

use crate::Superblock;
use anchor_client::solana_sdk::pubkey::Pubkey;
use anchor_client::solana_sdk::signature::Signer;
use anchor_client::{Client, Cluster};
use rand::{rngs::StdRng, RngCore, SeedableRng};
use solana_sdk::pubkey;
use solana_sdk::{commitment_config::CommitmentConfig, signature::read_keypair_file};
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::Error;
use std::sync::Arc;
#[derive(Debug)]
pub struct TryFromSliceError(());

pub fn slice_to_array_64<T>(slice: &[T]) -> Result<&[T; 64], TryFromSliceError> {
    if slice.len() == 64 {
        let ptr = slice.as_ptr() as *const [T; 64];
        unsafe { Ok(&*ptr) }
    } else {
        Err(TryFromSliceError(()))
    }
}

pub fn convert_batch_fixed<T>(v: Vec<T>) -> [T; 10] {
    let boxed_slice = v.into_boxed_slice();
    let boxed_array: Box<[T; 10]> = match boxed_slice.try_into() {
        Ok(ba) => ba,
        Err(o) => panic!("Expected a Vec of length {} but it was {}", 10, o.len()),
    };
    *boxed_array
}
pub fn save_to_file(data: Superblock, path: String) -> Result<(), Error> {
    let mut sblock = read_from_file(path.clone());
    let mut file = OpenOptions::new()
        .write(true)
        .append(false)
        .open(path)
        .unwrap();
    sblock.push(data);
    let d = serde_json::to_string(&sblock).unwrap();
    writeln!(file, "{}", &d)
}

pub fn read_from_file(path: String) -> Vec<Superblock> {
    let file = fs::File::open(path).expect("file should open read only");
    let superblocks: Vec<Superblock> = serde_json::from_reader(file).unwrap();
    superblocks
}

pub fn send_root_to_contract(superblock: Superblock, root: [u8; 32]) {
    let payer = read_keypair_file(&*shellexpand::tilde("~/.config/solana/id.json"))
        .expect("Example requires a keypair file");
    let url = Cluster::Custom(
        "http://localhost:8899".to_string(),
        "ws://127.0.0.1:8900".to_string(),
    );
    let payer = Rc::new(payer);
    let client =
        Client::new_with_options(url.clone(), payer.clone(), CommitmentConfig::processed());
   
    let client = Arc::new(client);
    let pid = pubkey!("8iRs7VTbXwErab5vUjRH1tzJoUKLJpe2crYPXMZQKFpR");
    let program = client.program(pid);
    let seed = [3u8; 32];
  
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let mut bytes = [0u8; 32];
    rng.fill_bytes(&mut bytes);
   
    let (superblock_account, _) = Pubkey::find_program_address(
        &[
            b"superblock",
            payer.pubkey().as_ref(),
            &superblock.start_slot.to_be_bytes(),
            &bytes,
        ],
        &pid,
    );


    let authority = program.payer();
    let signature = payer.sign_message(&root);

    program
        .request()
        .accounts(tinydancer_program_library::accounts::PushSuperblock {
            superblock: superblock_account,
            signer: authority,
            system_program: anchor_client::solana_sdk::system_program::ID,
        })
        .args(tinydancer_program_library::instruction::PushSuperblock {
            slot_start: superblock.start_slot,
            random_hash: bytes,
            root,
            signature: signature.into(),
            slot_end: superblock.end_slot,
        })
        .send().unwrap();
}
