use anchor_lang::prelude::*;
use anchor_spl::{token_2022::{TransferChecked, close_account, transfer_checked}, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::state::Escrow;




#[derive(Accounts)]
pub struct Take<'info>{

    #[account(mut)]
    pub taker:Signer<'info>,

    #[account(mut)]
    pub maker:SystemAccount<'info>,
    
    #[account(
        mut,
        has_one=maker,
        has_one=mint_a,
        has_one=mint_b,
        close=maker,
        seeds=[b"escrow".as_ref(),maker.key().as_ref(),seed.to_le_bytes().as_ref()],
        bump=escrow.bump
    )]
    pub escrow:Account<'info,Escrow>,

    pub mint_a:InterfaceAccount<'info,Mint>,

    
    pub mint_b:InterfaceAccount<'info,Mint>,

    #[account(
        init_if_needed,
        payer=taker,
        associated_token::mint=mint_a,
        associated_token::authority=taker,
        associated_token::token_program=token_program
    )]
    pub taker_ata_a:InterfaceAccount<'info,TokenAccount>,
    
    #[account(
        mut,
        associated_token::authority=taker,
        associated_token::mint=mint_b,
        associated_token::token_program=token_program
    )]
    pub taker_ata_b:InterfaceAccount<'info,TokenAccount>,

    #[account(
        mut,
        associated_token::mint=mint_a,
        associated_token::authority=escrow,
        associated_token::token_program=token_program
    )]
    pub vault:InterfaceAccount<'info,TokenAccount>,

    #[account(
        init_if_needed,
        payer=taker,
        associated_token::mint=mint_b,
        associated_token::authority=maker,
        associated_token::token_program=token_program
    )]
    pub maker_ata_b:InterfaceAccount<'info,TokenAccount>,

    pub system_program:Program<'info,System>,
    pub token_program:Interface<'info,TokenInterface>,
    pub associated_token_program:Program<'info,AssociatedToken>
}




pub fn handler(ctx:Context<Take>)->Result<()>{

    //Transfering tokens b from taker to maker. 
    // Simple CPI call is needed
    let cpi_accounts=TransferChecked{
        from:ctx.accounts.taker_ata_b.to_account_info(),
        to:ctx.accounts.maker_ata_b.to_account_info(),
        mint:ctx.accounts.mint_b.to_account_info(),
        authority:ctx.accounts.taker_ata_b.to_account_info()
    };

    let cpi_context=CpiContext::new(ctx.accounts.token_program.key(),cpi_accounts);
    transfer_checked(cpi_context, ctx.accounts.escrow.receive_amount, ctx.accounts.mint_b.decimals);

    //Transfering tokens a from valut to taker
    // Since valut owner is a escrow PDA so we need to sign the txn on behalf of escrow
    let seeds=&[
        b"escrow",
        ctx.accounts.maker.to_account_info().key.as_ref(),
        &ctx.accounts.escrow.seed.to_be_bytes()[..],
        &[ctx.accounts.escrow.bump]
    ];

    let signer_seeds=[&seeds[..]];
    let cpi_accounts=TransferChecked{
        from:ctx.accounts.vault.to_account_info(),
        authority:ctx.accounts.escrow.to_account_info(),
        to:ctx.accounts.taker_ata_a.to_account_info(),
        mint:ctx.accounts.mint_a.to_account_info()
    };

    let cpi_context=CpiContext::new_with_signer(ctx.accounts.token_program.key(), cpi_accounts, &signer_seeds);
    
    transfer_checked(cpi_context, ctx.accounts.vault.amount, ctx.accounts.mint_a.decimals);

    let cpi_accounts=CloseAccount{
        account:ctx.accounts.vault.to_account_info(),
        destination:ctx.accounts.maker.to_account_info(),
        authority:ctx.accounts.escrow.to_account_info(),
    };

    let cpi_context=CpiContext::new_with_signer(ctx.accounts.token_program.key(), cpi_accounts, &signer_seeds);
    close_account(cpi_context);
    Ok(())
}