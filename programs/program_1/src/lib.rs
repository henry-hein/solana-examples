use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("Hzh24Pda2aB3BpNeKSZkKEQxtqnZGxbq4NHwN2UNSAcA");

#[program]
pub mod solana_deposit_example {
    use super::*;

    /// Deposits SOL from the caller (hot wallet) into the recipient wallet address on behalf of a specific user.
    pub fn deposit_sol(
        ctx: Context<DepositSol>,
        user_pubkey: Pubkey, // The actual user's address
        amount: u64,
    ) -> Result<()> {
        // Transfer SOL from caller (hot wallet) to the recipient wallet address
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.caller.to_account_info(),
                to: ctx.accounts.recipient_address.to_account_info(),
            },
        );
        system_program::transfer(cpi_context, amount)?;

        // Emit event, using the instruction parameter (user_pubkey) to identify the depositor
        emit!(SwapRequest {
            user: user_pubkey, // Using the passed-in user_pubkey
            recipient_address: ctx.accounts.recipient_address.key(),
            amount,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }
}

// Account Validation Structs

/// Account validation for depositing SOL.
#[derive(Accounts)]
pub struct DepositSol<'info> {
    // The recipient wallet address where funds are sent
    #[account(mut)]
    pub recipient_address: SystemAccount<'info>,

    // The sender of the SOL (the hot wallet/signer)
    #[account(mut)]
    pub caller: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// Data Structures

// Event & Error Codes

/// Event emitted when a deposit is made, used by the off-chain relayer service.
#[event]
pub struct SwapRequest {
    pub user: Pubkey, // The actual user's address is recorded here
    pub recipient_address: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized access")]
    Unauthorized,
}
