use anchor_lang::prelude::*;

use crate::{error::ErrorCode, state::Event};

#[derive(Accounts)]
pub struct CancelEvent<'info> {
    pub organizer: Signer<'info>,
    #[account(mut, has_one = organizer @ ErrorCode::UnauthorizedOrganizer)]
    pub event: Account<'info, Event>,
}

pub fn handler(ctx: Context<CancelEvent>) -> Result<()> {
    require!(ctx.accounts.event.active, ErrorCode::EventInactive);
    require!(Clock::get()?.unix_timestamp < ctx.accounts.event.start_time, ErrorCode::EventAlreadyStarted);
    ctx.accounts.event.active = false;
    emit!(EventCancelled { event: ctx.accounts.event.key() });
    Ok(())
}

#[event]
pub struct EventCancelled {
    pub event: Pubkey,
}