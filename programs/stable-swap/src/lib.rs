//! Anchor-compatible SDK for the stable-swap program.
#![allow(unused)]

pub mod context;

use crate::context::*;
use anchor_lang::prelude::*;

declare_id!("SSwpkEEcbUqx4vtoEByFjSkhKdCT862DNVb52nZg1UZ");

#[program]
pub mod stable_swap {
    use super::*;

    pub fn swap(ctx: Context<Swap>, amount_in: u64, minimum_amount_out: u64) -> Result<()> {
        Ok(())
    }
}
