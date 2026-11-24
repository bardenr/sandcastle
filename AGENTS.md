# Project guidance

## Coding style and architecture

- Prefer plain data structures and procedural or functional code.
- Use traits where an interface is useful or a dependency requires one; avoid
  inheritance-style designs.
- Keep application logic independent of presentation and OS integration.
- The application starts as a native, local application in a single crate.
  Some pieces may eventually run as cloud services. Preserve clear boundaries,
  but introduce service interfaces, networking, or additional crates only when
  a concrete feature needs them.

## Structure

- `src/main.rs`: startup and wiring.
- `src/app.rs`: application state and actions, independent of UI and OS types
  (currently a placeholder).
- `src/ui/mod.rs`: native window lifecycle and input handling through winit.
- `src/ui/renderer.rs`: GPU resources and rendering functions through wgpu.
- `src/ui/shader.wgsl`: rendering shader.
- `src/platform/mod.rs`: OS integration as concrete needs arise
  (currently a placeholder).

## Validation

Run checks from the repository root:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
```

Run the native application with `cargo run` when manually checking UI behavior.
Report checks that could not run and whether the UI was manually checked.

Event diagnostics are opt-in through `SANDCASTLE_LOG_INPUT=1`.
