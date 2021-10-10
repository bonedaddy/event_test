use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod event_test {
    use super::*;
    pub fn initialize_a(ctx: Context<Initialize>) -> ProgramResult {
        emit!(HelloWorld{
            message: "foobar".to_string(),
            number: 123,
            such: "baz".to_string(),
            compute_units: "bar".to_string(),
            very_wow: true,
        });
        Ok(())
    }
    pub fn initialize_b(ctx: Context<Initialize>) -> ProgramResult {
        msg!("{{\"message\": \"{}\", \"number\": {}, \"such\": \"{}\", \"compute_units\": \"{}\", \"very_wow\": \"{}\"}}", "foobar", 123, "baz", "bar", true);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}


#[event]
pub struct HelloWorld {
    pub message: String,
    pub number: u64,
    pub such: String,
    pub compute_units: String,
    pub very_wow: bool,
}

