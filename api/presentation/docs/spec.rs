pub const OPENAPI_SPEC: &str = r##"{
  "openapi": "3.0.3",
  "info": {
    "title": "AI Finance Tracker API",
    "description": "Upload a bank statement, let the AI file the transactions and tell you what to cut.",
    "version": "0.1.0",
    "contact": {
      "name": "maulanasdqn",
      "url": "https://github.com/maulanasdqn/ai-personal-finance-tracker"
    }
  },
  "servers": [
    { "url": "/", "description": "Current server" }
  ],
  "components": {
    "securitySchemes": {
      "bearerAuth": {
        "type": "http",
        "scheme": "bearer",
        "bearerFormat": "JWT"
      }
    },
    "schemas": {
      "AuthResponse": {
        "type": "object",
        "properties": {
          "token": { "type": "string" },
          "user_id": { "type": "string", "format": "uuid" },
          "email": { "type": "string", "format": "email" },
          "full_name": { "type": "string" }
        },
        "required": ["token", "user_id", "email", "full_name"]
      },
      "RegisterRequest": {
        "type": "object",
        "properties": {
          "email": { "type": "string", "format": "email" },
          "password": { "type": "string", "minLength": 8 },
          "full_name": { "type": "string" }
        },
        "required": ["email", "password", "full_name"]
      },
      "LoginRequest": {
        "type": "object",
        "properties": {
          "email": { "type": "string", "format": "email" },
          "password": { "type": "string" }
        },
        "required": ["email", "password"]
      },
      "Workspace": {
        "type": "object",
        "properties": {
          "id": { "type": "string", "format": "uuid" },
          "name": { "type": "string" },
          "description": { "type": "string", "nullable": true },
          "owner_id": { "type": "string", "format": "uuid" },
          "created_at": { "type": "string", "format": "date-time" },
          "updated_at": { "type": "string", "format": "date-time" }
        }
      },
      "CreateWorkspaceRequest": {
        "type": "object",
        "properties": {
          "name": { "type": "string" },
          "description": { "type": "string", "nullable": true }
        },
        "required": ["name"]
      },
      "UpdateWorkspaceRequest": {
        "type": "object",
        "properties": {
          "name": { "type": "string" },
          "description": { "type": "string", "nullable": true }
        }
      },
      "WorkspaceMember": {
        "type": "object",
        "properties": {
          "workspace_id": { "type": "string", "format": "uuid" },
          "user_id": { "type": "string", "format": "uuid" },
          "role": { "type": "string", "enum": ["owner", "admin", "member"] },
          "joined_at": { "type": "string", "format": "date-time" }
        }
      },
      "InviteMemberRequest": {
        "type": "object",
        "properties": {
          "email": { "type": "string", "format": "email" }
        },
        "required": ["email"]
      },
      "Transaction": {
        "type": "object",
        "properties": {
          "id": { "type": "string", "format": "uuid" },
          "workspace_id": { "type": "string", "format": "uuid" },
          "amount": { "type": "number", "format": "float" },
          "currency": { "type": "string", "example": "IDR" },
          "category": { "type": "string", "example": "Food" },
          "description": { "type": "string", "nullable": true },
          "transaction_date": { "type": "string", "format": "date" },
          "transaction_type": { "type": "string", "enum": ["income", "expense"] },
          "source": { "type": "string", "enum": ["manual", "imported"] },
          "created_by": { "type": "string", "format": "uuid" },
          "created_at": { "type": "string", "format": "date-time" },
          "updated_at": { "type": "string", "format": "date-time" }
        }
      },
      "CreateTransactionRequest": {
        "type": "object",
        "properties": {
          "amount": { "type": "number", "format": "float", "minimum": 0.01 },
          "currency": { "type": "string", "example": "IDR" },
          "category": { "type": "string", "example": "Food" },
          "description": { "type": "string", "nullable": true },
          "transaction_date": { "type": "string", "format": "date", "example": "2026-06-03" },
          "transaction_type": { "type": "string", "enum": ["income", "expense"] }
        },
        "required": ["amount", "category", "transaction_date", "transaction_type"]
      },
      "BankStatement": {
        "type": "object",
        "properties": {
          "id": { "type": "string", "format": "uuid" },
          "workspace_id": { "type": "string", "format": "uuid" },
          "file_name": { "type": "string" },
          "file_type": { "type": "string", "enum": ["pdf", "image"] },
          "status": { "type": "string", "enum": ["pending", "processing", "processed", "failed"] },
          "parsed_transactions": { "type": "array", "nullable": true, "items": { "type": "object" } },
          "ai_summary": { "type": "string", "nullable": true },
          "created_by": { "type": "string", "format": "uuid" },
          "created_at": { "type": "string", "format": "date-time" },
          "updated_at": { "type": "string", "format": "date-time" }
        }
      },
      "AiInsight": {
        "type": "object",
        "properties": {
          "id": { "type": "string", "format": "uuid" },
          "workspace_id": { "type": "string", "format": "uuid" },
          "insight_type": { "type": "string", "enum": ["tip", "reduction", "recommendation", "analysis"] },
          "title": { "type": "string" },
          "content": { "type": "string" },
          "metadata": { "type": "object", "nullable": true },
          "created_at": { "type": "string", "format": "date-time" }
        }
      },
      "GenerateInsightsRequest": {
        "type": "object",
        "properties": {
          "date_from": { "type": "string", "format": "date", "example": "2026-01-01" },
          "date_to": { "type": "string", "format": "date", "example": "2026-06-03" }
        },
        "required": ["date_from", "date_to"]
      },
      "ErrorResponse": {
        "type": "object",
        "properties": {
          "message": { "type": "string" }
        }
      }
    }
  },
  "security": [{ "bearerAuth": [] }],
  "paths": {
    "/api/v1/auth/register": {
      "post": {
        "tags": ["Auth"],
        "summary": "Register a new user",
        "security": [],
        "requestBody": {
          "required": true,
          "content": { "application/json": { "schema": { "$ref": "#/components/schemas/RegisterRequest" } } }
        },
        "responses": {
          "200": { "description": "Registered", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/AuthResponse" } } } },
          "400": { "description": "Validation error", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/ErrorResponse" } } } },
          "409": { "description": "Email already registered", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/ErrorResponse" } } } }
        }
      }
    },
    "/api/v1/auth/login": {
      "post": {
        "tags": ["Auth"],
        "summary": "Login and receive a JWT",
        "security": [],
        "requestBody": {
          "required": true,
          "content": { "application/json": { "schema": { "$ref": "#/components/schemas/LoginRequest" } } }
        },
        "responses": {
          "200": { "description": "Authenticated", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/AuthResponse" } } } },
          "401": { "description": "Invalid credentials", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/ErrorResponse" } } } }
        }
      }
    },
    "/api/v1/workspaces": {
      "get": {
        "tags": ["Workspaces"],
        "summary": "List all workspaces the authenticated user belongs to",
        "responses": {
          "200": { "description": "List of workspaces", "content": { "application/json": { "schema": { "type": "array", "items": { "$ref": "#/components/schemas/Workspace" } } } } }
        }
      },
      "post": {
        "tags": ["Workspaces"],
        "summary": "Create a new workspace",
        "requestBody": {
          "required": true,
          "content": { "application/json": { "schema": { "$ref": "#/components/schemas/CreateWorkspaceRequest" } } }
        },
        "responses": {
          "200": { "description": "Created workspace", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/Workspace" } } } }
        }
      }
    },
    "/api/v1/workspaces/{id}": {
      "parameters": [{ "name": "id", "in": "path", "required": true, "schema": { "type": "string" } }],
      "get": {
        "tags": ["Workspaces"],
        "summary": "Get workspace by ID",
        "responses": {
          "200": { "description": "Workspace", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/Workspace" } } } },
          "403": { "description": "Not a member" },
          "404": { "description": "Not found" }
        }
      },
      "put": {
        "tags": ["Workspaces"],
        "summary": "Update workspace (owner/admin only)",
        "requestBody": {
          "required": true,
          "content": { "application/json": { "schema": { "$ref": "#/components/schemas/UpdateWorkspaceRequest" } } }
        },
        "responses": {
          "200": { "description": "Updated workspace", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/Workspace" } } } }
        }
      },
      "delete": {
        "tags": ["Workspaces"],
        "summary": "Delete workspace (owner only)",
        "responses": {
          "200": { "description": "Deleted" },
          "403": { "description": "Forbidden" }
        }
      }
    },
    "/api/v1/workspaces/{id}/members": {
      "parameters": [{ "name": "id", "in": "path", "required": true, "schema": { "type": "string" } }],
      "get": {
        "tags": ["Workspaces"],
        "summary": "List workspace members",
        "responses": {
          "200": { "description": "Members", "content": { "application/json": { "schema": { "type": "array", "items": { "$ref": "#/components/schemas/WorkspaceMember" } } } } }
        }
      },
      "post": {
        "tags": ["Workspaces"],
        "summary": "Invite a user by email (owner/admin only)",
        "requestBody": {
          "required": true,
          "content": { "application/json": { "schema": { "$ref": "#/components/schemas/InviteMemberRequest" } } }
        },
        "responses": {
          "200": { "description": "Member added", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/WorkspaceMember" } } } },
          "404": { "description": "User not found" }
        }
      }
    },
    "/api/v1/workspaces/{workspace_id}/transactions": {
      "parameters": [
        { "name": "workspace_id", "in": "path", "required": true, "schema": { "type": "string" } },
        { "name": "category", "in": "query", "schema": { "type": "string" } },
        { "name": "type", "in": "query", "schema": { "type": "string", "enum": ["income", "expense"] } },
        { "name": "from", "in": "query", "schema": { "type": "string", "format": "date" } },
        { "name": "to", "in": "query", "schema": { "type": "string", "format": "date" } },
        { "name": "limit", "in": "query", "schema": { "type": "integer", "default": 50 } },
        { "name": "offset", "in": "query", "schema": { "type": "integer", "default": 0 } }
      ],
      "get": {
        "tags": ["Transactions"],
        "summary": "List transactions in a workspace",
        "responses": {
          "200": { "description": "Transactions", "content": { "application/json": { "schema": { "type": "array", "items": { "$ref": "#/components/schemas/Transaction" } } } } }
        }
      },
      "post": {
        "tags": ["Transactions"],
        "summary": "Create a transaction",
        "requestBody": {
          "required": true,
          "content": { "application/json": { "schema": { "$ref": "#/components/schemas/CreateTransactionRequest" } } }
        },
        "responses": {
          "200": { "description": "Created transaction", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/Transaction" } } } }
        }
      }
    },
    "/api/v1/workspaces/{workspace_id}/transactions/{id}": {
      "parameters": [
        { "name": "workspace_id", "in": "path", "required": true, "schema": { "type": "string" } },
        { "name": "id", "in": "path", "required": true, "schema": { "type": "string" } }
      ],
      "get": {
        "tags": ["Transactions"],
        "summary": "Get a transaction by ID",
        "responses": {
          "200": { "description": "Transaction", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/Transaction" } } } },
          "404": { "description": "Not found" }
        }
      },
      "delete": {
        "tags": ["Transactions"],
        "summary": "Delete a transaction",
        "responses": {
          "200": { "description": "Deleted" }
        }
      }
    },
    "/api/v1/workspaces/{workspace_id}/statements": {
      "parameters": [{ "name": "workspace_id", "in": "path", "required": true, "schema": { "type": "string" } }],
      "get": {
        "tags": ["Bank Statements"],
        "summary": "List bank statements for a workspace",
        "responses": {
          "200": { "description": "Statements", "content": { "application/json": { "schema": { "type": "array", "items": { "$ref": "#/components/schemas/BankStatement" } } } } }
        }
      },
      "post": {
        "tags": ["Bank Statements"],
        "summary": "Upload a bank statement (image or PDF). AI will auto-parse transactions.",
        "description": "Send raw file bytes as the request body. Set `Content-Type` to `image/jpeg`, `image/png`, or `application/pdf`. Set `X-File-Name` to the original filename.",
        "requestBody": {
          "required": true,
          "content": {
            "image/jpeg": { "schema": { "type": "string", "format": "binary" } },
            "image/png": { "schema": { "type": "string", "format": "binary" } },
            "application/pdf": { "schema": { "type": "string", "format": "binary" } }
          }
        },
        "parameters": [
          { "name": "X-File-Name", "in": "header", "required": true, "schema": { "type": "string" } }
        ],
        "responses": {
          "200": { "description": "Statement uploaded and parsed", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/BankStatement" } } } }
        }
      }
    },
    "/api/v1/workspaces/{workspace_id}/statements/{id}": {
      "parameters": [
        { "name": "workspace_id", "in": "path", "required": true, "schema": { "type": "string" } },
        { "name": "id", "in": "path", "required": true, "schema": { "type": "string" } }
      ],
      "get": {
        "tags": ["Bank Statements"],
        "summary": "Get a bank statement by ID",
        "responses": {
          "200": { "description": "Statement", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/BankStatement" } } } },
          "404": { "description": "Not found" }
        }
      }
    },
    "/api/v1/workspaces/{workspace_id}/insights": {
      "parameters": [
        { "name": "workspace_id", "in": "path", "required": true, "schema": { "type": "string" } },
        { "name": "type", "in": "query", "schema": { "type": "string", "enum": ["tip", "reduction", "recommendation", "analysis"] } }
      ],
      "get": {
        "tags": ["AI Insights"],
        "summary": "List AI insights for a workspace",
        "responses": {
          "200": { "description": "Insights", "content": { "application/json": { "schema": { "type": "array", "items": { "$ref": "#/components/schemas/AiInsight" } } } } }
        }
      }
    },
    "/api/v1/workspaces/{workspace_id}/insights/generate": {
      "parameters": [{ "name": "workspace_id", "in": "path", "required": true, "schema": { "type": "string" } }],
      "post": {
        "tags": ["AI Insights"],
        "summary": "Generate AI financial insights from transaction data",
        "description": "Analyzes transactions in the given date range using DeepSeek AI. Returns tips to improve savings, spending categories to reduce, and purchase recommendations.",
        "requestBody": {
          "required": true,
          "content": { "application/json": { "schema": { "$ref": "#/components/schemas/GenerateInsightsRequest" } } }
        },
        "responses": {
          "200": { "description": "Generated insights", "content": { "application/json": { "schema": { "type": "array", "items": { "$ref": "#/components/schemas/AiInsight" } } } } },
          "400": { "description": "No transactions found in the specified period" }
        }
      }
    }
  }
}"##;
