use std::fs;

use std::io::Error;

use crate::Superblock;
use std::fs::OpenOptions;
use std::io::prelude::*;
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
