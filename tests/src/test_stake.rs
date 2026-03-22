use litesvm::LiteSVM;
use litesvm_token::{CreateAssociatedTokenAccount, CreateMint, MintTo};
use solana_sdk::{message::{AccountMeta, Instruction}, pubkey::Pubkey, signature::{Keypair, read_keypair_file}, signer::Signer};
use spl_associated_token_account::get_associated_token_address;
use spl_token::native_mint::DECIMALS;
use solana_transaction::{Transaction};

#[derive(Debug)]
struct StakeArgs{
    pub amount:u64
}

#[test]
fn test_stake(){
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

    let tx=Transaction::new_signed_with_payer(&[instruction], Some(&user.pubkey()), &[user.insecure_clone()], svm.latest_blockhash());
    svm.send_transaction(tx).unwrap();
    
    let d = [
        206,
        176,
        202,
        18,
        200,
        209,
        179,
        108
      ];
    let mut stake_descriminator = [0u8; 8];
    stake_descriminator.copy_from_slice(&d[..8]);

    let mut stake_data = stake_descriminator.to_vec();
    let stakeargs = StakeArgs{
        amount:1_000_000_000
    };
    stake_data.extend_from_slice(&stakeargs.amount.to_le_bytes());
    let stake_instruction = Instruction{
        program_id,
        accounts:vec![
            AccountMeta::new(user.pubkey(), true),
            AccountMeta::new(vault, false),
            AccountMeta::new_readonly(solana_system_interface::program::id(), false)
        ],
        data:stake_data
    };

    let current_amount_in_vault = svm.get_balance(&vault).unwrap();

    let tx_2=Transaction::new_signed_with_payer(&[stake_instruction], Some(&user.pubkey()), &[user.insecure_clone()], svm.latest_blockhash());
    svm.send_transaction(tx_2).unwrap();
    let amount_in_vault = svm.get_balance(&vault).unwrap();
    assert_eq!(amount_in_vault,current_amount_in_vault+1_000_000_000);
}