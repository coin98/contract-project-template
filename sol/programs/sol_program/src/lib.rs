pub mod constant;
pub mod context;
pub mod error;
pub mod event;
pub mod state;

use anchor_lang::prelude::*;
use context::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
mod sol_program {
  use super::*;

  pub fn hello(
    _ctx: Context<HelloContext>,
  ) -> Result<()> {

    Ok(())
  }
}
