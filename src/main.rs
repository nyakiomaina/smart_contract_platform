mod blockchain;
mod bytecode;
mod transaction;
mod util;

use bytecode::ByteCode;
use blockchain::{Blockchain,Block};
use transaction::Transaction;

fn main() {
    let mut blockchain = Blockchain::new();

    let tx1 = Transaction::DeployContract { name: "Contract1".to_string(), bytecode: vec![0x00, 0x61, 0x73, 0x6D] };

    let block1 = Block::new(vec![tx1], None);
    assert!(block1.is_valid(&None));
    blockchain.add_block(block1);

    let tx2 = Transaction::ExecuteContract { name: "Contract1".to_string(), args: vec!["arg1".to_string(), "arg2".to_string()] };

    let block2 = Block::new(vec![tx2], Some(blockchain.chain.last().unwrap().hash.clone()));
    assert!(block2.is_valid(&blockchain.chain.last()));
    blockchain.add_block(block2)
}