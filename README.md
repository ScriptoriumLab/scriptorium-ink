# Scriptorium Ink

> The stateless UI renderer for the Scriptorium input method ecosystem.

**Scriptorium Ink** is the presentation layer of Scriptorium.

It provides the candidate window and other user-facing UI for the input method, while deliberately keeping input-method state and business logic outside the UI process.

Ink is currently built with **Tauri, Svelte, and TypeScript**.

---

## Role in Scriptorium

Scriptorium is designed as a multi-process input method architecture.

Ink is responsible only for presentation:

```text
                 Scriptorium-Inkstone
                    Stateful Core
                         │
                         │ RenderState
                         ▼
                  Scriptorium-Ink
                 Stateless Renderer
                         │
                         │ UserAction
                         ▼
                 Scriptorium-Inkstone
```

The input method core remains the single source of truth.

Ink receives state from the core, renders it, and sends user interactions back as actions.

It does **not** own input-method business state.

This keeps the UI replaceable and allows the core and presentation layers to evolve independently.

---

## Design Principles

### Stateless UI

Ink is a renderer, not the owner of input-method state.

If the UI process is restarted, the authoritative state still belongs to Scriptorium-Inkstone.

### Unidirectional Data Flow

Communication follows a simple model:

```text
Core → UI : RenderState
UI → Core : UserAction
```

This makes state ownership explicit and avoids maintaining competing copies of the same business state.

### Process Isolation

Ink runs independently from both the platform integration layer and the input-method core.

A UI failure should not compromise the core input method or the host application.

### Replaceable Presentation

The UI is treated as an implementation detail behind a stable architectural boundary.

Tauri is the current implementation, but the architecture is intentionally designed so that alternative rendering technologies can be explored in the future without changing the input-method core.

---

## UI

The current candidate window follows a lightweight, macOS-inspired visual style.

Ink is responsible for presenting information such as:

- composition text
- candidate lists
- candidate selection
- paging and navigation state
- other input-method presentation state

Business decisions—such as candidate generation, segmentation, ranking, and composition state—belong to Scriptorium-Inkstone rather than Ink.

---

## Technology Stack

The current implementation uses:

- **Tauri 2** — native application shell
- **Svelte 5** — UI framework
- **TypeScript** — frontend implementation
- **Vite** — frontend tooling
- **Rust** — Tauri native runtime

The choice of UI technology is intentionally isolated from the rest of the Scriptorium architecture.

---

## Development

### Prerequisites

To build and run Ink locally, install:

- Rust
- Node.js
- pnpm
- the platform prerequisites required by Tauri

Then install the project dependencies:

```bash
pnpm install
```

### Run in Development Mode

```bash
pnpm tauri dev
```

### Check the Frontend

```bash
pnpm check
```

### Build

```bash
pnpm tauri build
```

---

## Why "Ink"?

The Scriptorium repositories use traditional writing tools as an architectural metaphor.

**Ink** is what ultimately becomes visible on the page.

Likewise, Scriptorium-Ink is the visible expression of the input method's internal state: it does not decide what should be written, but presents the state produced by the core to the user.

---

## Project Status

Scriptorium Ink is under active development.

The current implementation primarily targets the Scriptorium development environment on Windows, while its architectural boundary is designed with future cross-platform evolution in mind.

APIs, protocols, UI behavior, and project structure may continue to evolve as the wider Scriptorium architecture develops.

---

## License

Scriptorium Ink is licensed under the **Apache License 2.0**.

See `LICENSE` for details.

---

Copyright © 2026 ScriptoriumLab.
