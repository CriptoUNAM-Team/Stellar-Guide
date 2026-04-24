# NFT Membership Contract (Soroban)

Contrato base para emitir NFTs de membresía/certificados.

## Funciones

- `initialize(admin)`
- `mint(to, metadata_uri) -> token_id`
- `transfer(token_id, to)`
- `get_token(token_id)`

## Uso

```bash
cd contracts/nft-membership
make test
```
