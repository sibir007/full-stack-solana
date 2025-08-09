use anchor_lang::prelude::*;

declare_id!("J5RWjigXEQfW3WrykWAsJvb58MqhDM8pqJBzVaWEFiAs");



pub const SEED_COUNTER: &[u8] = b"counter";

#[account]
pub struct Counter {
    pub current_value: u64,
    pub updated_at: i64,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut, seeds = [SEED_COUNTER], bump)]
    pub counter: Account<'info, Counter>,

    #[account(seeds = [THREAD_AUTHORITY_SEED], bump)]
    pub thread_authority: SystemAccount<'info>,


    #[account(signer, constrain = thread.authority.eq(&thread_authority.key()))]
    pub thread: Account<'info, Thread>,

}


#[program]
pub mod counter {
    use super::*;
    
    // 2️⃣ We define an instruction to mutate the `Counter`
    pub fn increment(ctx: Context<Increment>) -> Result<()> {    
        ctx.accounts.counter.current_value = ctx.accounts.counter.current_value.checked_add(1).unwrap();
        ctx.accounts.counter.updated_at = Clock::get().unwrap().unix_timestamp;
        Ok(())
    }
}