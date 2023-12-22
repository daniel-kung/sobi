use dotenv::dotenv;
use sobi::{instruction::*, state::*, utils::*};
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_request::TokenAccountsFilter;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh0_10::try_from_slice_unchecked,
    bpf_loader_upgradeable::close_any,
    clock::UnixTimestamp,
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    system_program,
    sysvar::{clock::Clock, rent, Sysvar},
};
use solana_sdk::{
    signature::{read_keypair_file, Keypair, Signer},
    system_instruction::create_account,
    transaction::Transaction,
};
use spl_associated_token_account::{create_associated_token_account, get_associated_token_address};
use spl_token::{
    instruction::{initialize_mint, mint_to},
    state::{Account, Mint},
};
use std::env;
use std::str::FromStr;

const PROGRAM_ID: &str = "BCCBkgbofwLpBuJgfKzuUtxBSPxvxTMq6dqBeXzQ1vG7";
const METADATA_PROGRAM: &str = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s";

fn create_dev() {
    let client = RpcClient::new("https://api.devnet.solana.com".to_string());
    let program_id = Pubkey::from_str(PROGRAM_ID).unwrap();
    dotenv().ok();
    let kp_str = env::var("SECRET").unwrap();
    let metadata_program = Pubkey::from_str(METADATA_PROGRAM).unwrap();
    let signer = Keypair::from_base58_string(&kp_str.as_str());
    let token_program = spl_token::ID;
    let auth = signer.pubkey();
    let signer_pubkey = signer.pubkey();
    let new_mint = Keypair::new();
    let mint_pubkey = new_mint.pubkey();


    let seeds = &[
        program_id.as_ref(),
        mint_pubkey.as_ref(),
        "token_info".as_bytes(),
    ];
    let (token_info, _) = Pubkey::find_program_address(seeds, &program_id);
    println!(" token_info::::::{:?}", token_info.to_string());
    let seeds = &[
        program_id.as_ref(),
        mint_pubkey.as_ref(),
        "mint_auth".as_bytes(),
    ];
    let (auth, _) = Pubkey::find_program_address(seeds, &program_id);
    println!(" auth::::::{:?}", auth.to_string());

    let metadata_seeds = &[
        "metadata".as_bytes(),
        &metadata_program.as_ref(),
        mint_pubkey.as_ref(),
    ];
    let (metadata_key, _) = Pubkey::find_program_address(metadata_seeds, &metadata_program);
    println!(" metadata_key::::::{:?}", metadata_key.to_string());
    let mut instructions = vec![];
    let tokenargs = CreateTokenArgs {
        name: "taozi".to_string(),
        symbol: "tz".to_string(),
        uri: "https://arweave.net/tuYnuXbs7MfkspgEUbvKuA_yhejGovNmlo5cS2WFkao".to_string(),
        decimals: 9,
    };

    let mut new_mint_instructions = vec![
        create_account(
            &signer.pubkey(),
            &mint_pubkey,
            client
                .get_minimum_balance_for_rent_exemption(Mint::LEN)
                .unwrap(),
            Mint::LEN as u64,
            &spl_token::id(),
        ),
        initialize_mint(&spl_token::id(), &mint_pubkey, &auth, Some(&auth), tokenargs.decimals).unwrap(),
    ];
    instructions.append(&mut new_mint_instructions);

    instructions.push(
        create_token(
            &program_id,
            &signer_pubkey,
            &token_info,
            &mint_pubkey,
            &auth,
            &metadata_key,
            &metadata_program,
            tokenargs,
        )
        .unwrap(),
    );

    let mut transaction = Transaction::new_with_payer(&instructions, Some(&signer.pubkey()));
    let recent_blockhash = client.get_latest_blockhash().unwrap();
    let mut signers = vec![&signer];
    signers.push(&signer);
    signers.push(&new_mint);
    transaction.sign(&signers, recent_blockhash);
    let signature = client.send_and_confirm_transaction(&transaction).unwrap();

    println!("signature:::{:?}", &signature);
}

fn config_dev() {
    let client = RpcClient::new("https://api.devnet.solana.com".to_string());
    let program_id = Pubkey::from_str(PROGRAM_ID).unwrap();
    dotenv().ok();
    let kp_str = env::var("SECRET").unwrap();
    let signer = Keypair::from_base58_string(&kp_str.as_str());
    let token_program = spl_token::ID;
    let auth = signer.pubkey();
    let signer_pubkey = signer.pubkey();
    let seeds = &[program_id.as_ref(), "config".as_bytes()];
    let (config_info, _) = Pubkey::find_program_address(seeds, &program_id);
    println!(" config_info::::::{:?}", config_info.to_string());

    let mut instructions = vec![];
    let configargs = ConfigureArgs {
        /// Initialized state.
        /// Contract admin
        authority: auth,
    };

    instructions.push(configure(&program_id, &signer_pubkey, &config_info, configargs).unwrap());

    let mut transaction = Transaction::new_with_payer(&instructions, Some(&signer.pubkey()));
    let recent_blockhash = client.get_latest_blockhash().unwrap();
    let mut signers = vec![&signer];
    signers.push(&signer);
    transaction.sign(&signers, recent_blockhash);
    let signature = client.send_and_confirm_transaction(&transaction).unwrap();

    println!("signature:::{:?}", &signature);
}

fn mint_dev(mint_pubkey: &Pubkey) {
    let client = RpcClient::new("https://api.devnet.solana.com".to_string());
    let program_id = Pubkey::from_str(PROGRAM_ID).unwrap();
    dotenv().ok();
    let kp_str = env::var("SECRET").unwrap();
    let signer = Keypair::from_base58_string(&kp_str.as_str());
    let token_program = spl_token::ID;
    let signer_pubkey = signer.pubkey();
    let seeds = &[program_id.as_ref(), "config".as_bytes()];
    let (config_info, _) = Pubkey::find_program_address(seeds, &program_id);
    println!(" config_info::::::{:?}", config_info.to_string());

    let seeds = &[
        program_id.as_ref(),
        mint_pubkey.as_ref(),
        "token_info".as_bytes(),
    ];
    let (token_info, _) = Pubkey::find_program_address(seeds, &program_id);
    println!(" token_info::::::{:?}", token_info.to_string());
    let seeds = &[
        program_id.as_ref(),
        mint_pubkey.as_ref(),
        "mint_auth".as_bytes(),
    ];
    let (auth, _) = Pubkey::find_program_address(seeds, &program_id);
    println!(" auth::::::{:?}", auth.to_string());
    let user_seeds = &[
        &program_id.as_ref(),
        signer_pubkey.as_ref(),
        "user_info".as_bytes(),
    ];
    let (user_info, _) = Pubkey::find_program_address(user_seeds, &program_id);
    println!(" user_info::::::{:?}", user_info.to_string());

    let mut instructions = vec![];

    let token_account = get_associated_token_address(&signer_pubkey, &mint_pubkey);
    println!("token_account:::{:?}", token_account);

    if client.get_balance(&token_account).unwrap() == 0 {
        let new_token_account_instruction =
            create_associated_token_account(&signer_pubkey, &signer_pubkey, &mint_pubkey);
        instructions.push(new_token_account_instruction);
    }

    let mintargs = MintArgs {
        amt: 1000,
    };

    instructions.push(
        mint(
            &program_id,
            &signer_pubkey,
            &config_info,
            &mint_pubkey,
            &token_account,
            &token_info,
            &auth,
            &token_program,
            mintargs
        )
        .unwrap(),
    );

    let mut transaction = Transaction::new_with_payer(&instructions, Some(&signer.pubkey()));
    let recent_blockhash = client.get_latest_blockhash().unwrap();
    let mut signers = vec![&signer];
    signers.push(&signer);
    transaction.sign(&signers, recent_blockhash);
    let signature = client.send_and_confirm_transaction(&transaction).unwrap();

    println!("signature:::{:?}", &signature);
}

fn burn_dev(mint_pubkey: &Pubkey) {
    let client = RpcClient::new("https://api.devnet.solana.com".to_string());
    let program_id = Pubkey::from_str(PROGRAM_ID).unwrap();
    dotenv().ok();
    let kp_str = env::var("SECRET").unwrap();
    let signer = Keypair::from_base58_string(&kp_str.as_str());
    let token_program = spl_token::ID;
    let signer_pubkey = signer.pubkey();
    let seeds = &[program_id.as_ref(), "config".as_bytes()];
    let (config_info, _) = Pubkey::find_program_address(seeds, &program_id);
    println!(" config_info::::::{:?}", config_info.to_string());

    let seeds = &[
        program_id.as_ref(),
        mint_pubkey.as_ref(),
        "token_info".as_bytes(),
    ];
    let (token_info, _) = Pubkey::find_program_address(seeds, &program_id);
    println!(" token_info::::::{:?}", token_info.to_string());
    let seeds = &[
        program_id.as_ref(),
        mint_pubkey.as_ref(),
        "mint_auth".as_bytes(),
    ];
    let (auth, _) = Pubkey::find_program_address(seeds, &program_id);
    println!(" auth::::::{:?}", auth.to_string());
    let user_seeds = &[
        &program_id.as_ref(),
        signer_pubkey.as_ref(),
        "user_info".as_bytes(),
    ];
    let (user_info, _) = Pubkey::find_program_address(user_seeds, &program_id);
    println!(" user_info::::::{:?}", user_info.to_string());

    let mut instructions = vec![];

    let token_account = get_associated_token_address(&signer_pubkey, &mint_pubkey);
    println!("token_account:::{:?}", token_account);

    if client.get_balance(&token_account).unwrap() == 0 {
        let new_token_account_instruction =
            create_associated_token_account(&signer_pubkey, &signer_pubkey, &mint_pubkey);
        instructions.push(new_token_account_instruction);
    }

    let burnargs = BurnArgs {
        amt: 1000,
    };

    instructions.push(
        burn(
            &program_id,
            &signer_pubkey,
            &config_info,
            &mint_pubkey,
            &token_account,
            &token_info,
            &token_program,
            burnargs
        )
        .unwrap(),
    );

    let mut transaction = Transaction::new_with_payer(&instructions, Some(&signer.pubkey()));
    let recent_blockhash = client.get_latest_blockhash().unwrap();
    let mut signers = vec![&signer];
    signers.push(&signer);
    transaction.sign(&signers, recent_blockhash);
    let signature = client.send_and_confirm_transaction(&transaction).unwrap();

    println!("signature:::{:?}", &signature);
}

fn main() {
    // config_dev();
    let config_info = Pubkey::from_str("AqnULXaaHcxK4fRPJDnhjQfjQgBiSiJyb6HmFS5DuvfQ").unwrap();
    create_dev();
}
