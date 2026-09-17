use anchor_lang::prelude::*;

use crate::{constants::*, error::ErrorCode, state::Event};

#[derive(Accounts)]
#[instruction(event_id: u64)]
pub struct InitializeEvent<'info> {
    #[account(mut)]
    pub organizer: Signer<'info>,
    #[account(
        init,
        payer = organizer,
        space = Event::SPACE,
        seeds = [EVENT_SEED, organizer.key().as_ref(), &event_id.to_le_bytes()],
        bump
    )]
    pub event: Account<'info, Event>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<InitializeEvent>,
    event_id: u64,
    name: String,
    venue: String,
    start_time: i64,
    end_time: i64,
    ticket_price: u64,
    total_tickets: u32,
) -> Result<()> {
    require!(name.len() <= MAX_EVENT_NAME_LEN, ErrorCode::EventNameTooLong);
    require!(venue.len() <= MAX_VENUE_LEN, ErrorCode::VenueTooLong);
    require!(end_time > start_time, ErrorCode::InvalidEventWindow);
    require!(total_tickets > 0, ErrorCode::InvalidTicketSupply);

    let event = &mut ctx.accounts.event;
    event.organizer = ctx.accounts.organizer.key();
    event.name = name;
    event.venue = venue;
    event.start_time = start_time;
    event.end_time = end_time;
    event.ticket_price = ticket_price;
    event.total_tickets = total_tickets;
    event.sold_tickets = 0;
    event.active = true;
    event.bump = ctx.bumps.event;

    emit!(EventCreated {
        event: event.key(),
        organizer: event.organizer,
        event_id,
        total_tickets,
        ticket_price,
    });

    Ok(())
}

#[event]
pub struct EventCreated {
    pub event: Pubkey,
    pub organizer: Pubkey,
    pub event_id: u64,
    pub total_tickets: u32,
    pub ticket_price: u64,
}