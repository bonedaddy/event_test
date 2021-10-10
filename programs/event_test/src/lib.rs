use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod event_test {
    use super::*;
    pub fn initialize_a(ctx: Context<Initialize>) -> ProgramResult {
        emit!(HelloWorld{
            message: "foobar".to_string(),
            number: 123
        });
        Ok(())
    }
    pub fn initialize_b(ctx: Context<Initialize>) -> ProgramResult {
        msg!("{{\"message\": \"{}\", \"number\": {}}}", "foobar", 123);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}


#[event]
pub struct HelloWorld {
    pub message: String,
    pub number: u64
}

