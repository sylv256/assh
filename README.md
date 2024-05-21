# `assh`
`assh` is a creative take on shells. `assh` massively simplifies the role of the shell while providing a fully-fleshed scripting language and REPL. Read more in the [design philosophy](#design-philosophy).

## How do you pronounce `assh`?
There are a few ways you can pronounce `assh`:
 - "ass" where the "h" is silent.
 - "ash" (not to be confused with the shell `ash`)
 - "ashes"
 - "sash"

## What does `assh` stand for?
It stands for many things, like:
 - Agony & Suffering Shell
 - Cool-ass Shell
 - 42 (The meaning of life, the universe, and everything)
 - …and whatever you want.

## Design Philosophy

### `assh` will always be FOSS.
This is ultimately my passion project, and I seek no commercial revenue from it. Additionally, do whatever the heck you want with it provided I'm credited and the GPLv3 license is maintained under my copyright. Fork it, tinker, do whatever. `assh` is yours, always.

### `assh` does one thing and does it well.
`assh` does one thing: execute code. `assh` does not feel naturally esoteric and cumbersome, as `assh` describes a meaningful syntax compared to shells like `bash`.

#### `assh`'s Purpose
`assh` is the command interpreter. `assh` has little syntax because its core purpose is to executes code. However, `assh` provides basic I/O redirection and forking.

#### `assh` as a Scripting Language
`assh` implements a scripting language that can be invoked via `assh <path>` or with a shebang.

### `assh` is lightweight.
`assh` takes as little overhead as possible. We use optimization level `3` to optimize for speed as well as a number of strategies for reducing binary size. Read more [here](https://github.com/johnthagen/min-sized-rust). Ultimately, `assh` remains fast and tiny.

### `assh` is written in Rust.
`assh` will always be written in a principled, memory-safe systems language. This allows the shell to make more guarantees about memory-safety.

### If `assh` panics/coredumps, it will do so without ending the TTY/terminal first.
`assh` has a custom panic handler to ensure that when something goes wrong, you know about it. `assh` also stores its logs per-user so you have direct access at all times.
