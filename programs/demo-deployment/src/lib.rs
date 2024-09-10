use anchor_lang::prelude::*;

declare_id!("RL9Bcs7ypX4ydkBcwL6qMVxLJr5yRdWx3wFCmyCddw8");

#[program]
pub mod demo_deployment {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
