use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    // example_mocks::solana_sdk::instruction,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{Sysvar, rent::Rent},
};

// Define program state
#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct CounterAccount {
    pub count: u64,
}

// Define instruction enum
#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub enum CounterInstruction {
    IncrementCounter,
    DecrementCounter,
    InitializeCounter { initial_value: u64 },
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Implement instruction deserialization
    let instruction = CounterInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    // Route instructions to handlers
    // let _: () = match instruction {
    match instruction {
        CounterInstruction::IncrementCounter => {
            msg!("Incrementing counter");
            increment_counter(accounts, program_id)?
        }
        CounterInstruction::DecrementCounter => {
            msg!("Decrementing counter");
            decrement_counter(accounts, program_id)?
        }
        CounterInstruction::InitializeCounter { initial_value } => {
            msg!("Initializing counter with value: {}", initial_value);
            initialize_counter(accounts, program_id, initial_value)?
        }
    }
    Ok(())
}

fn increment_counter(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let counter_account = next_account_info(accounts_iter)?;

    if counter_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut data = counter_account.data.borrow_mut();

    let mut counter_data: CounterAccount = CounterAccount::try_from_slice(&data)?;

    counter_data.count = counter_data
        .count
        .checked_add(1)
        .ok_or(ProgramError::InvalidAccountData)?;

    counter_data.serialize(&mut &mut data[..])?;

    msg!("Counter incremented to: {}", counter_data.count);

    Ok(())
}

fn decrement_counter(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
    Ok(())
}

fn initialize_counter(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    initial_value: u64,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let counter_account = next_account_info(accounts_iter)?;
    let payer_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    let account_space: usize = 8;

    let rent = Rent::get()?;
    let required_lamports = rent.minimum_balance(account_space);

    invoke(
        &system_instruction::create_account(
            payer_account.key,
            counter_account.key,
            required_lamports,
            account_space as u64,
            program_id,
        ),
        &[
            payer_account.clone(),
            counter_account.clone(),
            system_program.clone(),
        ],
    )?;

    let counter_data = CounterAccount {
        count: initial_value,
    };

    let mut account_data = &mut counter_account.data.borrow_mut()[..];
    counter_data.serialize(&mut account_data)?;

    msg!("Counter initialized with value: {}", initial_value);

    Ok(())
}

#[cfg(test)]
mod test {
    use std::result;

    use super::*;
    use litesvm::LiteSVM;
    use solana_program::example_mocks::solana_sdk::transaction;
    use solana_sdk::{
        account::ReadableAccount,
        instruction::{AccountMeta, Instruction},
        message::{self, Message},
        signature::{Keypair, Signer},
        system_program,
        transaction::Transaction,
    };

    #[test]
    fn test_counter_program() {
        // Initialize test environment
        let mut svm = LiteSVM::new();
        let payer = Keypair::new();

        svm.airdrop(&payer.pubkey(), 1_000_000_000)
            .expect("Failed to airdrop");

        // Load the program
        let program_keypair = Keypair::new();
        let program_id = program_keypair.pubkey();

        svm.add_program_from_file(program_id, "../../target/deploy/counter_nativ.so")
            .expect("Failed to load program");

        // Test initialization instruction
        let counter_keypair = Keypair::new();
        let intial_value = 42;
        println!("Testing counter intitialization...");

        let init_instruction_date = borsh::to_vec(&CounterInstruction::InitializeCounter {
            initial_value: intial_value,
        })
        .expect("Filed to serialize instruction");

        let intialise_instruction = Instruction::new_with_bytes(
            program_id,
            &init_instruction_date,
            vec![
                AccountMeta::new(counter_keypair.pubkey(), true),
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new(system_program::id(), false),
            ],
        );

        let message = Message::new(&[intialise_instruction], Some(&payer.pubkey()));

        let transaction =
            Transaction::new(&[&payer, &counter_keypair], message, svm.latest_blockhash());

        let result = svm.send_transaction(transaction);

        let logs = result.unwrap().logs;
        println!("Transaction logs:\n{:#?}", logs);

        let account = svm
            .get_account(&counter_keypair.pubkey())
            .expect("Failed to get counter account");

        let counter: CounterAccount = CounterAccount::try_from_slice(account.data())
            .expect("Failed to deserialize counter data");

        assert_eq!(counter.count, 42);
        println!(
            "Counter initialized successfully with value: {}",
            counter.count
        );

        // Test increment instruction
        println!("Testing counter increment...");

        let increment_instruction_data = borsh::to_vec(&CounterInstruction::IncrementCounter)
            .expect("Failed to serialize instruction");

        let increment_instruction = Instruction::new_with_bytes(
            program_id,
            &increment_instruction_data,
            vec![AccountMeta::new(counter_keypair.pubkey(), true)],
        );

        let message = Message::new(&[increment_instruction], Some(&payer.pubkey()));
        let transaction =
            Transaction::new(&[&payer, &counter_keypair], message, svm.latest_blockhash());

        let result = svm.send_transaction(transaction);
        assert!(result.is_ok(), "Increment transaction should succeed");

        let logs = result.unwrap().logs;
        println!("Transaction logs:\n{:#?}", logs);

        // Verify final results
        let account = svm
            .get_account(&counter_keypair.pubkey())
            .expect("Failed to get counter account");
        let counter: CounterAccount = CounterAccount::try_from_slice(account.data())
            .expect("Failed to deserialize counter data");
        assert_eq!(counter.count, 43, "Counter should be incremented to 43");
        println!("Counter incremented successfully to: {}", counter.count);
    }
}
