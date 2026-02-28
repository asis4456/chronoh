# ChronoH

Time-aware harness for long-running AI agents.

## Quick Start

```bash
# Initialize project
cargo run -- init --name my-project

# Develop
cd my-project
cargo run -- dev

# Check status
cargo run -- status
```

## Architecture

ChronoH provides:
- **StateEngine**: Persistent state with Sled KV store
- **Hook System**: Lifecycle hooks for control
- **4-Primitive Tools**: read, write, edit, bash
- **Role-Based Agents**: Initializer, Coder, Reviewer

## License

MIT
