use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("GqtdriywSmas3dMgjoVYgk8DDoYyRFXtdg4edx7iSK2j");

#[program]
pub mod eventticketingsystem {
    use super::*;

    #[derive(Accounts)]
    pub struct CreateEvent<'info> {
        #[account(init, payer = creator, space = 8 + 64)]
        pub event: Account<'info, Event>,
        #[account(mut)]
        pub creator: Signer<'info>,
        pub system_program: Program<'info, System>,
    }

    #[derive(AnchorSerialize, AnchorDeserialize, Clone)]
    pub struct CreateEventParams {
        pub event_name: String,
        pub event_date: i64,
        pub ticket_price: u64,
        pub total_tickets: u64,
    }

    pub fn create_event(ctx: Context<CreateEvent>, event_name: String, event_date: i64, ticket_price: u64, total_tickets: u64) -> Result<()> {
        let event = &mut ctx.accounts.event;
        event.creator = *ctx.accounts.creator.key;
        event.event_name = event_name;
        event.event_date = event_date;
        event.ticket_price = ticket_price;
        event.total_tickets = total_tickets;
        event.tickets_sold = 0;
        Ok(())
    }

    #[derive(Accounts)]
    pub struct MintTicket<'info> {
        #[account(mut)]
        pub event: Account<'info, Event>,
        #[account(mut)]
        pub buyer: Signer<'info>,
        /// CHECK: The `ticket_mint` is an SPL Token Mint account, but is not directly validated in this program.
        /// This account is checked by the SPL Token program, and this program only verifies that it is mutable.
        #[account(mut)]
        pub ticket_mint: AccountInfo<'info>,
        /// CHECK: The `buyer_token_account` is an SPL Token Account, but is not directly validated in this program.
        /// This account is checked by the SPL Token program, and this program only verifies that it is mutable.
        #[account(mut)]
        pub buyer_token_account: AccountInfo<'info>,
        /// CHECK: The `event_token_account` is an SPL Token Account, but is not directly validated in this program.
        /// This account is checked by the SPL Token program, and this program only verifies that it is mutable.
        #[account(mut)]
        pub event_token_account: AccountInfo<'info>,
        pub token_program: Program<'info, Token>,
    }

    pub fn mint_ticket(ctx: Context<MintTicket>, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: ctx.accounts.event_token_account.to_account_info(),
            to: ctx.accounts.buyer_token_account.to_account_info(),
            authority: ctx.accounts.event.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

        token::transfer(cpi_context, amount)?;
        Ok(())
    }
}

#[account]
pub struct Event {
    pub creator: Pubkey,
    pub event_name: String,
    pub event_date: i64,
    pub ticket_price: u64,
    pub total_tickets: u64,
    pub tickets_sold: u64,
}

