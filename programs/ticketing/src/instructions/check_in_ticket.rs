use anchor_lang::prelude::*;

use crate::{constants::TICKET_SEED, error::ErrorCode, state::{Event, Ticket}};

#[derive(Accounts)]
pub struct CheckInTicket<'info> {
    #[account(mut)]
    pub organizer: Signer<'info>,
    #[account(has_one = organizer @ ErrorCode::UnauthorizedOrganizer)]
    pub event: Account<'info, Event>,
    #[account(
        mut,
        seeds = [TICKET_SEED, event.key().as_ref(), &ticket.ticket_number.to_le_bytes()],
        bump = ticket.bump,
        constraint = ticket.event == event.key() @ ErrorCode::InvalidTicketEvent
    )]
    pub ticket: Account<'info, Ticket>,
}

pub fn handler(ctx: Context<CheckInTicket>) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    let event = &ctx.accounts.event;
    require!(event.active, ErrorCode::EventInactive);
    require!(now >= event.start_time, ErrorCode::EventNotStarted);
    require!(now <= event.end_time, ErrorCode::EventEnded);
    require!(!ctx.accounts.ticket.used, ErrorCode::TicketAlreadyUsed);

    ctx.accounts.ticket.used = true;
    emit!(TicketCheckedIn {
        event: event.key(),
        ticket: ctx.accounts.ticket.key(),
        owner: ctx.accounts.ticket.owner,
    });
    Ok(())
}

#[event]
pub struct TicketCheckedIn {
    pub event: Pubkey,
    pub ticket: Pubkey,
    pub owner: Pubkey,
}