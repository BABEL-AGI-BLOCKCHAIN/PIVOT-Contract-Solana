use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

#[program]
mod pivot_topic {
    use super::*;
    
    pub fn initialize(ctx: Context<Initialize>, sbt_address: Pubkey) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.owner = *ctx.accounts.signer.key;
        state.sbt_address = sbt_address;
        state.topic_id = 0;
        state.nonce = 0;
        Ok(())
    }

    pub fn create_topic(ctx: Context<CreateTopic>, amount: u64, topic_hash: [u8; 32]) -> Result<()> {
        let state = &mut ctx.accounts.state;
        let topic = &mut ctx.accounts.topic;
        let token_account = &mut ctx.accounts.token_account;

        require!(amount > 0, CustomError::InsufficientAmount);

        state.topic_id += 1;
        topic.id = state.topic_id;
        topic.promoter = *ctx.accounts.signer.key;
        topic.fixed_investment = amount;
        topic.topic_hash = topic_hash;
        topic.token_mint = ctx.accounts.token_mint.key();
        topic.total_balance = amount;

        // Transfer tokens from promoter to contract
        let cpi_accounts = Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: token_account.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, amount)?;

        emit!(CreateTopicEvent {
            promoter: topic.promoter,
            topic_id: topic.id,
            investment: amount,
            token_mint: topic.token_mint,
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(init, payer = signer, space = 8 + 64)]
    pub state: Account<'info, State>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateTopic<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub state: Account<'info, State>,
    #[account(init, payer = signer, space = 8 + 128)]
    pub topic: Account<'info, Topic>,
    pub token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct State {
    pub owner: Pubkey,
    pub sbt_address: Pubkey,
    pub topic_id: u64,
    pub nonce: u64,
}

#[account]
pub struct Topic {
    pub id: u64,
    pub promoter: Pubkey,
    pub fixed_investment: u64,
    pub topic_hash: [u8; 32],
    pub token_mint: Pubkey,
    pub total_balance: u64,
}

#[error_code]
pub enum CustomError {
    #[msg("Insufficient Amount")]
    InsufficientAmount,
}

#[event]
pub struct CreateTopicEvent {
    pub promoter: Pubkey,
    pub topic_id: u64,
    pub investment: u64,
    pub token_mint: Pubkey,
}
