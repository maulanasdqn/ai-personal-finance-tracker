# AI Finance Tracker

Most personal finance apps ask you to do the work: manually categorize every purchase, export CSVs, paste them into spreadsheets, and stare at charts that tell you what you already know. We want to flip that.

The goal of this project is a finance assistant that does the heavy lifting for you. Upload a photo of a receipt or a screenshot of your bank statement — the AI reads it, extracts the transactions, and files them. At the end of the month, ask it what you spent too much on and what you should cut. It tells you, concretely, not in vague pie charts.

Beyond personal use, the workspace model means you can share a financial view with a partner, a small business co-founder, or an accountant — everyone sees the same numbers, with role-based access controlling who can edit.

The long-term direction: a system that understands your financial patterns well enough to catch anomalies before you do, flag recurring charges you've forgotten about, and surface insights you'd never think to ask for.

---

## What it does today

- JWT-based auth with PBKDF2 password hashing
- Multi-workspace with owner / admin / member roles
- Manual transaction entry, filterable by category, type, and date
- Bank statement upload (JPEG, PNG) — DeepSeek AI parses and imports transactions automatically
- AI-generated spending insights over any date range
- File storage via Cloudflare R2

---

## Stack

Rust compiled to `wasm32-unknown-unknown`, running on Cloudflare Workers. Clean architecture: domain traits with no external dependencies, D1 (SQLite-compatible) for persistence, R2 for file storage, DeepSeek for AI. No servers, no cold starts.

| Concern | Technology |
|---|---|
| Runtime | Cloudflare Workers (WASM) |
| Database | Cloudflare D1 |
| File storage | Cloudflare R2 |
| AI | DeepSeek API (`deepseek-chat`) |
| Auth | JWT + PBKDF2 (pure Rust) |
| Deployment | Wrangler CLI |

---

## Getting started

Prerequisites: [Rust](https://rustup.rs/) stable, [Wrangler](https://developers.cloudflare.com/workers/wrangler/install-and-update/) v3+.

```bash
# create D1 database and copy the database_id into wrangler.toml
wrangler d1 create ai-finance-tracker

# create R2 bucket
wrangler r2 bucket create ai-finance-tracker-storage

# run migrations
wrangler d1 execute ai-finance-tracker --file=migrations/0001_create_users.sql
wrangler d1 execute ai-finance-tracker --file=migrations/0002_create_workspaces.sql
wrangler d1 execute ai-finance-tracker --file=migrations/0003_create_transactions.sql
wrangler d1 execute ai-finance-tracker --file=migrations/0004_create_bank_statements.sql
wrangler d1 execute ai-finance-tracker --file=migrations/0005_create_ai_insights.sql

# set secrets
wrangler secret put JWT_SECRET
wrangler secret put DEEPSEEK_API_KEY

# deploy
wrangler deploy
```

For local dev, create `.dev.vars` (do not commit):

```ini
JWT_SECRET=your-local-secret
DEEPSEEK_API_KEY=your-deepseek-key
```

Then run `wrangler dev`.

---

## Environments

| Environment | Command |
|---|---|
| Production | `wrangler deploy` |
| Staging | `wrangler deploy --env staging` |
| Local | `wrangler dev` |

Staging uses a separate D1 database and R2 bucket defined in `wrangler.toml`. Set staging secrets with `wrangler secret put <KEY> --env staging`.

---

## License

MIT
