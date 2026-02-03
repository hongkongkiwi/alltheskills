# AllSkills

A Rust library and CLI tool for reading and installing AI skills from various sources.

## Supported Sources

| Source | Location Pattern | Format |
|--------|-----------------|--------|
| Claude Code | `~/.claude/skills/`, `~/.claude/plugins/` | JSON/YAML + markdown |
| Cline | `~/.cline/skills/`, VS Code extensions | JSON config + files |
| OpenClaw | `~/.openclaw/skills/`, VS Code extensions | JSON config + files |
| Vercel AI SDK | `~/.vercel/ai/skills/`, `~/.ai/skills/` | skill.json, ai.config.json |
| Cloudflare Workers AI | `~/.cloudflare/workers/skills/` | worker.js/ts, wrangler.toml |
| Roo Code | `~/.roo/skills/` | JSON/YAML |
| OpenAI Codex | `~/.codex/skills/`, project config | JSON |
| Kilo Code | `~/.kilo/skills/` | YAML + markdown |
| GitHub | Direct repo URLs | Any skill format |
| Local paths | Custom directories | Format-agnostic |

## Self-Exposing Skill

The CLI can export itself as a Claude skill using:

```bash
alltheskills export-as-skill --output ~/.claude/skills/alltheskills-manager
```

This creates a skill that wraps alltheskills functionality, allowing you to manage skills from within Claude.

## Installation

```bash
cargo install alltheskills-cli
```

## Usage

```bash
# List all skills
alltheskills list

# Install from GitHub
alltheskills install --from https://github.com/user/skill-repo

# Search for skills
alltheskills search git

# Get skill information
alltheskills info my-skill

# Add a custom source
alltheskills add-source --name my-skills --path /path/to/skills --source-type local

# Show configuration
alltheskills config show

# Export as Claude skill
alltheskills export-as-skill --output ~/.claude/skills/
```

## Library Usage

```rust
use alltheskills::{SkillReader, AllSkillsConfig, providers::ClaudeProvider};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let config = AllSkillsConfig::default();
    let mut reader = SkillReader::new(config);

    reader.add_provider(ClaudeProvider);

    let skills = reader.list_all_skills().await?;
    println!("Found {} skills", skills.len());

    Ok(())
}
```

## Configuration

Configuration is stored in `~/.config/alltheskills/alltheskills.toml`:

```toml
version = 1
default_scope = "user"
install_dir = ".alltheskills"
cache_dir = ".alltheskills/cache"
```

## License

MIT
