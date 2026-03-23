# Simulación de blockchain (demo educativa)

Monorepo con **backend en Rust (Axum)** y **frontend en React + Vite + TypeScript + Tailwind**. Estado **solo en memoria** (sin base de datos). La API expone cuentas, mempool, bloques, minería PoW simplificada y validación de cadena.

## Requisitos

- [Rust](https://rustup.rs/) (stable) y `cargo`
- [Node.js](https://nodejs.org/) 18+ (probado con Vite 5 y Tailwind 3)

## Cómo ejecutar

### Backend (puerto 3000)

```bash
cd backend
cargo run
```

Deberías ver: `blockchain_demo API listening on http://127.0.0.1:3000`.

### Frontend (puerto 5173, proxy `/api` → 3000)

En otra terminal:

```bash
cd frontend
npm install
npm run dev
```

Abre la URL que indique Vite (normalmente `http://127.0.0.1:5173`). El frontend llama a la API vía proxy, así que el backend debe estar en marcha.

## API (prefijo `/api`)

| Método | Ruta | Descripción |
|--------|------|-------------|
| GET | `/api/health` | Estado del servicio |
| GET | `/api/accounts` | Lista de cuentas |
| GET | `/api/accounts/:id` | Detalle de cuenta |
| GET | `/api/transactions/pending` | Mempool |
| POST | `/api/transactions` | Crear transacción (body JSON `from_account_id`, `to_account_id`, `amount`) |
| GET | `/api/blocks` | Todos los bloques |
| GET | `/api/blocks/:index` | Bloque por índice |
| POST | `/api/mine` | Minar (body JSON `miner_account_id`, opcional `difficulty`, `max_transactions`) |
| GET | `/api/blockchain/validate` | Validar estructura + PoW + replay económico |
| POST | `/api/reset-demo` | Reiniciar demo (génesis + cuentas iniciales) |

## Fases del proyecto

- **Fase 1–2** (este repo): esqueleto + backend funcional + dashboard mínimo con datos reales.
- **Fase 3–4**: pulir UI (badges, loaders, tamper, dificultad en UI, etc.).
- **Fase 5**: documentación técnica ampliada en `docs/` (opcional).

## Arquitectura del backend

- `app_state.rs`: estado compartido (`Arc<RwLock<DemoState>>`), configuración y bloque génesis.
- `models/`: dominio + DTOs (`dto.rs`).
- `services/`: minería, cuentas, consultas de cadena.
- `routes/`: handlers HTTP delgados.
- `utils/hashing.rs`: payload canónico (JSON) + SHA-256 hex.
- `utils/validation.rs`: validación de transacciones, bloques y cadena completa.

## Commits sugeridos (si inicializas git)

1. `chore: scaffold monorepo (backend + frontend)`
2. `feat(api): blockchain simulation, PoW, mempool, REST`
