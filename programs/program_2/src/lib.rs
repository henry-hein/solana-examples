// Program Summary:
// This program facilitates a presale purchase.
// 1. **Initializes a Program Derived Address (PDA)** named `UserAccount` using the client's public key (`user`) as a seed. 
//    This PDA is used to **store the total quantity of presale units (tokens_assigned)** purchased by the client.
// 2. **Executes a Cross-Program Invocation (CPI)** to the native System Program to transfer the calculated amount of native SOL (lamports) from the **3rd-party service's wallet ('signer')** into the designated **Vault Wallet**.

#![allow(unexpected_cfgs)] // Compiler flag to ignore warnings about unexpected configurations

use anchor_lang::prelude::*;
use anchor_lang::solana_program::{self, system_instruction}; // Import the native Solana program library and system instructions

declare_id!("FgVJ3gHPMw5ZCgZYyY4gExtdYQYnma4gG7Z3QzaF6DSN"); // Program's unique address (ID) on the Solana network

#[program]
pub mod presale_program {

    use super::*;

    /// Instruction to initialize a UserAccount PDA and transfer SOL to the user
    pub fn buy_presale_tokens(ctx: Context<CreateUserAccount>, amount: u32) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account; // Get mutable reference to the UserAccount PDA
        let sender = &mut ctx.accounts.signer; // Get mutable reference to the transaction signer (SOL source)
        let vault_wallet = &mut ctx.accounts.vault_wallet; // Get mutable reference to the recipient account (SOL destination)

        // Update the program state by increasing the counter
        user_account.tokens_assigned += amount; 

        // 1. Construct the native Solana System Program transfer instruction
        let instruction =
            system_instruction::transfer(
                &sender.key(), // Source of SOL (Signer)
                &vault_wallet.key,     // Destination of SOL (User)
                (amount * 10_000_000).into(), // Calculate lamport amount (amount * 0.01 SOL)
            );

        // 2. Perform a Cross-Program Invocation (CPI) to the System Program
        solana_program::program::invoke(
            &instruction, // The transfer instruction created above
            &[
                // Accounts required by the System Program transfer instruction:
                sender.to_account_info(), // The source account (must be mutable and signer)
                vault_wallet.to_account_info(), // The destination account (must be mutable)
                ctx.accounts.system_program.to_account_info(), // The System Program itself
            ],
        )?; // The '?' operator handles errors from the invocation

        Ok(()) // Return success
    }
}

// Account validation and context structure for the instruction
#[derive(Accounts)]
pub struct CreateUserAccount<'info> {
    /// CHECK: The signer pays for the transaction and the SOL transfer
    #[account(mut)]
    pub signer: Signer<'info>, 
    
    pub user: SystemAccount<'info>, 

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

// Data structure for the UserAccount PDA
#[account]
#[derive(InitSpace)]
pub struct UserAccount {
    pub tokens_assigned: u32, // A counter field to track assigned tokens/amounts
}