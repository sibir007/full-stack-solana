use anchor_lang::prelude::*;

declare_id!("HBq27XcCHHCyXbaVwacJyk6DpDTKfa2AnrcDrYEr48x");

#[program]
pub mod full_stack_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
