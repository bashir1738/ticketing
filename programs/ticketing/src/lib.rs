pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("GQFxmaiXc9sbyzrAtC9oTqdxbB9D6YZvTCzxxHkKefuZ");

#[program]
pub mod ticketing {
    use super::*;

    pub fn initialize_event(
        ctx: Context<InitializeEvent>,
        event_id: u64,
        name: String,
        venue: String,
        start_time: i64,
        end_time: i64,
        ticket_price: u64,
        total_tickets: u32,
    ) -> Result<()> {
        initialize_event::handler(ctx, event_id, name, venue, start_time, end_time, ticket_price, total_tickets)
    }

    pub fn buy_ticket(ctx: Context<BuyTicket>, ticket_number: u32) -> Result<()> {
        buy_ticket::handler(ctx, ticket_number)
    }

    pub fn check_in_ticket(ctx: Context<CheckInTicket>) -> Result<()> {
        check_in_ticket::handler(ctx)
    }

    pub fn cancel_event(ctx: Context<CancelEvent>) -> Result<()> {
        cancel_event::handler(ctx)
    }
}
