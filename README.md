# AI Finance Tracker

A multi-workspace personal finance tracker powered by AI. Upload bank statements, track transactions, and receive AI-generated financial insights. Built in Rust, compiled to WebAssembly, and deployed serverlessly on Cloudflare Workers.

---

## Features

- **JWT authentication** — register, login, 24-hour token expiry, PBKDF2 password hashing
- **Multi-workspace** — users can create and belong to multiple workspaces with role-based access (owner / admin / member)
- **Transaction management** — manual entry with filtering by category, type, and date range
- **Bank statement ingestion** — upload JPEG, PNG, or PDF statements; DeepSeek AI parses transactions automatically
- **AI financial insights** — generate spending tips, reduction targets, and purchase recommendations over any date range
- **File storage** — raw statement files stored in Cloudflare R2
- **Interactive API docs** — Swagger UI at `/api/docs`, raw OpenAPI JSON at `/api/docs/openapi.json`
- **Zero cold-start infrastructure** — runs entirely on Cloudflare Workers + D1 + R2, no servers to manage

---

## Architecture

Clean architecture with strict dependency direction: domain has no external dependencies; each outer layer depends only on the layer directly beneath it.

```
┌─────────────────────────────────────────────────────┐
│                   presentation/                      │
│   handlers · middleware · docs (Swagger UI)          │
├─────────────────────────────────────────────────────┤
│                   application/                       │
│   use cases: register, login, create-workspace,      │
│   invite-member, create-tx, upload-statement,        │
│   generate-insights                                  │
├─────────────────────────────────────────────────────┤
│                 infrastructure/                      │
│   D1 repositories · R2 storage · JWT · PBKDF2 ·     │
│   DeepSeek HTTP client                               │
├─────────────────────────────────────────────────────┤
│                     domain/                          │
│   entities: User · Workspace · Transaction ·         │
│   BankStatement · AiInsight                          │
│   repository traits (pure Rust, no I/O)             │
└─────────────────────────────────────────────────────┘
         compiled to wasm32-unknown-unknown
                       │
              Cloudflare Workers runtime
```

---

## Tech Stack

| Concern | Technology |
|---|---|
| Language | Rust (edition 2021) |
| Compilation target | `wasm32-unknown-unknown` via `worker-build` |
| Runtime | Cloudflare Workers |
| Database | Cloudflare D1 (SQLite-compatible) |
| File storage | Cloudflare R2 |
| AI / LLM | DeepSeek API (`deepseek-chat`, OpenAI-compatible) |
| Auth | JWT (`jwt-simple`, pure-Rust) + PBKDF2 (`pbkdf2` crate) |
| Serialization | `serde` + `serde_json` |
| UUIDs | `uuid` v4 (WASM-compatible via `js` feature) |
| Time | `chrono` with `wasmbind` feature |
| Deployment | Wrangler CLI |

---

## API Endpoints

All endpoints are prefixed with `/api/v1`. Routes marked **Auth** require `Authorization: Bearer <token>`.

| Method | Path | Auth | Description |
|---|---|---|---|
| `POST` | `/api/v1/auth/register` | No | Register a new user, returns JWT |
| `POST` | `/api/v1/auth/login` | No | Login, returns JWT |
| `GET` | `/api/v1/workspaces` | Yes | List workspaces the caller belongs to |
| `POST` | `/api/v1/workspaces` | Yes | Create a new workspace |
| `GET` | `/api/v1/workspaces/:id` | Yes | Get workspace by ID |
| `PUT` | `/api/v1/workspaces/:id` | Yes | Update workspace (owner / admin) |
| `DELETE` | `/api/v1/workspaces/:id` | Yes | Delete workspace (owner only) |
| `GET` | `/api/v1/workspaces/:id/members` | Yes | List workspace members |
| `POST` | `/api/v1/workspaces/:id/members` | Yes | Invite a user by email (owner / admin) |
| `GET` | `/api/v1/workspaces/:workspace_id/transactions` | Yes | List transactions (filterable by category, type, date) |
| `POST` | `/api/v1/workspaces/:workspace_id/transactions` | Yes | Create a transaction manually |
| `GET` | `/api/v1/workspaces/:workspace_id/transactions/:id` | Yes | Get a single transaction |
| `DELETE` | `/api/v1/workspaces/:workspace_id/transactions/:id` | Yes | Delete a transaction |
| `GET` | `/api/v1/workspaces/:workspace_id/statements` | Yes | List bank statements |
| `POST` | `/api/v1/workspaces/:workspace_id/statements` | Yes | Upload a bank statement (image or PDF); AI parses it |
| `GET` | `/api/v1/workspaces/:workspace_id/statements/:id` | Yes | Get a single bank statement |
| `GET` | `/api/v1/workspaces/:workspace_id/insights` | Yes | List AI insights (filterable by type) |
| `POST` | `/api/v1/workspaces/:workspace_id/insights/generate` | Yes | Generate AI insights from transactions in a date range |
| `GET` | `/api/docs/openapi.json` | No | Raw OpenAPI 3.0 spec |
| `GET` | `/api/docs` | No | Swagger UI |

**Transaction list query parameters:** `category`, `type` (`income`\|`expense`), `from` (date), `to` (date), `limit` (default 50), `offset` (default 0).

**Statement upload:** send raw file bytes as the request body. Set `Content-Type` to `image/jpeg`, `image/png`, or `application/pdf`. Pass the original filename in the `X-File-Name` header.

**Insight types:** `tip` · `reduction` · `recommendation` · `analysis`

---

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [Wrangler CLI](https://developers.cloudflare.com/workers/wrangler/install-and-update/) v3+
- `worker-build` (installed automatically on first deploy, or manually):

```bash
cargo install worker-build
```

### Clone and configure

```bash
git clone https://github.com/your-org/ai-finance-tracker.git
cd ai-finance-tracker
```

Open `wrangler.toml` and replace the placeholder `database_id` with your real D1 database ID (created in the next step).

### Create the D1 database

```bash
wrangler d1 create ai-finance-tracker
```

Copy the `database_id` from the output into `wrangler.toml`.

### Run migrations

```bash
wrangler d1 execute ai-finance-tracker --file=migrations/0001_create_users.sql
wrangler d1 execute ai-finance-tracker --file=migrations/0002_create_workspaces.sql
wrangler d1 execute ai-finance-tracker --file=migrations/0003_create_transactions.sql
wrangler d1 execute ai-finance-tracker --file=migrations/0004_create_bank_statements.sql
wrangler d1 execute ai-finance-tracker --file=migrations/0005_create_ai_insights.sql
```

### Create the R2 bucket

```bash
wrangler r2 bucket create ai-finance-tracker-storage
```

### Set secrets

```bash
wrangler secret put JWT_SECRET       # enter a long random string
wrangler secret put DEEPSEEK_API_KEY # enter your DeepSeek API key
```

### Deploy

```bash
wrangler deploy
```

Wrangler will compile the crate to WASM and upload the Worker. The API will be live at your `*.workers.dev` subdomain.

---

## Environment and Secrets

| Name | Where | Description |
|---|---|---|
| `JWT_SECRET` | Wrangler secret | Signing key for JWT tokens |
| `DEEPSEEK_API_KEY` | Wrangler secret | DeepSeek API key for AI features |
| `ENVIRONMENT` | `wrangler.toml` `[vars]` | Runtime environment label (default: `production`) |
| `DB` | D1 binding | Cloudflare D1 database |
| `STORAGE` | R2 binding | Cloudflare R2 bucket for statement files |

---

## Project Structure

```
src/
├── lib.rs                        # Worker entry point, route table
├── error.rs                      # Unified error type
├── domain/
│   ├── user/                     # User entity + repository trait
│   ├── workspace/                # Workspace + member entity + trait
│   ├── transaction/              # Transaction entity + trait
│   ├── bank_statement/           # BankStatement entity + trait
│   └── ai_insight/               # AiInsight entity + trait
├── application/
│   ├── auth/                     # register, login use cases
│   ├── workspace/use_cases/      # create, invite use cases
│   ├── transaction/use_cases/    # create use case
│   ├── bank_statement/use_cases/ # upload + AI parse use case
│   └── ai_insight/use_cases/     # generate insights use case
├── infrastructure/
│   ├── repository/               # D1 implementations of all traits
│   ├── auth/                     # JWT issuance/validation, PBKDF2
│   ├── ai/                       # DeepSeek HTTP client
│   └── storage/                  # R2 upload/download
└── presentation/
    ├── auth/handlers.rs
    ├── workspace/handlers.rs
    ├── transaction/handlers.rs
    ├── bank_statement/handlers.rs
    ├── ai_insight/handlers.rs
    ├── docs/                     # OpenAPI spec + Swagger UI handler
    └── middleware.rs             # JWT extraction middleware
```

---

## Development Notes

The project targets `wasm32-unknown-unknown`. Standard `cargo build` will not work unless the target is installed.

**Type-check without compiling to WASM:**

```bash
cargo check --target wasm32-unknown-unknown
```

**Local development with Wrangler (uses Miniflare under the hood):**

```bash
wrangler dev
```

D1 and R2 are emulated locally by Wrangler. Secrets can be placed in a `.dev.vars` file (never commit this file):

```ini
JWT_SECRET=your-local-secret
DEEPSEEK_API_KEY=your-deepseek-key
```

**Adding a migration:** create the next numbered file in `migrations/`, then run `wrangler d1 execute` as shown above. For remote production databases, add `--remote` to the command.

---

## Database Schema

| Table | Purpose |
|---|---|
| `users` | Accounts with hashed passwords |
| `workspaces` | Named workspaces owned by a user |
| `workspace_members` | Many-to-many with roles (`owner`, `admin`, `member`) |
| `transactions` | Financial records, manual or imported from statements |
| `bank_statements` | Uploaded files metadata + AI-parsed transactions (JSON) |
| `ai_insights` | AI-generated insights stored per workspace |

---

## License

MIT
