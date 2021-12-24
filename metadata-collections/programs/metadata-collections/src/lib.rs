use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod metadata_collections {
  use super::*;

  pub fn create_page(ctx: Context<CreatePage>, position: u64) -> ProgramResult {
    let maintainer = ctx.accounts.maintainer.clone();
    let page = &mut ctx.accounts.page;

    page.maintainer = maintainer.key();
    page.position = position;
    page.metadata = Vec::new();

    Ok(())
  }

  pub fn replace_metadata(ctx: Context<AddMetadata>, metadata: Vec<MetadataEntry>) -> ProgramResult {
    let page = &mut ctx.accounts.page;

    page.metadata = metadata.clone();

    Ok(())
  }
}

#[derive(Accounts)]
pub struct CreatePage<'info> {
  #[account(init, payer = maintainer, space = 8 + 32 + 8 + 4 + (8 + 32) * 30)]
  pub page: Account<'info, MetadataPage>,
  #[account(mut)]
  pub maintainer: Signer<'info>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddMetadata<'info> {
  #[account(mut, has_one = maintainer)]
  pub page: Account<'info, MetadataPage>,
  pub maintainer: Signer<'info>,
}

#[derive(Accounts)]
pub struct SwapMetadata<'info> {
  #[account(mut, has_one = maintainer)]
  pub page: Account<'info, MetadataPage>,
  pub maintainer: Signer<'info>,
}

#[account]
pub struct MetadataPage {
  maintainer: Pubkey,
  position: u64,
  metadata: Vec<MetadataEntry>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct MetadataEntry {
    pub address: Pubkey,
}