pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("EakyJn3c1n623iNUBqq8gx2vvrGvEHcfYewXjBZ3vFuU");

#[program]
pub mod swap {
    use super::*;

    pub fn make_offer(
        context: Context<MakeOffer>,
        id: u64,
        transfer_amount: u64,
        expected_amount: u64,
    ) -> Result<()> {
        instructions::make_offer::send_tokens_to_vault(&context, transfer_amount)?;

        instructions::make_offer::save_offer(context, id, expected_amount)
    }

    pub fn take_offer(context: Context<TakeOffer>) -> Result<()> {
        instructions::take_offer::send_tokens_to_maker(&context)?;

        instructions::take_offer::withdraw_and_close_vault(context)
    }
}
