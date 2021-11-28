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

  pub fn add_metadata(ctx: Context<AddMetadata>, metadata: Pubkey, index: u8) -> ProgramResult {
    let page = &mut ctx.accounts.page;

    page.metadata.insert(index.into(), metadata);

    Ok(())
  }

  pub fn drop_metadata(ctx: Context<DropMetadata>, metadata: Pubkey) -> ProgramResult {
    let page =&mut ctx.accounts.page;

    page.metadata.retain(|key| key != &metadata);
    Ok(())
  }

  pub fn swap_metadata(ctx: Context<SwapMetadata>, head: Pubkey, tail: Pubkey) -> ProgramResult {
    let page = &mut ctx.accounts.page;
    let previous = page.metadata.clone();
    let head_index = previous.iter().position(|a| a == &head).unwrap();
    let tail_index = previous.iter().position(|a| a == &tail).unwrap();

    page.metadata[head_index] = tail;
    page.metadata[tail_index] = head;
    
    Ok(())
  }
}

#[derive(Accounts)]
pub struct CreatePage<'info> {
  #[account(init, payer = maintainer, space = 32 + 8 + 32*100)]
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
pub struct DropMetadata<'info> {
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
  metadata: Vec<Pubkey>,
}