import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { MetadataCollections } from '../target/types/metadata_collections';
import { PublicKey } from '@solana/web3.js';
import assert from 'assert';

const { SystemProgram } = anchor.web3;

describe('Metaplex Metadata Collections', () => {
  const provider = anchor.Provider.local();

  anchor.setProvider(provider);

  const program = anchor.workspace.MetadataCollections as Program<MetadataCollections>;
  const pageKeypair = anchor.web3.Keypair.generate();
  let metadata: PublicKey[] = [];

  before(() => {
    for (let i = 0; i < 30; i++) {
      metadata.push(new PublicKey("ar33eNRegXzzC6gb5GXX7YfKhwCbv3snQJLz4Y5nhys"));
    }
  });

  it('creates a collection page', async () => {
    await program.rpc.createPage(new anchor.BN(0), {
      accounts: {
        page: pageKeypair.publicKey,
        maintainer: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [pageKeypair],
    })

    const page = await program.account.metadataPage.fetch(pageKeypair.publicKey)

    assert.ok(page.maintainer.equals(provider.wallet.publicKey))
    assert.ok(page.position.toNumber() === 0);
    assert.ok(page.metadata.length === 0)
  });

  it('replaces metadata to a page', async () => {
    await program.rpc.replaceMetadata(metadata, {
      accounts: {
        page: pageKeypair.publicKey,
        maintainer: provider.wallet.publicKey,
      },
    });

    const page = await program.account.metadataPage.fetch(pageKeypair.publicKey)
    assert.ok(page.metadata.length === 30);
  });
});
