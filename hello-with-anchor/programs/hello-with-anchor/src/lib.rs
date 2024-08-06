use anchor_lang::prelude::*;

declare_id!("9EKuUwB6mHov7xzpsW2Mu6uVhFJWJPSK1S3wA6xCvqew");

#[program]
pub mod hello_with_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
