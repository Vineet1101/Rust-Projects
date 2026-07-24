use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_2022::{CloseAccount, TransferChecked, transfer_checked}, token_interface::{Mint, TokenAccount,close_account}};

use crate::state::Escrow;


#[derive(Accounts)]
pub struct Refund<'info>{

    #[account(mut)]
    pub maker:Signer<'info>,
    #[account(mut)]
    pub maker_ata_a:InterfaceAccount<'info,TokenAccount>,

    #[account(
        mut,
        associated_token::mint=mint_a,
        associated_token::authority=escrow,
        associated_token::token_program=token_program
    )]
    pub vault:InterfaceAccount<'info,TokenAccount>,


    #[account(
        mut,
        close=maker,
        has_one=mint_a,
        has_one=maker,
        seeds=[b"escrow",maker.key().as_ref(),escrow.seed.to_le_bytes().as_ref()],
        bump=escrow.bump
    )]
    pub escrow:Account<'info,Escrow>,

    pub mint_a:InterfaceAccount<'info,Mint>,

    pub system_program:Program<'info,System>,
    pub token_program:Interface<'info,TokenInterface>,
    pub associated_token_program:Program<'info,AssociatedToken>
}




pub fn handler(ctx:Context<Refund>)->Result<()>{

    //Transfering token a from vault to maker
    let cpi_accounts=TransferChecked{
        from:ctx.accounts.vault.to_account_info(),
        to:ctx.accounts.maker_ata_a.to_account_info(),
        mint:ctx.accounts.mint_a.to_account_info(),
        authority:ctx.accounts.escrow.to_account_info()
    };

    let seeds=&[
        b"escrow",
        ctx.accounts.maker.to_account_info().key.as_ref(),
        &ctx.accounts.escrow.seed.to_le_bytes(),
        &[ctx.accounts.escrow.bump]
    ];

    let signer_seeds=[&seeds[..]];

    let cpi_context=CpiContext::new_with_signer(ctx.accounts.token_program.key(), cpi_accounts, &signer_seeds);

    transfer_checked(cpi_context, ctx.accounts.vault.amount, ctx.accounts.mint_a.decimals);

    let cpi_accounts=CloseAccount{
        account:ctx.accounts.vault.to_account_info(),
        destination:ctx.accounts.maker.to_account_info(),
        authority:ctx.accounts.escrow.to_account_info()
    };

    let cpi_context=CpiContext::new_with_signer(ctx.accounts.token_program.key(), cpi_accounts, &signer_seeds);

    close_account(cpi_context);

    


    Ok(())
}