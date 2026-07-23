use anchor_lang::prelude::*;

use crate::state::Escrow;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked}
};


#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Make<'info>{
    #[account(mut)]
    pub maker:Signer<'info>,

    #[account(
        init,
        payer=maker,
        space=Escrow::INIT_SPACE+8,
        seeds=[b"escrow".as_ref(),maker.key().as_ref(),seed.to_le_bytes().as_ref()],
        bump
    )]
    pub escrow:Account<'info,Escrow>,

    pub mint_a:InterfaceAccount<'info,Mint>,

    #[account(mint::token_program=token_program)]
    pub mint_b:InterfaceAccount<'info,Mint>,

    #[account(
        mut,
        associated_token::mint=mint_a,
        associated_token::authority=maker,
    )]
    pub maker_ata_a:InterfaceAccount<'info,TokenAccount>,

    #[account(
        init,
        payer=maker,
        associated_token::mint=mint_a,
        associated_token::authority=escrow,
    )]
    pub vault:InterfaceAccount<'info,TokenAccount>,
    
    pub system_program:Program<'info,System>,
    pub token_program:Interface<'info,TokenInterface>,
    pub associated_token_program:Program<'info,AssociatedToken>
}


pub fn handler(context:Context<Make>,seed:u64,receive_amount:u64,deposit_amount:u64)->Result<()>{

    //Populating the fields of escrow account
    let escrow_account=&mut context.accounts.escrow;
    escrow_account.seed=seed;
    escrow_account.receive_amount=receive_amount;
    escrow_account.bump=context.bumps.escrow;
    escrow_account.maker=context.accounts.maker.key();
    escrow_account.mint_a=context.accounts.mint_a.key();
    escrow_account.mint_b=context.accounts.mint_b.key();


    //Transfering the tokens from Maker's ATA to Vault
    let decimals=context.accounts.mint_a.decimals;
    let cpi_accounts=TransferChecked{
        mint:context.accounts.mint_a.to_account_info(),
        from:context.accounts.maker_ata_a.to_account_info(),
        to:context.accounts.vault.to_account_info(),
        authority:context.accounts.maker.to_account_info()
    };
    let cpi_program=context.accounts.token_program.key();
    let cpi_context=CpiContext::new(cpi_program,cpi_accounts);
    transfer_checked(cpi_context, deposit_amount, decimals)?;
    Ok(())
}