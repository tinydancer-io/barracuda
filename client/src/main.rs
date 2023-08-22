use jito_geyser_protos::solana::geyser::{
    geyser_client::GeyserClient, SubscribeTransactionUpdatesRequest, TransactionUpdate,
};
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;
use std::time::Duration;
use utils::{convert_batch_fixed, save_to_file};

use crate::utils::slice_to_array_64;
mod utils;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // creating a channel ie connection to server
    let channel = tonic::transport::Channel::from_static("http://0.0.0.0:10000")
        .connect()
        .await?;
    // creating gRPC client from channel
    let mut client = GeyserClient::new(channel);
    // creating a new Request
    let request = tonic::Request::new(SubscribeTransactionUpdatesRequest {});
    // sending request and waiting for response
    let mut stream = client
        .subscribe_transaction_updates(request)
        .await?
        .into_inner();

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    let tx_stream = tx.clone();

    tokio::spawn(async move {
        while let Ok(received) = stream.message().await {
            let received = received.unwrap();
            tx_stream.send(received.transaction.clone().unwrap());
            println!(
                "\treceived message: `{:?}`",
                received.transaction.unwrap().slot
            );
        }
    });
    let mut txns_in_slot = vec![];
    let (tx2, mut rx2) = tokio::sync::mpsc::unbounded_channel::<Vec<TransactionUpdate>>();
    let (tx3, mut rx3) = tokio::sync::mpsc::unbounded_channel();
    tokio::spawn(async move {
        loop {
            if let Some(batch) = rx2.recv().await {
                println!("Batch for slot {} has {} txns", batch[0].slot, batch.len());
                let receipts = batch
                    .iter()
                    .map(|tx| Receipt {
                        signature: tx.signature.clone(),
                        status: if tx.tx.clone().unwrap().meta.unwrap().err.is_none() {
                            1
                        } else {
                            0
                        },
                        tx_idx: tx.tx_idx,
                    })
                    .collect::<Vec<Receipt>>();

                tx3.send(ReceiptBatch(receipts, batch[0].slot));
            }
        }
    });

    tokio::spawn(async move {
        let mut superblock_batches = vec![];
        let mut last_end_slot = 0;
        while let Some(batch) = rx3.recv().await {
            if superblock_batches.clone().len() < 10 {
                superblock_batches.push(batch)
            } else {
                if last_end_slot == 0 {
                    let start_slot = superblock_batches[0].1;
                    let end_slot = superblock_batches.last().unwrap().1 + 1;
                    let superblock = Superblock {
                        start_slot,
                        end_slot,
                        receipts: convert_batch_fixed(superblock_batches.clone()),
                    };
                    save_to_file(superblock, "src/data.json".into());
                    last_end_slot = end_slot;
                    superblock_batches = vec![];
                } else {
                    let start_slot = last_end_slot;
                    let end_slot = superblock_batches.last().unwrap().1 + 1;
                    let superblock = Superblock {
                        start_slot,
                        end_slot,
                        receipts: convert_batch_fixed(superblock_batches.clone()),
                    };
                    save_to_file(superblock, "src/data.json".into());
                    last_end_slot = end_slot;
                    superblock_batches = vec![];
                }
            }
        }
    });
    while let Some(latest_txn) = rx.recv().await {
        if txns_in_slot.len() != 0 {
            let last_txn: &TransactionUpdate = txns_in_slot.last().unwrap();
            if last_txn.slot == latest_txn.slot {
                // safe because len check
                txns_in_slot.push(latest_txn);
            } else {
                tx2.send(txns_in_slot);
                txns_in_slot = vec![];
                txns_in_slot.push(latest_txn);
            }
        } else {
            txns_in_slot.push(latest_txn);
        }
    }

    Ok(())
}

#[derive(Serialize, Deserialize)]
pub struct Superblock {
    /// inclusive
    pub start_slot: u64,
    /// exclusive
    pub end_slot: u64,

    pub receipts: [ReceiptBatch; 10],
}

/// (receipts, start_slot)
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct ReceiptBatch(Vec<Receipt>, u64);

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Receipt {
    // receipts must be order by the txns index in the block
    pub tx_idx: u64,
    // #[serde(with = "BigArray")]
    pub signature: String,
    // 1 being successful and 0 being failed
    pub status: u8,
}