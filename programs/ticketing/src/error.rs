use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("The event name is too long")]
    EventNameTooLong,
    #[msg("The venue is too long")]
    VenueTooLong,
    #[msg("The event must end after it starts")]
    InvalidEventWindow,
    #[msg("The event must have at least one ticket")]
    InvalidTicketSupply,
    #[msg("The event is not active")]
    EventInactive,
    #[msg("The event has already started")]
    EventAlreadyStarted,
    #[msg("The event has not started")]
    EventNotStarted,
    #[msg("The event has ended")]
    EventEnded,
    #[msg("All tickets have been sold")]
    SoldOut,
    #[msg("The requested ticket number is not the next available ticket")]
    InvalidTicketNumber,
    #[msg("This ticket has already been checked in")]
    TicketAlreadyUsed,
    #[msg("The ticket does not belong to this event")]
    InvalidTicketEvent,
    #[msg("Only the event organizer can perform this action")]
    UnauthorizedOrganizer,
    #[msg("Only the current ticket owner can transfer this ticket")]
    UnauthorizedTicketOwner,
    #[msg("The ticket recipient must be different from the current owner")]
    InvalidTicketRecipient,
}
