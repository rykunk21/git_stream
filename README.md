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
````

This README template:
1. Clearly communicates the project's value proposition
2. Highlights key features and capabilities
3. Provides quick start instructions
4. Explains the dataset structure and usage
5. Addresses privacy concerns
6. Includes monetization information
7. Provides links to further documentation

You can customize this template based on your specific implementation details and additional features.


================================================================================

	*You*

how do I redo in neovim?

--------------------------------------------------------------------------------

	*Augment*

Based on the provided configuration files, here are the standard ways to redo in Neovim:

1. Press `<Ctrl-r>` in normal mode
2. Type `R` in normal mode

The first option (`<Ctrl-r>`) is the most commonly used method and works in both Vim and Neovim by default.

Note that redo is the opposite of undo (`u`), and you can only redo changes that were previously undone. If you make a new change after an undo, you won't be able to redo the previously undone changes as the redo stack will be cleared.

You can also use a count before the redo command to redo multiple changes at once, for example:
- `5<Ctrl-r>` will redo 5 changes
- `10<Ctrl-r>` will redo 10 changes


================================================================================

	*You*

generate me a readme for a project named git_stream, a rust based project that watches your git history and turns it into actionable datasets that can be queried

--------------------------------------------------------------------------------

	*Augment*

I'll create a README that focuses on the technical and data processing aspects of Git Stream.

<augment_code_snippet path="git_stream/README.md" mode="EDIT">
# Git Stream

> A Rust-powered Git history analyzer that transforms repository data into queryable datasets

[![Crates.io](https://img.shields.io/crates/v/git_stream.svg)](https://crates.io/crates/git_stream)
[![Docs.rs](https://docs.rs/git_stream/badge.svg)](https://docs.rs/git_stream)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Git Stream processes your Git repository's history into structured datasets, enabling powerful querying and analysis capabilities. Perfect for project analytics, ML training data generation, and development pattern analysis.

## Features

- 🔄 Real-time Git event monitoring
- 📊 Structured data extraction
  - Commit patterns
  - Code change analytics
  - Developer collaboration metrics
  - Project velocity indicators
- 🚀 High-performance Rust implementation
- 📝 Multiple export formats (Parquet, JSON, CSV)
- 🔍 Rich query API

## Installation

```bash
cargo install git_stream
```

## Quick Start

```bash
# Initialize Git Stream in your repository
git_stream init

# Start monitoring
git_stream watch

# Export data
git_stream export --format parquet
```

## Usage

### Basic Configuration

```toml
# git_stream.toml
[stream]
repository = "."
watch_interval = "5m"
export_format = "parquet"

[filters]
exclude_paths = ["target/", "node_modules/"]
start_date = "2023-01-01"
```

### Query API

```rust
use git_stream::{Repository, Query};

let repo = Repository::new("./my-project")?;
let dataset = repo.query()
    .since("2024-01-01")
    .author("alice@example.com")
    .file_pattern("src/**/*.rs")
    .build()?;

// Get commit frequency by author
let stats = dataset.analyze_commit_patterns()?;

// Export results
dataset.export("analysis.parquet")?;
```

## Data Schema

### Commit Data
```rust
struct CommitData {
    hash: String,
    author: String,
    timestamp: DateTime<Utc>,
    message: String,
    changes: Vec<FileChange>,
    metrics: CommitMetrics,
}
```

### File Changes
```rust
struct FileChange {
    path: String,
    additions: u32,
    deletions: u32,
    content_delta: String,
}
```

## CLI Commands

- `git_stream init` - Initialize monitoring
- `git_stream watch` - Start real-time monitoring
- `git_stream query` - Query the dataset
- `git_stream export` - Export data
- `git_stream analyze` - Run built-in analytics

## Performance

- Efficient incremental processing
- Parallel data extraction
- Optimized storage format
- Low memory footprint

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [git2-rs](https://github.com/rust-lang/git2-rs)
- Uses [Apache Arrow](https://arrow.apache.org/) for data processing
- Inspired by Git forensics tools and ML data pipelines
</augment_code_snippet>

This README focuses on the technical aspects and provides clear examples of how to use the library both through the CLI and as a Rust library. It includes:

1. Clear feature overview
2. Installation and quick start guide
3. Configuration examples
4. Code samples for the Query API
5. Data schema documentation
6. Performance considerations
7. Contributing guidelines

The structure emphasizes Git Stream's role as a data processing tool while maintaining a professional and technical tone appropriate for a Rust project.


================================================================================

	*You*

change the rust code section to include an example of a python code section api as well

--------------------------------------------------------------------------------

	*Augment*

I'll update the README to include both Rust and Python API examples.

<augment_code_snippet path="git_stream/README.md" mode="EDIT">
# Git Stream

[previous sections remain the same...]

## Usage

### Basic Configuration

```toml
# git_stream.toml
[stream]
repository = "."
watch_interval = "5m"
export_format = "parquet"

[filters]
exclude_paths = ["target/", "node_modules/"]
start_date = "2023-01-01"
```

### API Examples

#### Rust API

```rust
use git_stream::{Repository, Query};

let repo = Repository::new("./my-project")?;
let dataset = repo.query()
    .since("2024-01-01")
    .author("alice@example.com")
    .file_pattern("src/**/*.rs")
    .build()?;

// Get commit frequency by author
let stats = dataset.analyze_commit_patterns()?;

// Export results
dataset.export("analysis.parquet")?;
```

#### Python API

```python
from git_stream import Repository, Query

# Initialize repository
repo = Repository("./my-project")

# Create and execute query
dataset = (
    repo.query()
    .since("2024-01-01")
    .author("alice@example.com")
    .file_pattern("src/**/*.rs")
    .build()
)

# Analyze commit patterns
stats = dataset.analyze_commit_patterns()

# Export as pandas DataFrame
df = dataset.to_pandas()

# Or export to file
dataset.export("analysis.parquet")

# Advanced analysis example
commit_metrics = (
    dataset.group_by("author")
    .aggregate({
        "commits": "count",
        "lines_changed": "sum",
        "files_modified": "mean"
    })
    .sort_values("commits", ascending=False)
)
```

