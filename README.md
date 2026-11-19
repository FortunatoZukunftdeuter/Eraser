# Eraser

**Eraser** is a lightweight command-line utility written in Rust that clears your terminal 
screen before and/or after executing a command.

## How It Works

Eraser determines whether to clear the screen based on the casing of the 
executable name (or alias) used to invoke it:

* **Lowercase** (e.g., `e`): Clears only the **visible** terminal screen.
* **Uppercase** (e.g., `E`): Clears the **entire** terminal scrollback history.
* **Non-alphabetic** (e.g., `3`): Does **not** clear the screen beforehand.

### Command Execution & Trailing Triggers

1. **Command Wrapping:** Any arguments following the binary name are executed as a standard 
shell command.
2. **Post-Execution Clearing:** If the **final argument** matches an Eraser trigger, it will execute a second clear after the command finishes. 
    > *Note: When invoked with a non-alphabetic name, Eraser always treats the final argument as a 
post-execution clear trigger.*

---

## Examples

Assuming you have aliased or symlinked the binary to `e`, `E`, and `3`:

**Clear visible screen, then run a command:**
```bash
e cargo run
```

**Clear entire scrollback history, then run a command:**
```bash
E cargo run
```

**Clear visible screen, run a command, then clear entire history:**
```bash
e cargo run E
```

**Clear entire history, run a command, then clear visible screen:**
```bash
E cargo run e
```

**Run a command without clearing first, then clear entire history:**
```bash
3 cargo run E
```

**Run a command without clearing first, then clear visible screen:**
```bash
3 cargo run e
```
