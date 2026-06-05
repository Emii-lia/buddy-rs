# buddy-rs

`buddy-rs` is a Rust-based cli tools that helps you through your Linux/macOS terminal journey. It provides a set of tools to enhance your command-line experience, including command explanations, feedback on command execution, and more.
## How It Works

1.  **Installation**: When you run `buddy install`, it configures your shell to call `buddysh` after each command execution.
2.  **Hooking**: Every time a command is executed in your shell, the hook (`buddysh`) captures the command data,
3.  **Communication**: `buddysh` sends this data to the `buddyd` daemon via a Unix domain socket.
4.  **Reaction**: The daemon processes the event and can provide feedback or perform actions based on the command result.
5.  **Explanation**: ask buddy to explain a command for you.
## Getting Started

### Prerequisites

- Rust (latest stable)
- A Unix-like environment (Linux/macOS)

### Installation

To install the buddy hooks into your shell:

```bash
cargo install buddy-rs
```
Four commands will be installed after the installation:
- `buddy` - to manage buddy (install, uninstall, explain)
- `buddyd` - the buddy daemon that processes events
- `buddysh` - the shell hook that captures command execution details
- `buddy-rs` - buddy greeting and initialiser

Then initialise buddy reaction:

```bash
buddy init
```

### Usage


#### Explain a command
You can ask buddy to explain a command for you using:
```bash
buddy explain "ls -la"
```
You can also specify an assistant to give you the explanation:
```bash
buddy explain "rm -rf /" --assistant gremlin
```

Available assistants:
- `buddy` (default): A bro, a buddy, a friend who gives casual, laid-back explanations with a good dose of sarcasm and humour.
- `boss`: A strict, straightforward boss who gives concise, no-nonsense explanations.
- `mochi`: A helpful, gentle close friend who provides clear explanations with a touch of humour.
- `mrbruno`: A math teacher who is known for his ability to explain complex concepts in a clear and funny way.
- `gremlin`: A mischievous gremlin who loves to cause chaos and confusion.
- `spock`: A logical and precise Vulcan who provides clear, concise explanations.

### Feedback on command execution

Buddy will give you its feedback on some commands:
- `build`: build commands
- `dev`: some popular development tools commands
- `file-operation`: touch, mkdir, rm, mv, cp, ln, rmdir
- `git`: git commands
- `package-manager`: your system package manager commands
- `search`: find, lookup, grep, rg, fd, locate.
- `fail`: any command that fails with a non-zero exit code.

### Uninstallation
To uninstall the buddy hooks from your shell:
```bash
buddy uninstall
```

P.S: This will remove the hooks from your shell but does not remove the buddy cargo installations.

### Manage buddy daemon and hooks
Buddy daemon is handled by `buddyd` and hooks are handled by `buddysh`.
You can manage buddy service with `buddy.service`.

```bash
systemctl --user <start|stop|restart|status> buddy.service
```

The buddy service will be started automatically after installation.

To manage buddy shell hook, you can modify the `preexec` and `postexec` functions in:
- `~/.config/buddy/buddy.bash` for `bash` which is sourced by `~/.bashrc`
- `~/.config/buddy/buddy.zsh` for `zsh` which is sourced by `~/.zshrc`
- `~/.config/fish/functions/buddy.fish` for `fish

For fish, you can remove the `preexec` and `postexec` functions to uninstall the hooks with:

```bash
functions -e buddy_preexec
functions -e buddy_postexec
```

## License

MIT