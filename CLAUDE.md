## Overview

You are in lightweight, a custom fitness tracker that makes use of a custom rust framework called cali. Cali provides scaffolding commands to help backend development, manages proto file to code generation, lays out a project structure and set's up a base sveltekit project in FE only mode so that we can have a frontend to the rust backend.

The following is a breakdown of the folder structure:
- web: the backend rust binary running a grpc tonic server making use of tonic's grpc web layer. It contains the controllers endpoint that services request.
- store: Contains all database logic for the application. This makes use of a sqlx query generator called Ensnare. This is also part of cali.
- interface: contains the proto files that FE and BE clients are generated from.
- frontend: contains the Sveltekit project with the FE

## Way of working

For features, you design the interface between BE and FE, write the protos and generate them using `cali generate controllers`. If the feature needs the DB, write the migrations in raw sql my generating new migration files in store using `sqlx migrate add <migration name>`, then implement the DB interface in store, using ensnare where possible. Then create the FE as specified by the user.

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
