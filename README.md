# Demo blockchain (bloques + mempool + NICOCIN)

Rust (Axum) + React (Vite, TypeScript, Tailwind). Cadena de **5 bloques** con PoW (`0000…`), **mempool**, **transacciones simples** y **coinbase NICOCIN** al minar. Estado en memoria.

## Usuarios

- **Nico** (`nico`), **Martin** (`martin`), **Sofía** (`sofia`) — saldo inicial 1000 cada uno.

## Coinbase

Al minar un bloque se añade una transacción especial con `label` y `from` **NICOCIN**, `to` = minero (query `miner_id`), cantidad fija **50**.

## Hash del bloque

`sha256("{index}|{nonce}|{data}|{previous_hash}|{json(coinbase,transactions)}")`

## API (`/api`)

| Método | Ruta | Descripción |
|--------|------|-------------|
| GET | `/blocks` | Cadena |
| GET | `/users` | Usuarios y saldos |
| GET | `/mempool` | Pendientes |
| POST | `/transactions` | `{ "sender", "recipient", "amount" }` (JSON; evita que traductores rompan el campo `to`) |
| PUT | `/blocks/:index` | `{ "data"?, "nonce"? }` |
| POST | `/blocks/:index/mine?miner_id=nico` | Incluye mempool + NICOCIN, PoW, re-enlaza siguientes |
| GET | `/validate` | Validez estructural + libro |
| POST | `/reset` | Reinicio |

## Ejecutar

```bash
cd backend && cargo run
cd frontend && npm install && npm run dev
```

## Flujo manual

1. Crear transacciones → aparecen en **mempool**.
2. Elegir **minero** (Nico/Martin/Sofía) y pulsar **Mine** en un bloque: se vacía la mempool en ese bloque, se añade **NICOCIN** y se recalcula el hash válido.
3. Editar **data/nonce** rompe PoW o enlaces; **Mine** en bloques afectados para recuperar validez.
4. **Reset** restaura cadena inicial y mempool vacío.
