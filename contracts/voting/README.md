# voting — Votación sí/no (campus)

Propuestas abiertas por el admin. Un voto por address. Cierre y consulta de resultados.

Sirve para enseñar: un voto = una clave, no hay “doble voto”, el resultado es auditable.

## Funciones

- `initialize(admin)`
- `create_proposal(title) -> proposal_id`
- `vote(proposal_id, voter, support)`
- `close_proposal(proposal_id)`
- `get_proposal(proposal_id)`

## Lab (sesión 2)

Comandos de deploy/invoke: `docs/comandos-basicos.md`.

```bash
cd contracts/voting
make test
```
