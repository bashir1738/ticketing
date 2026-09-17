import * as anchor from "@anchor-lang/core";
import { Program } from "@anchor-lang/core";
import { expect } from "chai";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { Ticketing } from "../target/types/ticketing";

describe("ticketing", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.ticketing as Program<Ticketing>;
  const organizer = program.provider.publicKey;
  const eventId = new anchor.BN(Date.now());
  const eventPda = PublicKey.findProgramAddressSync(
    [Buffer.from("event"), organizer.toBuffer(), eventId.toArrayLike(Buffer, "le", 8)],
    program.programId,
  )[0];
  const ticketPda = (ticketNumber: number) =>
    PublicKey.findProgramAddressSync(
      [Buffer.from("ticket"), eventPda.toBuffer(), new anchor.BN(ticketNumber).toArrayLike(Buffer, "le", 4)],
      program.programId,
    )[0];

  const startTime = Math.floor(Date.now() / 1000) - 5;
  const endTime = startTime + 3600;

  it("creates an event and sells a PDA-backed ticket", async () => {
    await program.methods
      .initializeEvent(
        eventId,
        "Solana Summer Conference",
        "Lisbon Expo",
        new anchor.BN(startTime),
        new anchor.BN(endTime),
        new anchor.BN(1_000_000),
        2,
      )
      .accountsPartial({ organizer, event: eventPda, systemProgram: SystemProgram.programId })
      .rpc();

    await program.methods
      .buyTicket(0)
      .accountsPartial({ buyer: organizer, event: eventPda, ticket: ticketPda(0), organizer, systemProgram: SystemProgram.programId })
      .rpc();

    const event = await program.account.event.fetch(eventPda);
    const ticket = await program.account.ticket.fetch(ticketPda(0));
    expect(event.soldTickets).to.equal(1);
    expect(ticket.owner.toBase58()).to.equal(organizer.toBase58());
    expect(ticket.used).to.equal(false);
  });

  it("checks in a ticket only during the event and rejects reuse", async () => {
    await program.methods
      .checkInTicket()
      .accountsPartial({ organizer, event: eventPda, ticket: ticketPda(0) })
      .rpc();

    const ticket = await program.account.ticket.fetch(ticketPda(0));
    expect(ticket.used).to.equal(true);

    try {
      await program.methods
        .checkInTicket()
        .accountsPartial({ organizer, event: eventPda, ticket: ticketPda(0) })
        .rpc();
      expect.fail("a ticket cannot be checked in twice");
    } catch (error) {
      expect(String(error)).to.contain("TicketAlreadyUsed");
    }
  });

  it("cancels an event before it starts", async () => {
    const cancellationId = new anchor.BN(Date.now() + 1);
    const cancellationEvent = PublicKey.findProgramAddressSync(
      [Buffer.from("event"), organizer.toBuffer(), cancellationId.toArrayLike(Buffer, "le", 8)],
      program.programId,
    )[0];

    await program.methods
      .initializeEvent(
        cancellationId,
        "Cancelled Event",
        "Online",
        new anchor.BN(endTime + 100),
        new anchor.BN(endTime + 200),
        new anchor.BN(0),
        1,
      )
      .accountsPartial({ organizer, event: cancellationEvent, systemProgram: SystemProgram.programId })
      .rpc();

    await program.methods
      .cancelEvent()
      .accountsPartial({ organizer, event: cancellationEvent })
      .rpc();

    const event = await program.account.event.fetch(cancellationEvent);
    expect(event.active).to.equal(false);
  });
});
