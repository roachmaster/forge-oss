# forge-template

A small, focused library for rendering Mustache templates from a simple, serializable context. It’s designed to be a thin, deterministic building block that other Forge crates (including proc-macros) can rely on.

## What it does

- Takes a template (inline text or a file).
- Takes a context (hierarchical key→value map).
- Produces a rendered string.
- Optionally resolves partials from a directory or an in-memory map.
- Enforces a missing-key policy (error, empty, or keep the tag).
- Applies whitespace shaping (keep, trim, or smart-indent).
- Caches parsed templates for repeat renders.

## Why this exists

General Mustache engines are flexible but not always deterministic enough for codegen pipelines. This crate keeps the surface area tight and predictable so downstream tools (like compile-time generators) can depend on stable behavior.

## Core concepts

- **Context**: a map from `String` to a JSON-like `Value` (`null`, `bool`, `number`, `string`, `array`, `object`). Any `serde::Serialize` data can be converted into this format.
- **TemplateSource**: where the template comes from.
  - Inline text.
  - File path (useful with partials on disk).
- **Partials**: how `{{> name}}` is resolved.
  - None (disabled).
  - Directory (`dir/name.mustache`).
  - Map (in-memory name→template).
  - Map-then-Dir (map wins; dir is fallback).
- **MissingKeyPolicy**:
  - Error (stop rendering if a variable is absent).
  - Empty (default Mustache behavior).
  - KeepTag (leave `{{var}}` literally in output).
- **WhitespaceMode**:
  - Keep (verbatim).
  - TrimLines (trim trailing spaces; drop whitespace-only lines).
  - SmartIndent (TrimLines + collapse multi-blank runs to one).
- **Cache**: a small global cache keyed by the template text and option "meta" to avoid reparsing.

## Design goals

- Minimal surface; easy to audit.
- Deterministic outputs given the same inputs.
- Helpful, targeted error messages.
- Ready to embed in compile-time workflows.

## Public surface (high-level)

- **Engine**
  - Create with options; render with a source and a context.
- **Stateless render helper**
  - Same functionality without constructing an Engine.
- **Options**
  - `html_escape` (kept for compatibility; raw/escaped handled by Mustache’s tag types).
  - `on_missing` policy.
  - `partials` strategy.
  - `whitespace` mode.
- **Context helpers**
  - Build an empty context, insert values, or convert from serde data.
- **Cache utilities**
  - Read-only hit/miss stats to verify caching works in tests.

## Behavior notes

- Booleans in variable position (`{{ok}}`) are rendered as `"true"`/`"false"` to avoid upstream Mustache panics; for section semantics use `{{#ok}}...{{/ok}}`.
- Partials expansion is a single pass before compilation. It’s intentionally simple and fast; nested partials work if their tags appear after expansion (typical in templating trees).

## Limitations (by design)

- No runtime template discovery or network I/O.
- No built-in filters/helpers yet (reserved for a follow-up crate or optional module).
- One-pass partial expansion (good for codegen; avoids surprising recursion).

## Typical workflow

1. Build a Context from your data (via serde or by inserting values).
2. Choose a TemplateSource (inline or file).
3. Set RenderOptions (partials directory or map, missing-key policy, whitespace).
4. Render; optionally reuse the same Engine for repeated calls to benefit from parsing cache.

## Testing strategy (shipped examples)

- Inline render: verifies minimal end-to-end path.
- File + partials dir: checks partial resolution and path rules.
- Missing-key policy: validates Error, Empty, and KeepTag.
- Whitespace: ensures Keep/TrimLines/SmartIndent produce expected strings.
- Caching determinism: asserts 1 miss, then hits with identical inputs.
- Partials map override: proves map-first precedence over on-disk partials.

## Roadmap

- Optional filters and helpers with a stable, composable API.
- Extended partials precedence strategies as needed.
- Snapshot-style test helpers for codegen use cases.

## Security & support

- The crate reads from local files only (when you choose file/dir partials).
- Please report issues with reproducible inputs and expected/actual output so we can keep behavior tight and predictable.