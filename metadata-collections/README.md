# Metadata Collection

Assemble a collection of metadata by some maintainer wallet. The collection is made up of pages whose derived address is computed from the maintainer's pubkey and page position.

The maintainer wallet could be a curator looking to showcase NFTs for sell via the Auction House program or a collection of metadata minted by a creator from a Metaplex storefront. 

```rust
# pda ["maintainer_pubkey", "page"]
pub struct MetadataPage {
  maintainer: Pubkey,
  position: u64,
  # max 30
  metadata: Vec<Pubkey>,
}
```

The client sdk helps web3 clients interact with the collection. It supports finding metdata for a page as well as updating the collection.

__Waringing: This is a draft of the sdk for the program. It is not currently implemented within `@metaplex/js`. It is open for discussion.__

```javascript
import { programs } from '@metaplex/js';
const { collection: { MetadataCollection }, metadata: { Metdata }  } = programs;

# setup a collection context for a given mainter wallet.
const collection = await MetadataCollection.forAuhority(wallet.provider.publicKey);
let metadatas: Metadata[];

# position based on number of exisiting pages starting at 0.
const page = await collection.addPage();

# add metadata to the collection. Optionally accepts an index. 
await collection.add('<metadata_pubkey>');

# saves the collection on-chain updating only the pages altered by collection mutations.
# The wallet passed to persist must be the maintainer of the collection.
await collection.persist(wallet);

# fetch the first page from the cache using getMultpleAccountInfo and deserializing with Metadata.from
metadatas = await collection.getPage(0);

console.log(metadata.length); # 1

# drop metadata from the collection
colection.remove(metadata);

# save the updated collection on-chain.
await collection.persist(wallet);

metadatas = await collection.getPage(0);

console.log(metadata.length) # 0
```
## Instructions

- Create Metadata Page

Adds a page to the maintainer's collection.

- Swap Metadata for Page

Updates the whole list of metadata associated to the collection page.

## Benefits

- Perdictable address structure for querying groups of metadata without using getProgramAccounts which is slow and deminishis in performance with the number accounts under management by the program.
- On-chain record of metadata associated to a collection that doesn't rely on collection attributes within the Metadata json.

## Potential Enhancements

- A user can manage multiple collections without switching wallets.
- Collection schema specification for descriptive information about the metadata like website, rarity scores, etc. The maintainer is able to switch the reference unless it is locked. 
- Locking a collection closes it for change and provides an on-chain record of metadata associated to the collection.
