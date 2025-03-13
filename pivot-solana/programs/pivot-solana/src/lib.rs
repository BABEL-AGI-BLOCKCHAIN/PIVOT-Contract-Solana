use anchor_lang::prelude::*;

declare_id!("Ey6jTbZuCCdTa6EbibHLFwtfrHnWHop9BJM4UZw2znxc");

#[program]
pub mod pivot_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
