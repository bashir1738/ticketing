use anchor_lang::prelude::*;

use crate::constants::{MAX_EVENT_NAME_LEN, MAX_VENUE_LEN};

#[account]
pub struct Event {
	pub organizer: Pubkey,
	pub name: String,
	pub venue: String,
	pub start_time: i64,
	pub end_time: i64,
	pub ticket_price: u64,
	pub total_tickets: u32,
	pub sold_tickets: u32,
	pub active: bool,
	pub bump: u8,
}

impl Event {
	pub const SPACE: usize = 8
		+ 32
		+ 4
		+ MAX_EVENT_NAME_LEN
		+ 4
		+ MAX_VENUE_LEN
		+ 8
		+ 8
		+ 8
		+ 4
		+ 4
		+ 1
		+ 1;
}

#[account]
pub struct Ticket {
	pub event: Pubkey,
	pub owner: Pubkey,
	pub ticket_number: u32,
	pub price_paid: u64,
	pub purchased_at: i64,
	pub used: bool,
	pub bump: u8,
}

impl Ticket {
	pub const SPACE: usize = 8 + 32 + 32 + 4 + 8 + 8 + 1 + 1;
}
