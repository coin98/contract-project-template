use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct HelloContext<'info> {

  /// CHECK: For warning removal
  pub owner: AccountInfo<'info>,
}
