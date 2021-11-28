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
  const head = new PublicKey("ar33eNRegXzzC6gb5GXX7YfKhwCbv3snQJLz4Y5nhys");
  const tail = new PublicKey("DJPBUT1DucpYVjP2TpJwmbVmCdVW2kj4HZVfFhkh4nY5");

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

  it('adds metadata to a page', async () => {
    await program.rpc.addMetadata(tail, 0, {
      accounts: {
        page: pageKeypair.publicKey,
        maintainer: provider.wallet.publicKey,
      },
    }); 

    await program.rpc.addMetadata(head, 0, {
      accounts: {
        page: pageKeypair.publicKey,
        maintainer: provider.wallet.publicKey,
      },
    });

    const page = await program.account.metadataPage.fetch(pageKeypair.publicKey)

    assert.ok(page.metadata[0].equals(head));
    assert.ok(page.metadata[1].equals(tail));
  });

  it('swaps metadata positions for a page', async () => {
    await program.rpc.swapMetadata(tail, head, {
      accounts: {
        page: pageKeypair.publicKey,
        maintainer: provider.wallet.publicKey,
      },
    });

    const page = await program.account.metadataPage.fetch(pageKeypair.publicKey)

    assert.ok(page.metadata[0].equals(tail));
    assert.ok(page.metadata[1].equals(head));
  })

  it('drops metadata from a page', async () => {
    await program.rpc.dropMetadata(tail, {
      accounts: {
        page: pageKeypair.publicKey,
        maintainer: provider.wallet.publicKey,
      },
    })  

    const page = await program.account.metadataPage.fetch(pageKeypair.publicKey)

    assert.ok(page.metadata[0].equals(head));
  });
});
