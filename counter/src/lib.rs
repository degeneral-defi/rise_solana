mod instructions;

use crate::instructions::CounterInstruction;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::{next_account_info, Account, AccountInfo};
use solana_program::entrypoint_deprecated::ProgramResult;
use solana_program::pubkey::Pubkey;
use solana_program::{entrypoint, msg};

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct CounterAccount {
    pub counter: u32,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Counter program entrypoint");
    let instruction = CounterInstruction::unpack(instruction_data)?;
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;
    let mut counter_account = CounterAccount::try_from_slice(&account.data.borrow())?;
    match instruction {
        CounterInstruction::Increment(args) => {
            counter_account.counter += args.value;
        }
        CounterInstruction::Decrement(args) => {
            if args.value > counter_account.counter {
                counter_account.counter = 0;
            }
            else {
                counter_account.counter -= args.value;
            }
        }
        CounterInstruction::Update(args) => {
            counter_account.counter = args.value;
        }
        CounterInstruction::Reset => {
            counter_account.counter = 0;
        }
    }

    counter_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;
    use std::arch::aarch64::vaba_s8;
    use std::mem;

    #[test]
    fn test_counter() {
        let program_id = Pubkey::default();
        let key: Pubkey = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();

        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );

        let accounts = vec![account];

        let mut increment_instruction = vec![0];
        let mut decrement_instruction = vec![1];
        let mut update_instruction = vec![2];
        let reset_instruction = vec![3];

        let increment_amount = 20u32;
        increment_instruction.extend_from_slice(&increment_amount.to_le_bytes());
        process_instruction(&program_id, &accounts, &increment_instruction).unwrap();
        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            20
        );

        decrement_instruction.extend_from_slice(&increment_amount.to_le_bytes());
        process_instruction(&program_id, &accounts, &decrement_instruction).unwrap();
        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );

        let update_value = 100u32;
        update_instruction.extend_from_slice(&update_value.to_le_bytes());
        process_instruction(&program_id, &accounts, &update_instruction).unwrap();
        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            100
        );

        process_instruction(&program_id, &accounts, &reset_instruction).unwrap();
        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );
    }
}
