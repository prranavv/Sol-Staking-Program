use litesvm::LiteSVM;
use litesvm_token::{CreateAssociatedTokenAccount, CreateMint, MintTo};
use solana_sdk::{message::{AccountMeta, Instruction}, pubkey::Pubkey, signature::{Keypair, read_keypair_file}, signer::Signer};
use spl_associated_token_account::get_associated_token_address;
use spl_token::native_mint::DECIMALS;
use solana_transaction::{Transaction};

#[test]
fn test_create_stake_account(){
    let mut svm = LiteSVM::new();
    let program_keypair = read_keypair_file("../target/deploy/staking-keypair.json").unwrap();
    let program_id = program_keypair.pubkey();
    let program_bytes = include_bytes!("../../target/deploy/staking.so");

    svm.add_program(program_id, program_bytes).unwrap();
    
    let user = Keypair::new();
    svm.airdrop(&user.pubkey(), 1_000_000_000_000).unwrap();

    let discrimator = [
        105,
        24,
        131,
        19,
        201,
        250,
        157,
        73
      ];
    let mut mint_descrimator = [0u8; 8];
    mint_descrimator.copy_from_slice(&discrimator[..8]);

    let mint_instruction_data = mint_descrimator.to_vec();

    let pubkey = user.pubkey();
    let (vault,_bump) =Pubkey::find_program_address(&[b"staking",pubkey.as_ref()], &program_id); 


    let instruction = Instruction{
        program_id,
        accounts:vec![
            AccountMeta::new(user.pubkey(), true),
            AccountMeta::new(vault, false),
            AccountMeta::new_readonly(solana_system_interface::program::id(), false)
        ],
        data:mint_instruction_data
    };

    let tx=Transaction::new_signed_with_payer(&[instruction], Some(&user.pubkey()), &[user], svm.latest_blockhash());
    svm.send_transaction(tx).unwrap();
}