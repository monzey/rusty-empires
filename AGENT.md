# Agent Guide - Rusty Empires

## Project Summary

Rusty Empires is a Rust prototype for a turn-based tactical strategy game inspired by historical strategy games and by the tactical feel of Age of Empires on Nintendo DS. The project is original and must not reuse proprietary assets, names, brands, or content.

Current goal: build a small, testable tactical engine first, then grow toward a complete game through incremental, tested features.

## Stack

- Language: Rust 2021
- Game engine: Bevy 0.14
- Main paradigm: ECS through Bevy
- Dev environment: devenv / Nix
- Initial target: Linux desktop
- Package: `rusty-empires`
- Publishing: disabled with `publish = false`

## Commands

Prefer devenv scripts when inside the development shell.

```bash
devenv shell
devenv shell check
devenv shell run
devenv shell fmt
devenv shell lint
```

Equivalent Cargo commands:

```bash
cargo check
cargo run
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

## Repository Layout

- `src/lib.rs`: public library surface and re-exports.
- `src/core/`: deterministic game rules, stable IDs, actions, events, errors, turn loop, movement, construction, economy, and AI step logic.
- `src/core/rules/`: focused gameplay rule modules.
- `src/app/`: Bevy shell, rendering, input handling, entity spawning, grid conversion, and event-driven synchronization with `Game`.
- `src/main.rs`: minimal Bevy bootstrap that installs `RustyEmpiresAppPlugin`.
- `tests/*.rs`: integration tests split by gameplay domain.
- `README.md`: project overview, roadmap, launch commands, and high-level stack.
- `SPEC.md`: functional design document. Treat it as product direction, not as implemented behavior.
- `devenv.nix`: Rust toolchain, native libraries, and helper scripts.

## Architecture

The project uses a core/app split:

- Core rules are pure Rust.
- Bevy is the shell around the core.
- Bevy entities never enter the core.
- Core entities use stable IDs.
- External callers mutate game state through `Game::apply(Action)`.
- `Game::apply` returns `Vec<Event>` for the app to sync visuals.
- Tests target the core, not Bevy rendering.

Main core concepts:

- `Game`
- `Action`
- `Event`
- `GameError`
- `MoveError`
- `TurnError`
- `GridPosition`
- `UnitId`
- `Camp`

## Long-Term Architecture Direction

Keep the core/app split. The current project should scale through modules first. If it grows enough, split the mono-crate layout into crates:

- `rusty_empires_core`: deterministic rules and simulation.
- `rusty_empires_app`: Bevy rendering, input, UI, assets, audio.
- `rusty_empires_data`: typed gameplay definitions.
- `rusty_empires_ai`: advanced AI planners.
- `rusty_empires_save`: saves and replays.

Current module groups:

- `core`: rules and simulation.
- `core/rules`: focused deterministic gameplay rules.
- `app`: Bevy shell.
- `data`: future typed definitions or loaded config.
- `ai`: future advanced AI controller logic when it outgrows simple core helpers.

Target flow:

```text
Bevy input -> Action -> Game::apply -> Event list -> Bevy visual sync
```

Keep this invariant: the game must be able to run many turns in tests without opening a Bevy window.

## Feature Implementation Pattern

When adding gameplay features:

- Read existing code and `SPEC.md` first.
- Put deterministic rules in core code.
- Add an `Action` variant for player or AI intent.
- Validate the action in `Game::apply` or a focused core helper.
- Return `Event` values for state changes the app must display.
- Add stable IDs for new core entities when Bevy must reference them.
- Keep Bevy code limited to input, rendering, UI, animation, and visual sync.
- Add core tests for rules before or alongside app changes.
- Update docs only for architecture or workflow changes, not as a feature inventory.

## Engineering Guidelines For LLMs

- Keep core game rules in `src/core/` unless they must touch Bevy APIs.
- Keep Bevy-specific rendering and input in `src/app/`.
- Add or update tests in the relevant domain file under `tests/` when changing rules in `Game`.
- Prefer adding new rules through `Action` and `Event` instead of directly mutating public state.
- Use stable core IDs such as `UnitId`; do not store Bevy `Entity` in core state.
- Prefer small, explicit changes over broad architecture rewrites.
- Preserve deterministic rule behavior in tests.
- Avoid adding assets or external content unless user asks.
- Do not introduce networking, save files, async tasks, or asset pipelines without explicit need.
- Keep French user-facing strings consistent with current code and docs.
- Keep code formatted with `cargo fmt`.
- Run `cargo test` for logic changes and `cargo check` for Bevy integration changes when feasible.

## Bevy Notes

- App uses `DefaultPlugins` with a configured primary window.
- Window title: `Rusty Empires`.
- Present mode: `PresentMode::AutoNoVsync`.
- Camera: `Camera2dBundle`.
- Tiles and units use `SpriteBundle`.
- Map-to-world conversion is handled by `grid_to_world`.
- Cursor-to-grid conversion is handled by `cursor_grid_position` and `world_to_grid`.
- Core IDs are stored on Bevy components when app entities must sync from core events.

## Native Dependencies

`devenv.nix` provides Rust stable, `rust-analyzer`, `clang`, `pkg-config`, Vulkan tooling, and runtime libraries used by Bevy on Linux:

- ALSA
- libxkbcommon
- udev
- vulkan-loader
- Wayland
- X11 libraries
