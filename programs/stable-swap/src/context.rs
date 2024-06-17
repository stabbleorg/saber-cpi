use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Swap<'info> {
    /// The context of the user.
    pub user: SwapUserContext<'info>,
    /// Accounts for input tokens.
    pub input: SwapInput<'info>,
    /// Accounts for output tokens.
    pub output: SwapOutput<'info>,
    /// The spl_token program.
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SwapUserContext<'info> {
    /// The swap.
    pub swap: AccountInfo<'info>,
    /// The authority of the swap.
    pub swap_authority: AccountInfo<'info>,
    /// The authority of the user.
    #[account(signer)]
    pub user_authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SwapInput<'info> {
    /// The token account associated with the user.
    #[account(mut)]
    pub user: AccountInfo<'info>,
    /// The token account for the pool's reserves of this token.
    #[account(mut)]
    pub reserve: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SwapOutput<'info> {
    /// The token account for the pool's reserves of this token.
    #[account(mut)]
    pub reserve: AccountInfo<'info>,
    /// The token account associated with the user.
    #[account(mut)]
    pub user: AccountInfo<'info>,
    /// The token account for the fees associated with the token.
    #[account(mut)]
    pub fees: AccountInfo<'info>,
}
