# Frontend Integration Guide

This document explains how the Lightweight fitness tracker integrates SvelteKit with the Cali gRPC backend.

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    Proto Files (Source of Truth)            │
│              interface/grpc/services/*.proto                │
└──────────────┬──────────────────────────┬───────────────────┘
               │                          │
               ├─ cali generate           ├─ npm run generate
               │  controllers             │  (frontend)
               ↓                          ↓
       ┌──────────────┐          ┌──────────────────┐
       │ Rust Backend │←─gRPC-Web─│ SvelteKit        │
       │ (tonic)      │           │ (Connect-Web)    │
       └──────────────┘           └──────────────────┘
               ↑                          ↑
               │                          │
          Port 50051               Port 5173 (dev)
```

## What Was Built

### 1. Frontend Setup (`frontend/`)

- **SvelteKit**: Modern web framework with TypeScript
- **Connect-Web**: gRPC-Web client library for browsers
- **Auto-generation**: Script to generate TypeScript clients from protos
- **Prettier + ESLint**: Code quality tools configured

### 2. Code Generation Pipeline

```bash
# Backend (Rust controllers)
cali generate controllers
→ Reads: interface/grpc/services/*.proto
→ Generates: web/src/controllers/*.rs

# Frontend (TypeScript clients)
cd frontend && npm run generate
→ Reads: ../interface/grpc/services/*.proto
→ Generates: src/lib/generated/*_connect.ts, *_pb.ts
```

### 3. Example Implementation

See `frontend/src/routes/+page.svelte` for:
- Creating typed gRPC clients
- Calling backend methods with full type safety
- Handling responses and errors

## Running the Full Stack

### Terminal 1: Backend

```bash
cd web
cargo run
```

Runs tonic gRPC server on `localhost:50051`

### Terminal 2: Frontend

```bash
cd frontend
npm install  # First time only
npm run dev
```

Runs SvelteKit dev server on `http://localhost:5173`

## Important Notes

### ⚠️ gRPC-Web Layer Needed

The frontend uses gRPC-Web, but the current backend doesn't have tonic-web enabled yet.

**Options:**

1. **Add tonic-web to backend** (see `web/README_TONIC_WEB.md`)
2. **Use Envoy proxy** (recommended for production)
3. **Wait for Cali framework update** with built-in web support

### Current State

- ✅ Frontend scaffold created
- ✅ TypeScript client generation working
- ✅ Example Svelte components with typed API calls
- ✅ Tauri desktop app setup documented
- ⚠️ Backend needs tonic-web layer (manual setup required)
- ⚠️ Or use Envoy proxy in front of backend

## Development Workflow

1. **Modify proto files** in `interface/grpc/services/`
2. **Regenerate backend**: `cali generate controllers`
3. **Regenerate frontend**: `cd frontend && npm run generate`
4. **Implement logic**:
   - Backend: `web/src/controllers/*.rs`
   - Frontend: `frontend/src/routes/*.svelte`
5. **Test end-to-end**

## Type Safety Example

### Proto Definition

```protobuf
service Accounts {
  rpc CreateAccount (CreateAccountRequest) returns (CreateAccountResponse);
}

message CreateAccountRequest {
  string name = 1;
  string email = 2;
  string password = 3;
}
```

### Rust Controller (Auto-generated scaffold)

```rust
#[async_trait]
impl Accounts for AccountsController {
    async fn create_account(
        &self,
        request: Request<CreateAccountRequest>,
    ) -> Result<Response<CreateAccountResponse>, Status> {
        // Your implementation
    }
}
```

### TypeScript Client (Auto-generated)

```typescript
const client = createPromiseClient(Accounts, transport);

// Fully typed! Editor shows all fields
const response = await client.createAccount({
  name: "John",
  email: "john@example.com",
  password: "secret"
});
```

## Next Steps

### Immediate

1. Enable tonic-web on backend (see `web/README_TONIC_WEB.md`)
2. Test the full stack end-to-end
3. Add authentication/authorization

### Future

1. Add Tauri for desktop app (`frontend/TAURI_SETUP.md`)
2. Mobile apps with Capacitor
3. Deploy with Docker + Envoy proxy
4. Add WebSocket streaming for real-time updates

## Benefits

- **Single source of truth**: Proto files define everything
- **Type safety**: Can't call APIs with wrong types
- **Fast iteration**: Change proto → regenerate → immediate feedback
- **No drift**: Frontend and backend stay in sync automatically
- **Modern stack**: Best tools for Rust and TypeScript

## Troubleshooting

### "Connection refused" in browser

→ Backend not running or tonic-web not configured

### "Command not found: protoc"

→ Install protobuf compiler: `brew install protobuf` (macOS)

### TypeScript errors after proto changes

→ Regenerate: `npm run generate`

### Rust compilation errors after proto changes

→ Regenerate: `cali generate controllers`

## Learn More

- **Cali Framework**: See `../cali/README.md`
- **Frontend Support**: See `../cali/FRONTEND_SUPPORT.md`
- **Connect-Web**: https://connectrpc.com/docs/web/
- **SvelteKit**: https://svelte.dev/docs/kit
