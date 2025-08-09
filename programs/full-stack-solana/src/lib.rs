use anchor_lang::prelude::*;

declare_id!("HBq27XcCHHCyXbaVwacJyk6DpDTKfa2AnrcDrYEr48x");

#[program]
pub mod full_stack_solana {
    use anchor_lang::solana_program::entrypoint::ProgramResult;

    use super::*;

    pub fn create(ctx: Context<Create>) -> ProgramResult {
        let base_account = &mut ctx.accounts.my_account;
        base_account.count = 0;
        msg!("Creating a new account with the provided data");
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> ProgramResult {
        msg!("Incrementing the value in the account");
        Ok(())
    }
    

}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 8 + 32)]
    pub my_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub my_account: Account<'info, BaseAccount>,
}

#[account]
pub struct BaseAccount {
    pub count: u64,
}

