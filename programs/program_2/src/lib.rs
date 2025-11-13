// Program Summary:
// This program facilitates a presale purchase.
// 1. **Initializes a Program Derived Address (PDA)** named `UserAccount` using the client's public key (`user`) as a seed. 
//    This PDA is used to **store the total quantity of presale units (tokens_assigned)** purchased by the client.
// 2. **Executes a Cross-Program Invocation (CPI)** to the native System Program to transfer the calculated amount of native SOL (lamports) 
//    from the **Client's wallet ('signer')** into the designated **Vault Wallet**.

// change directory from root to programs/program_2 then run ***anchor build***
// go back to root then run ***solana program deploy target/deploy/program_2.so

#![allow(unexpected_cfgs)] // Compiler flag to ignore warnings about unexpected configurations

use anchor_lang::prelude::*;
use anchor_lang::system_program; // Anchor's idiomatic wrapper for the System Program

declare_id!("FgVJ3gHPMw5ZCgZYyY4gExtdYQYnma4gG7Z3QzaF6DSN"); // Program's unique address (ID) on the Solana network

#[program]
pub mod presale_program {

    use super::*;

    /// Instruction to initialize a UserAccount PDA and receive SOL from the user
    pub fn buy_presale_tokens(ctx: Context<CreateUserAccount>, amount: u64) -> Result<()> {
        
        let user_account = &mut ctx.accounts.user_account; // Get mutable reference to the UserAccount PDA

        // Update the program state by increasing the counter
        user_account.tokens_assigned += amount; 
        
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.signer.to_account_info(),
                to: ctx.accounts.vault_wallet.to_account_info(),
            },
        );
        
        system_program::transfer(cpi_context, (amount * 10_000_000).into())?;

        Ok(()) // Return success
    }
}

// Account validation and context structure for the instruction
#[derive(Accounts)]
pub struct CreateUserAccount<'info> {
    // wert wallet that transfers and pays for transaction fee
    #[account(mut)]
    pub signer: Signer<'info>, 
    
    /// The user account whose key is used as the PDA seed. Must be a SystemAccount.
    pub user: SystemAccount<'info>, 

    /// The program's Vault/Treasury account receiving the SOL
    #[account(mut)]
    pub vault_wallet: SystemAccount<'info>,
    
    /// The program's custom data account (PDA)
    #[account(
        init_if_needed, // Initialize the account if it doesn't exist
        payer = signer, // The 'signer' pays the rent for this PDA
        space = 8 + UserAccount::INIT_SPACE, // Allocates 8 bytes (discriminator) + space for the struct
        seeds = [user.key().as_ref()], // Derives the address using the 'user' key as a seed
        bump // Stores the PDA's bump seed
    )]
    pub user_account: Account<'info, UserAccount>,
    
    /// The native Solana System Program account (required for creating accounts and transfers)
    pub system_program: Program<'info, System>,
}

// --- DATA STRUCTURE ---

// Data structure for the UserAccount PDA
#[account]
#[derive(InitSpace)]
pub struct UserAccount {
    pub tokens_assigned: u64, // A counter field to track assigned tokens/amounts
}