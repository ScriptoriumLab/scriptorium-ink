# Scriptorium Ink

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

> The stateless presentation layer for Scriptorium.

**Scriptorium Ink** is the presentation layer of the [Scriptorium](https://github.com/ScriptoriumLab) input method ecosystem.

It renders candidate windows and other user-facing input-method state while deliberately keeping authoritative input-method state and business logic outside the UI process.

Ink is currently implemented with **Tauri, Svelte, TypeScript, and Rust**.

---

## Role in Scriptorium

Scriptorium separates platform integration, input-method logic, presentation, and shared infrastructure into independently evolving components.

![Scriptorium Architecture](assets/Scriptorium%20Architecture%20V1.1.1.png)

Within that architecture:

- **[Scriptorium Brush](https://github.com/ScriptoriumLab/scriptorium-brush)** integrates Scriptorium with the operating system through a platform-specific input-method framework.
- **[Scriptorium Inkstone](https://github.com/ScriptoriumLab/scriptorium-inkstone)** owns authoritative input-method state and core behavior such as composition, segmentation, dictionary lookup, candidate generation, and ranking.
- **Scriptorium Ink** renders user-facing state without owning input-method business state.
- **[Scriptorium Felt](https://github.com/ScriptoriumLab/scriptorium-felt)** provides shared protocols, IPC abstractions, and reusable infrastructure.

Ink is therefore a **presentation boundary**, not another input-method state owner.

Its role can be summarized as:

```text
Inkstone
   │
   │ RenderState
   ▼
  Ink
   │
   │ UserAction
   ▼
Inkstone
```

The input-method core remains the single source of truth.

Ink receives state, renders it, and reports user interactions back as actions.

---

## State Ownership

A central design rule in Scriptorium is:

> The input-method core owns input-method state; Ink renders it.

Ink does not own:

- composition state
- segmentation state
- dictionary state
- candidate generation
- candidate ranking
- authoritative candidate selection
- input-method policy

These responsibilities belong to Inkstone.

Ink may maintain transient UI state required for rendering or interaction, but that state must not become a competing source of truth for input-method behavior.

This keeps presentation and input-method logic independently evolvable.

---

## Unidirectional Data Flow

Communication between Inkstone and Ink follows a simple model:

```text
Inkstone → Ink : RenderState
Ink → Inkstone : UserAction
```

A typical interaction looks like:

```text
Input Method State
       │
       ▼
   RenderState
       │
       ▼
      Ink
       │
       │ user interaction
       ▼
   UserAction
       │
       ▼
   Inkstone
       │
       ▼
New Authoritative State
```

Ink does not directly mutate the authoritative input-method model.

Instead, user interaction is represented as an action.

Inkstone evaluates that action, updates the authoritative state, and produces the next render state.

This avoids maintaining competing state machines across processes.

---

## Responsibilities

Ink is responsible for presenting the visible input-method experience.

Current responsibilities include:

- composition text
- candidate lists
- candidate highlighting
- paging and navigation state
- selection feedback
- user interaction with candidate UI
- other user-facing input-method presentation

Ink decides **how state is presented**.

Inkstone decides **what the state means**.

A useful distinction is:

> Inkstone owns input-method policy.  
> Ink owns presentation.

---

## Stateless Presentation

Ink is intentionally designed as a stateless renderer from the perspective of input-method business logic.

If Ink restarts, the authoritative input-method state remains in Inkstone.

Conceptually:

```text
Ink crashes / restarts
        │
        ▼
Inkstone state remains valid
        │
        ▼
New RenderState
        │
        ▼
Ink renders again
```

This reduces synchronization complexity and makes the presentation process replaceable.

The goal is not that Ink literally contains no local state.

UI frameworks naturally maintain transient rendering and interaction state.

The architectural constraint is that Ink must not become the authoritative owner of input-method business state.

---

## Process Isolation

Ink runs independently from both the platform adapter and the input-method core.

This isolates UI technology and UI failures from the platform integration layer.

For example:

- the Windows TSF adapter does not need to embed the candidate-window implementation
- Inkstone does not need to depend on a UI framework
- changing UI technology does not require changing the input-method domain model
- restarting Ink does not require restarting the host application

Presentation is therefore treated as an independently evolving process boundary.

---

## Replaceable Presentation

Tauri is the current implementation technology.

It is not intended to become an architectural constraint on the rest of Scriptorium.

The stable boundary is:

```text
RenderState
    │
    ▼
Presentation
    │
    ▼
UserAction
```

As long as another UI implementation can consume the same presentation state and produce equivalent user actions, the input-method core does not need to know how the UI is rendered.

This makes presentation technology replaceable independently from input-method behavior.

---

## UI

The current implementation provides the candidate-window experience for Scriptorium.

The visual direction is intentionally lightweight and unobtrusive.

Ink renders information such as:

- current composition
- candidate text
- selected candidate
- candidate pages
- navigation state
- interaction feedback

Business decisions remain outside the UI.

For example:

```text
Candidate ranking
        │
        ▼
     Inkstone
        │
        ▼
   RenderState
        │
        ▼
      Ink
```

Ink renders the ranked candidates it receives.

It does not independently re-rank them.

---

## Architecture

Ink separates native application hosting from frontend presentation.

The current implementation uses Tauri as the native shell and Svelte as the frontend UI layer.

Conceptually:

```text
┌─────────────────────────────────────┐
│             Ink Process             │
│                                     │
│   Tauri / Rust Native Runtime       │
│               │                     │
│               ▼                     │
│      Svelte / TypeScript UI         │
│                                     │
└─────────────────────────────────────┘
                ▲
                │ RenderState
                │
             Inkstone
                │
                │ UserAction
                ▼
```

The implementation boundary is intentionally internal to Ink.

Inkstone should not need to know whether the UI is implemented with:

- Tauri
- Svelte
- another web-based renderer
- a future native UI toolkit
- another presentation technology entirely

The presentation contract matters more than the rendering technology.

---

## Technology Stack

The current implementation uses:

- **Tauri 2** — native application shell
- **Rust** — native runtime
- **Svelte 5** — UI framework
- **TypeScript** — frontend implementation
- **Vite** — frontend tooling

These technologies are implementation choices local to Ink.

They should not leak into Scriptorium's core input-method model.

---

## Design Principles

### Stateless Business State

Ink does not own authoritative input-method state.

It renders the state produced by Inkstone.

### Unidirectional Interaction

State flows from Inkstone to Ink.

User intent flows from Ink back to Inkstone.

```text
State → UI → Action → Core
```

### Presentation, Not Policy

Ink decides how information is displayed and interacted with.

It does not decide input-method behavior.

### Process Isolation

UI rendering runs independently from the platform adapter and core engine.

This reduces coupling between presentation technology and system integration.

### Replaceable Technology

Tauri, Svelte, Rust, and TypeScript are current implementation choices.

They should remain replaceable behind the presentation boundary.

### Evolution Without Lock-In

Today's UI implementation should not unnecessarily constrain tomorrow's presentation technology or platform support.

Ink therefore depends on stable presentation contracts rather than exposing UI-framework concepts to the wider Scriptorium architecture.

---

## Development

### Prerequisites

To build and run Ink locally, install:

- Rust
- Node.js
- pnpm
- the platform prerequisites required by Tauri

### Install Dependencies

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

For complete Scriptorium development workflows, Ink is increasingly intended to be built and operated through [Scriptorium CLI](https://github.com/ScriptoriumLab/scriptorium-cli).

---

## Project Status

Scriptorium Ink is under active development.

The current implementation provides the candidate-window presentation layer for the Scriptorium runtime.

Ongoing work continues to refine areas such as:

- candidate-window behavior
- interaction handling
- presentation protocol evolution
- UI responsiveness
- visual design
- process communication
- cross-platform behavior

The architectural goal is to keep Ink independent from both platform adapters and input-method business logic as Scriptorium expands to additional platforms.

---

## Why "Ink"?

The Scriptorium repositories use traditional writing tools as an architectural metaphor.

**Ink** is what ultimately becomes visible on the page.

Likewise, **Scriptorium Ink** is the visible expression of the input method's internal state.

It does not decide what should be written.

It presents the state produced by the input-method core to the user.

---

## License

Scriptorium Ink is licensed under the **Apache License 2.0**.

See `LICENSE` for details.

---

*Copyright © 2026 ScriptoriumLab.*