use anchor_lang::prelude::*;

declare_id!("AZ6jm7itX4h4w7hE8MkyK59QtE1yR9PUCrKEEeFu6PGG");

#[program]
pub mod temp_project {
    use super::*;

}

#[account]
#[derive(InitSpace)]
pub struct Counter {
    pub count: u64,
}

const DISCRIMNATOR: usize = 8;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init,
        payer = user,
        space = DISCRIMNATOR + Counter::INIT_SPACE
    )]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}


