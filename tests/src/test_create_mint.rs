use litesvm::LiteSVM;
use solana_sdk::{message::{AccountMeta, Instruction}, signature::{Keypair, read_keypair_file}, signer::Signer};
use solana_transaction::{Transaction};

#[test]
fn test_create_mint(){
    let mut svm = LiteSVM::new();
    let program_keypair = read_keypair_file("../target/deploy/staking-keypair.json").unwrap();
    let program_id = program_keypair.pubkey();
    let program_bytes = include_bytes!("../../target/deploy/staking.so");

    svm.add_program(program_id, program_bytes).unwrap();

    let signer = Keypair::new();
    let mint = Keypair::new();
    svm.airdrop(&signer.pubkey(), 1_000_000_000_000).unwrap();
    
    let discrimator = [
        69,
        44,
        215,
        132,
        253,
        214,
        41,
        45
      ];
    let mut mint_descrimator = [0u8; 8];
    mint_descrimator.copy_from_slice(&discrimator[..8]);

    let mint_instruction_data = mint_descrimator.to_vec();
    let mint_instructions = Instruction{
        program_id,
        accounts:vec![
            AccountMeta::new(signer.pubkey(), true),
            AccountMeta::new(mint.pubkey(), true),
            AccountMeta::new_readonly(spl_token::id(),false),
            AccountMeta::new_readonly(solana_system_interface::program::id(),false)
        ],
        data:mint_instruction_data
    };

    let mint_tx=Transaction::new_signed_with_payer(&[mint_instructions], Some(&signer.pubkey()), &[&signer,&mint], svm.latest_blockhash());
    svm.send_transaction(mint_tx).unwrap();
}