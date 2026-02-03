# AllSkills

A Rust library and CLI tool for reading and installing AI skills from various sources.

## Supported Sources

| Source | Location Pattern | Format |
|--------|-----------------|--------|
| Claude Code | `~/.claude/skills/`, `~/.claude/plugins/` | JSON/YAML + markdown |
| Cline | `~/.cline/skills/`, VS Code extensions | JSON config + files |
| OpenClaw | `~/.openclaw/skills/`, VS Code extensions | JSON config + files |
| Roo Code | `~/.roo/skills/` | JSON/YAML |
| OpenAI Codex | `~/.codex/skills/`, project config | JSON |
| Kilo Code | `~/.kilo/skills/` | YAML + markdown |
| GitHub (raw) | Direct repo URLs | Any skill format |
| Local paths | Custom directories | Format-agnostic |

## Self-Exposing Skill

The CLI can export itself as a Claude skill using:

```bash
allskills export-as-skill --output ~/.claude/skills/allskills-manager
```

This creates a skill that wraps allskills functionality, allowing you to manage skills from within Claude.

## Installation

```bash
cargo install allskills-cli
```

## Usage

```bash
# List all skills
allskills list

# Install from GitHub
allskills install --from https://github.com/user/skill-repo

# Search for skills
allskills search git

# Get skill information
allskills info my-skill

# Add a custom source
allskills add-source --name my-skills --path /path/to/skills --source-type local

# Show configuration
allskills config show

# Export as Claude skill
allskills export-as-skill --output ~/.claude/skills/
```

## Library Usage

```rust
use allskills::{SkillReader, AllSkillsConfig, providers::ClaudeProvider};

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

Configuration is stored in `~/.config/allskills/allskills.toml`:

```toml
version = 1
default_scope = "user"
install_dir = ".allskills"
cache_dir = ".allskills/cache"
```

## License

MIT
