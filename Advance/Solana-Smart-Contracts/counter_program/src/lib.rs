use solana_program::{account_info::next_account_info, msg,entrypoint,entrypoint::{ __AccountInfo, ProgramResult}, example_mocks::solana_sdk::system_instruction, program::invoke, program_error::ProgramError, pubkey::Pubkey, sysvar::{Sysvar, rent::Rent}};
use borsh::{BorshDeserialize,BorshSerialize};

entrypoint!(process_instruction);



#[derive(BorshDeserialize,BorshSerialize,Debug)]
pub struct Counter{
    count:u64
}


#[derive(BorshDeserialize,BorshSerialize,Debug)]
pub enum Instruction{
    Initialize{initial_value:u64},
    Double
}

pub fn process_instruction(
    program_id:&Pubkey,
    accounts:&[__AccountInfo],
    ixn:&[u8]
)->ProgramResult{

    


    let instruction_type=Instruction::try_from_slice(ixn).map_err(|_| ProgramError::InvalidInstructionData)?;

    match instruction_type{
        Instruction::Initialize{initial_value}=>{
            process_instruction_initialize(program_id, accounts, initial_value);
        },
        Instruction::Double=>{
            process_instruction_double(program_id, accounts);
        }
    }

    Ok(())
}


fn process_instruction_initialize(
    program_id:&Pubkey,
    accounts:&[__AccountInfo],
    initial_value:u64
)->ProgramResult{
    let account_iter=&mut accounts.iter();

    let counter_account=next_account_info(account_iter)?;
    let payer_account=next_account_info(account_iter)?;
    let system_program=next_account_info(account_iter)?;

    let account_space=8;

    let rent=Rent::get()?;
    let required_lamports=rent.minimum_balance(account_space);


    invoke(
        &system_instruction::create_account(
            payer_account.key, 
            counter_account.key, 
            required_lamports, 
            account_space as u64, 
            program_id
        ), 
        &[
            payer_account.clone(),
            counter_account.clone(),
            system_program.clone()
        ]
    )?;

    let counter_data=Counter{
        count:initial_value
    };

    let mut account_data=&mut counter_account.data.borrow_mut()[..];
    counter_data.serialize(&mut account_data)?;

    msg!("Counter initialized with value: {}",initial_value);
    Ok(())
}


fn process_instruction_double(
    program_id:&Pubkey,
    accounts:&[__AccountInfo],
)->ProgramResult{
    let account_iter=&mut accounts.iter();

    let data_account=next_account_info(account_iter).unwrap();

    if data_account.owner!=program_id{
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut counter=Counter::try_from_slice(&mut data_account.data.borrow_mut())?;

    counter.count=counter.count.checked_add(counter.count).ok_or(ProgramError::InvalidAccountData)?;

    counter.serialize(&mut &mut data_account.data.borrow_mut()[..])?;

    msg!("Counter doubled");

    Ok(())
}