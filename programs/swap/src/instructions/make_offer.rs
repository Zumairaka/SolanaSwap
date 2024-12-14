use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{Offer, ANCHOR_DISCRIMINATOR};

use super::transfer_tokens;

#[derive(Accounts)]
#[instruction(id:u64)]
pub struct MakeOffer<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(mint::token_program = token_program)]
    pub mint_token_a: InterfaceAccount<'info, Mint>,

    #[account(mint::token_program = token_program)]
    pub mint_token_b: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_token_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_account_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = maker,
        space = ANCHOR_DISCRIMINATOR + Offer::INIT_SPACE,
        seeds = [b"offer", maker.key().as_ref(), id.to_le_bytes().as_ref()],
        bump
    )]
    pub offer: Account<'info, Offer>,

    #[account(
        init,
        payer = maker,
        associated_token::mint = mint_token_a,
        associated_token::authority = offer,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn send_tokens_to_vault(context: &Context<MakeOffer>, offered_amount: u64) -> Result<()> {
    transfer_tokens(
        &context.accounts.maker_account_a,
        &context.accounts.vault,
        &offered_amount,
        &context.accounts.mint_token_a,
        &context.accounts.maker,
        &context.accounts.token_program,
    )
}

pub fn save_offer(context: Context<MakeOffer>, id: u64, expected_amount: u64) -> Result<()> {
    context.accounts.offer.set_inner(Offer {
        id: id,
        maker: context.accounts.maker.key(),
        token_mint_a: context.accounts.mint_token_a.key(),
        token_mint_b: context.accounts.mint_token_b.key(),
        token_amount_b: expected_amount,
        bump: context.bumps.offer,
    });

    Ok(())
}
