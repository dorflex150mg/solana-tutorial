use anchor_lang::prelude::*;

declare_id!("AZ6jm7itX4h4w7hE8MkyK59QtE1yR9PUCrKEEeFu6PGG");

#[program]
pub mod temp_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
