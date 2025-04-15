# Git Stream

> Transform your Git history into valuable, queryable datasets for AI training

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/git_stream.svg)](https://crates.io/crates/git_stream)

Git Stream enables open source projects to monetize their development history by converting Git repositories into structured, queryable datasets suitable for AI training.

## Features

- 🔄 Real-time Git history monitoring
- 📊 Structured data extraction from commits, pull requests, and code changes
- 🔍 Rich query interface for dataset consumers
- 🔐 Privacy-focused data filtering and anonymization options
- 💰 Built-in marketplace integration for dataset monetization
- 📈 Usage analytics and revenue tracking

## Quick Start

```bash
# Install Git Stream
cargo install git_stream

# Initialize tracking for a repository
git_stream init

# Start the monitoring service
git_stream watch

# Export your dataset
git_stream export --format jsonl
```

## Dataset Structure

Git Stream captures:
- Commit metadata and messages
- Code changes and diffs
- File evolution over time
- Developer interaction patterns
- Pull request discussions
- Issue tracking data

## Usage Examples

```rust
// Query your dataset
let dataset = GitStream::new("./my-repo")
    .filter_since("2023-01-01")
    .exclude_private_data()
    .build();

// Export for training
dataset.export("./output", Format::Parquet);
```

## Monetization

1. Configure your pricing model in `git_stream.toml`
2. Deploy your dataset to the marketplace
3. Track usage and receive payments

## Privacy & Security

- Configurable data anonymization
- Sensitive information filtering
- Compliance with data protection regulations
- Opt-out mechanisms for contributors

## Documentation

Visit [docs.gitstream.dev](https://docs.gitstream.dev) for:
- Complete API reference
- Integration guides
- Best practices
- Monetization strategies

## Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
