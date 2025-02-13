use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
    system_instruction,
};

pub fn create_transaction() -> Transaction {
    let from_keypair= Keypair::new();
    let to_pubkey = Pubkey::new_unique() ;

    let instruction = system_instruction::transfer(
        &from_keypair.pubkey(),
        &to_pubkey,
        100
    );
    let transaction = Transaction::new_with_payer(&[instruction], Some(&from_keypair.pubkey()));
    transaction 
}