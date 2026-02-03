# AllTheSkills

[![Crates.io](https://img.shields.io/crates/v/alltheskills)](https://crates.io/crates/alltheskills)
[![Docs.rs](https://docs.rs/alltheskills/badge.svg)](https://docs.rs/alltheskills)
[![CI](https://github.com/alltheskills/alltheskills/actions/workflows/ci.yml/badge.svg)](https://github.com/alltheskills/alltheskills/actions)
[![MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A Rust library and CLI for reading and installing AI skills from various sources including Claude, Cline, OpenClaw, Moltbot, Vercel, Cloudflare, and more.

## Features

- **Unified Skill Format** - Read skills from multiple AI assistants with a single API
- **Multiple Providers** - Support for Claude, Cline, OpenClaw, Moltbot (formerly ClawdBot), Vercel AI SDK, Cloudflare Workers AI, Roo Code, OpenAI Codex, Kilo Code, and GitHub
- **Flexible Installation** - Install skills from GitHub repositories or local directories
- **Extensible** - Trait-based provider architecture for adding new sources
- **Async** - Built on tokio for asynchronous operations

## Installation

### Library Only

```bash
cargo add alltheskills
```

### With CLI

```bash
cargo install alltheskills-cli
```

Or download pre-built binaries from the [releases page](https://github.com/alltheskills/alltheskills/releases).

## Quick Start

### Library Usage

```rust
use alltheskills::{SkillReader, AllSkillsConfig, providers::ClaudeProvider};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let config = AllSkillsConfig::default();
    let mut reader = SkillReader::new(config);

    // Add providers for sources you want to read from
    reader.add_provider(ClaudeProvider);
    reader.add_provider(alltheskills::providers::LocalProvider);

    // List all skills
    let skills = reader.list_all_skills().await?;
    println!("Found {} skill(s)", skills.len());

    // Search for skills
    let git_skills = reader
        .search_skills(|s| s.name.to_lowercase().contains("git"))
        .await?;

    Ok(())
}
```

### CLI Usage

```bash
# List all skills from all configured sources
alltheskills list

# Search for skills
alltheskills search git

# Install a skill from GitHub
alltheskills install https://github.com/user/skill-repo

# Install a skill from a local directory
alltheskills install /path/to/skill --target ./my-skills

# Get detailed information about a skill
alltheskills info my-skill

# Export alltheskills as a Claude skill
alltheskills export-as-skill --output-dir ~/.claude/skills/alltheskills-manager

# Add a new skill source
alltheskills add-source my-source /path/to/skills --source-type local

# Show configuration
alltheskills config
```

## Supported Sources

| Source | Location | Format |
|--------|----------|--------|
| Claude Code | `~/.claude/skills/` | `claude.json`, `skill.md`, `README.md` |
| Cline | `~/.cline/skills/` | `cline.json`, `custom-instructions.md`, `README.md` |
| OpenClaw | `~/.openclaw/skills/` | `skill.json`, `README.md` |
| **Moltbot** (formerly ClawdBot) | `~/.moltbot/skills/` or `~/.clawdbot/skills/` | `manifest.json`, `SKILL.md`, `README.md` |
| Vercel AI SDK | `~/.vercel/ai/skills/` | `skill.json`, `ai.config.json` |
| Cloudflare Workers AI | `~/.cloudflare/workers/skills/` | `worker.js/ts`, `wrangler.toml` |
| Roo Code | `~/.roo/skills/` | `roo.json`, `.roomodes`, `README.md` |
| OpenAI Codex | `~/.codex/skills/` | JSON |
| Kilo Code | `~/.kilo/skills/` | YAML + markdown |
| GitHub | Repository URLs | Any format |
| Local | Custom paths | Any format |

### Moltbot (ClawdBot) Skills

Moltbot (formerly known as ClawdBot) uses a `SKILL.md` format with a `manifest.json` configuration:

```text
~/.moltbot/skills/my-skill/
├── manifest.json    # Skill metadata and commands
├── SKILL.md         # Main skill instructions
├── index.ts         # Implementation (optional)
└── README.md        # Documentation (optional)
```

The library automatically detects both the new `.moltbot` path and the legacy `.clawdbot` path.

### Environment Variables

Each provider checks for an environment variable to override the default skill directory:

| Provider | Environment Variable | Default Path |
|----------|---------------------|--------------|
| Claude | `CLAUDE_SKILLS_DIR` | `~/.claude/skills` |
| Cline | `CLINE_SKILLS_DIR` | `~/.cline/skills` |
| OpenClaw | `OPENCLAW_SKILLS_DIR` | `~/.openclaw/skills` |
| Roo Code | `ROO_SKILLS_DIR` | `~/.roo/skills` |
| **Moltbot** | `MOLTBOT_SKILLS_DIR` or `CLAWDBOT_SKILLS_DIR` | `~/.moltbot/skills` |
| Vercel | `VERCEL_SKILLS_DIR` | `~/.vercel/ai/skills` |
| Cloudflare | `CLOUDFLARE_SKILLS_DIR` | `~/.cloudflare/workers/skills` |
| Kilo Code | `KILO_SKILLS_DIR` | `~/.kilo/skills` |
| OpenAI Codex | `CODEX_SKILLS_DIR` | `~/.codex/skills` |

## Custom Providers

Create custom providers by implementing the [`SkillProvider`] trait:

```rust
use async_trait::async_trait;
use alltheskills::{Skill, SkillProvider, SkillSource, SourceConfig, SourceType};

pub struct MyProvider;

#[async_trait]
impl SkillProvider for MyProvider {
    fn name(&self) -> &'static str {
        "My AI Skills"
    }

    fn source_type(&self) -> SourceType {
        SourceType::Custom("my-ai".to_string())
    }

    fn can_handle(&self, source: &SkillSource) -> bool {
        matches!(source, SkillSource::Local { path } if path.to_string_lossy().contains("my-ai"))
    }

    async fn list_skills(&self, config: &SourceConfig) -> Result<Vec<Skill>, alltheskills::Error> {
        // Implementation here
        Ok(vec![])
    }

    async fn read_skill(&self, skill: &Skill) -> Result<String, alltheskills::Error> {
        // Implementation here
        Ok(String::new())
    }

    async fn install(&self, source: SkillSource, target: std::path::PathBuf) -> Result<Skill, alltheskills::Error> {
        // Implementation here
        unimplemented!()
    }
}
```

## Configuration

Create a `~/.config/alltheskills/alltheskills.toml` file:

```toml
version = 1
default_scope = "user"
install_dir = ".alltheskills"
cache_dir = ".alltheskills/cache"
```

Or use the CLI to manage configuration:

```bash
# Show current configuration
alltheskills config

# Show config file path
alltheskills config --path

# Add a source
alltheskills add-source my-skills ~/my-skills --source-type local

# Remove a source
alltheskills remove-source my-skills
```

## Library Features

### Re-exported Providers

All providers are re-exported for convenient access:

```rust
use alltheskills::providers::{
    ClaudeProvider,
    ClineProvider,
    RooProvider,
    OpenClawProvider,
    MoltbotProvider,
    VercelProvider,
    CloudflareProvider,
    GitHubProvider,
    LocalProvider,
};
```

### Skill Detection

The [`KnownSources`] struct provides methods to detect skill directories:

```rust
use alltheskills::providers::KnownSources;

if let Some(path) = KnownSources::claude_skills_dir() {
    println!("Claude skills found at: {}", path.display());
}

if let Some(path) = KnownSources::moltbot_skills_dir() {
    println!("Moltbot skills found at: {}", path.display());
}
```

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](https://github.com/alltheskills/alltheskills/blob/main/CONTRIBUTING.md) for details.

## License

MIT License - see [LICENSE](LICENSE) for details.
