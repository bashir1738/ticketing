use anchor_lang::prelude::*;

use crate::{constants::TICKET_SEED, error::ErrorCode, state::{Event, Ticket}};

#[derive(Accounts)]
pub struct TransferTicket<'info> {
    pub owner: Signer<'info>,
    pub event: Account<'info, Event>,
    #[account(
        mut,
        seeds = [TICKET_SEED, event.key().as_ref(), &ticket.ticket_number.to_le_bytes()],
        bump = ticket.bump,
        constraint = ticket.event == event.key() @ ErrorCode::InvalidTicketEvent
    )]
    pub ticket: Account<'info, Ticket>,
    /// CHECK: The recipient only receives the ticket owner assignment.
    pub new_owner: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<TransferTicket>) -> Result<()> {
    require_keys_eq!(
        ctx.accounts.owner.key(),
        ctx.accounts.ticket.owner,
        ErrorCode::UnauthorizedTicketOwner
    );
    require!(!ctx.accounts.ticket.used, ErrorCode::TicketAlreadyUsed);
    require_keys_neq!(
        ctx.accounts.owner.key(),
        ctx.accounts.new_owner.key(),
        ErrorCode::InvalidTicketRecipient
    );

    ctx.accounts.ticket.owner = ctx.accounts.new_owner.key();
    Ok(())
}
