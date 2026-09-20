use anchor_lang::prelude::*;

use crate::{constants::TICKET_SEED, error::ErrorCode, state::{Event, Ticket}};

#[derive(Accounts)]
#[instruction(ticket_number: u32)]
pub struct BuyTicket<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub event: Account<'info, Event>,
    #[account(
        init,
        payer = buyer,
        space = Ticket::SPACE,
        seeds = [TICKET_SEED, event.key().as_ref(), &ticket_number.to_le_bytes()],
        bump
    )]
    pub ticket: Account<'info, Ticket>,
    /// CHECK: This address is constrained to the organizer stored in the event.
    #[account(mut, address = event.organizer)]
    pub organizer: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>, 
}

pub fn handler(ctx: Context<BuyTicket>, ticket_number: u32) -> Result<()> {
    let event = &mut ctx.accounts.event;
    let now = Clock::get()?.unix_timestamp;

    require!(event.active, ErrorCode::EventInactive);
    require!(now <= event.end_time, ErrorCode::EventEnded);
    require!(event.sold_tickets < event.total_tickets, ErrorCode::SoldOut);
    require!(ticket_number == event.sold_tickets, ErrorCode::InvalidTicketNumber);

    if event.ticket_price > 0 {
        let cpi_accounts = anchor_lang::system_program::Transfer {
            from: ctx.accounts.buyer.to_account_info(),
            to: ctx.accounts.organizer.to_account_info(),
        };
        anchor_lang::system_program::transfer(
            CpiContext::new(ctx.accounts.system_program.to_account_info(), cpi_accounts),
            event.ticket_price,
        )?;
    }

    let ticket = &mut ctx.accounts.ticket;
    ticket.event = event.key();
    ticket.owner = ctx.accounts.buyer.key();
    ticket.ticket_number = ticket_number;
    ticket.price_paid = event.ticket_price;
    ticket.purchased_at = now;
    ticket.used = false;
    ticket.bump = ctx.bumps.ticket;
    event.sold_tickets = event.sold_tickets.checked_add(1).ok_or(ErrorCode::SoldOut)?;

    emit!(TicketPurchased {
        event: ticket.event,
        ticket: ticket.key(),
        buyer: ticket.owner,
        ticket_number,
        price_paid: ticket.price_paid,
    });

    Ok(())
}

#[event]
pub struct TicketPurchased {
    pub event: Pubkey,
    pub ticket: Pubkey,
    pub buyer: Pubkey,
    pub ticket_number: u32,
    pub price_paid: u64,
}