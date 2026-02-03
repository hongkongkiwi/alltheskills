# AI Skills Consolidator - Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Create a Rust package and CLI tool for reading and installing AI skills from various sources (Claude, Codex, Roo, Cline, GitHub, Kilo, etc.) with support for project, user, and global scopes.

**Architecture:**
- Core library (`allskills`) with trait-based providers for each AI assistant
- CLI tool (`allskills-cli`) for command-line interaction
- Unified skill format with adapters for different sources
- Git-based installation with support for raw URLs and local paths

**Tech Stack:** Rust 2024 Edition, tokio async runtime, git2 for Git operations, serde for JSON/YAML, clap for CLI

---

## Known Skill Sources to Support

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

---

## Task 1: Initialize Rust Workspace

**Files:**
- Create: `Cargo.toml`
- Create: `crates/allskills/Cargo.toml`
- Create: `crates/allskills/src/lib.rs`
- Create: `crates/allskills-cli/Cargo.toml`
- Create: `crates/allskills-cli/src/main.rs`

**Step 1: Create workspace Cargo.toml**

```toml
[workspace]
members = [
    "crates/allskills",
    "crates/allskills-cli",
]

[workspace.package]
version = "0.1.0"
edition = "2024"
authors = ["AllSkills Contributors"]
```

**Step 2: Create core library Cargo.toml**

```toml
[package]
name = "allskills"
version.workspace = true
edition.workspace = true

[dependencies]
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
serde_yaml = "0.9"
thiserror = "1.0"
anyhow = "1.0"
futures = "0.3"
url = "2.5"
git2 = { version = "0.18", optional = true }
async-trait = "0.1"

[features]
git = ["dep:git2"]
default = ["git"]
```

**Step 3: Create library skeleton**

```rust
// crates/allskills/src/lib.rs
pub mod core;
pub mod providers;
pub mod types;
```

**Step 4: Create CLI skeleton**

```rust
// crates/allskills-cli/src/main.rs
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "allskills")]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    List,
    Install,
    Search,
    Info,
}

fn main() {
    let args = Args::parse();
    println!("AllSkills CLI");
}
```

**Step 5: Commit**

```bash
git add .
git commit -m "chore: initialize workspace with core and cli crates"
```

---

## Task 2: Define Core Types

**Files:**
- Create: `crates/allskills/src/types/mod.rs`
- Create: `crates/allskills/src/types/skill.rs`
- Create: `crates/allskills/src/types/source.rs`
- Create: `crates/allskills/src/types/config.rs`

**Step 1: Create skill type**

```rust
// crates/allskills/src/types/skill.rs
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: Option<String>,
    pub source: SkillSource,
    pub source_type: SourceType,
    pub path: PathBuf,
    pub installed_at: chrono::DateTime<chrono::Utc>,
    pub metadata: SkillMetadata,
    pub format: SkillFormat,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceType {
    Claude,
    Cline,
    RooCode,
    OpenAICodex,
    KiloCode,
    GitHub,
    Local,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SkillFormat {
    ClaudeSkill,
    ClaudePlugin,
    ClineSkill,
    RooSkill,
    CodexSkill,
    GenericMarkdown,
    GenericJson,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SkillMetadata {
    pub author: Option<String>,
    pub tags: Vec<String>,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub license: Option<String>,
    pub readme: Option<String>,
    pub requirements: Vec<String>,
}
```

**Step 2: Create source type**

```rust
// crates/allskills/src/types/source.rs
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SkillSource {
    /// Source is a local directory
    Local {
        path: PathBuf,
    },
    /// Source is a GitHub repository
    GitHub {
        owner: String,
        repo: String,
        subdir: Option<String>,
        branch: Option<String>,
    },
    /// Source is a raw URL
    Remote {
        url: String,
        headers: Vec<(String, String)>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceConfig {
    pub name: String,
    pub source_type: SourceType,
    pub enabled: bool,
    pub scope: SkillScope,
    pub priority: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SkillScope {
    Global,
    User,
    Project,
}
```

**Step 3: Create config type**

```rust
// crates/allskills/src/types/config.rs
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllSkillsConfig {
    pub version: u8,
    pub default_scope: SkillScope,
    pub sources: Vec<SourceConfig>,
    pub install_dir: PathBuf,
    pub cache_dir: PathBuf,
}

impl Default for AllSkillsConfig {
    fn default() -> Self {
        Self {
            version: 1,
            default_scope: SkillScope::User,
            sources: Vec::new(),
            install_dir: PathBuf::from(".allskills"),
            cache_dir: PathBuf::from(".allskills/cache"),
        }
    }
}
```

**Step 4: Create module file**

```rust
// crates/allskills/src/types/mod.rs
pub mod skill;
pub mod source;
pub mod config;

pub use skill::{Skill, SkillMetadata, SkillFormat, SourceType};
pub use source::{SkillSource, SourceConfig, SkillScope};
pub use config::AllSkillsConfig;
```

**Step 5: Commit**

```bash
git add crates/allskills/src/types/
git commit -m "feat: define core types for skills, sources, and config"
```

---

## Task 3: Create Provider Trait and Implement Source Detection

**Files:**
- Create: `crates/allskills/src/providers/mod.rs`
- Create: `crates/allskills/src/providers/trait.rs`
- Create: `crates/allskills/src/providers/detect.rs`

**Step 1: Create provider trait**

```rust
// crates/allskills/src/providers/trait.rs
use async_trait::async_trait;
use crate::types::{Skill, SkillSource, SourceConfig};

#[async_trait]
pub trait SkillProvider: Send + Sync {
    fn name(&self) -> &'static str;
    fn source_type(&self) -> crate::types::SourceType;
    fn can_handle(&self, source: &SkillSource) -> bool;
    async fn list_skills(&self, config: &SourceConfig) -> Result<Vec<Skill>, crate::Error>;
    async fn read_skill(&self, skill: &Skill) -> Result<String, crate::Error>;
    async fn install(&self, source: SkillSource, target: std::path::PathBuf) -> Result<Skill, crate::Error>;
}
```

**Step 2: Create source detection utilities**

```rust
// crates/allskills/src/providers/detect.rs
use std::path::{Path, PathBuf};
use crate::types::SourceType;

pub struct KnownSources;

impl KnownSources {
    pub fn claude_skills_dir() -> Option<PathBuf> {
        Self::detect_platform_path(
            "CLAUDE_SKILLS_DIR",
            "~/.claude/skills",
            "~/.claude/plugins/skills",
        )
    }

    pub fn cline_skills_dir() -> Option<PathBuf> {
        Self::detect_platform_path(
            "CLINE_SKILLS_DIR",
            "~/.cline/skills",
        )
    }

    pub fn roo_skills_dir() -> Option<PathBuf> {
        Self::detect_platform_path(
            "ROO_SKILLS_DIR",
            "~/.roo/skills",
        )
    }

    pub fn kilo_skills_dir() -> Option<PathBuf> {
        Self::detect_platform_path(
            "KILO_SKILLS_DIR",
            "~/.kilo/skills",
        )
    }

    fn detect_platform_path(env_key: &str, ...fallbacks: &str) -> Option<PathBuf> {
        // Implementation for detecting platform-specific paths
        None
    }
}
```

**Step 3: Create provider module**

```rust
// crates/allskills/src/providers/mod.rs
pub mod trait_;
pub mod detect;
pub mod claude;
pub mod cline;
pub mod roo;
pub mod github;
pub mod local;

pub use trait_::SkillProvider;
pub use detect::KnownSources;
```

**Step 4: Commit**

```bash
git add crates/allskills/src/providers/
git commit -m "feat: add provider trait and source detection"
```

---

## Task 4: Implement Claude Provider

**Files:**
- Create: `crates/allskills/src/providers/claude.rs`

**Step 1: Implement Claude skill provider**

```rust
// crates/allskills/src/providers/claude.rs
use async_trait::async_trait;
use std::path::PathBuf;
use serde_json;
use crate::types::{Skill, SkillFormat, SourceType, SourceConfig, SkillMetadata};
use super::SkillProvider;

pub struct ClaudeProvider;

#[async_trait]
impl SkillProvider for ClaudeProvider {
    fn name(&self) -> &'static str {
        "Claude Skills"
    }

    fn source_type(&self) -> SourceType {
        SourceType::Claude
    }

    fn can_handle(&self, source: &SkillSource) -> bool {
        matches!(source, SkillSource::Local { path } if path.to_string_lossy().contains("claude"))
    }

    async fn list_skills(&self, config: &SourceConfig) -> Result<Vec<Skill>, crate::Error> {
        let path = match &config.source_type {
            crate::types::SkillSource::Local { path } => path.clone(),
            _ => return Ok(vec![]),
        };

        let mut skills = Vec::new();

        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                let skill = self.parse_skill_dir(entry.path()).await?;
                if let Some(skill) = skill {
                    skills.push(skill);
                }
            }
        }

        Ok(skills)
    }

    async fn read_skill(&self, skill: &Skill) -> Result<String, crate::Error> {
        let content = std::fs::read_to_string(&skill.path)?;
        Ok(content)
    }

    async fn install(&self, source: SkillSource, target: PathBuf) -> Result<Skill, crate::Error> {
        // Copy skill files to target
        Ok(todo!())
    }
}

impl ClaudeProvider {
    async fn parse_skill_dir(&self, path: PathBuf) -> Result<Option<Skill>, crate::Error> {
        // Parse claude.json or skill.md to extract metadata
        Ok(None)
    }
}
```

**Step 2: Commit**

```bash
git add crates/allskills/src/providers/claude.rs
git commit -m "feat: implement Claude skill provider"
```

---

## Task 5: Implement GitHub Provider

**Files:**
- Create: `crates/allskills/src/providers/github.rs`

**Step 1: Implement GitHub provider**

```rust
// crates/allskills/src/providers/github.rs
use async_trait::async_trait;
use std::path::PathBuf;
use git2::Repository;
use crate::types::{Skill, SkillFormat, SourceType, SkillSource};
use super::SkillProvider;

pub struct GitHubProvider;

#[async_trait]
impl SkillProvider for GitHubProvider {
    fn name(&self) -> &'static str {
        "GitHub Repository"
    }

    fn source_type(&self) -> SourceType {
        SourceType::GitHub
    }

    fn can_handle(&self, source: &SkillSource) -> bool {
        matches!(source, SkillSource::GitHub { .. })
    }

    async fn list_skills(&self, config: &SourceConfig) -> Result<Vec<Skill>, crate::Error> {
        // Clone repo and list skills
        Ok(vec![])
    }

    async fn install(&self, source: SkillSource, target: PathBuf) -> Result<Skill, crate::Error> {
        let SkillSource::GitHub { owner, repo, subdir, branch } = source else {
            anyhow::bail!("Invalid source type");
        };

        let repo_url = format!("https://github.com/{}/{}", owner, repo);

        // Clone the repository
        let repo = Repository::clone(&repo_url, &target)?;

        // Checkout specific branch if provided
        if let Some(branch) = branch {
            let branch_ref = format!("refs/heads/{}", branch);
            repo.set_head(&branch_ref)?;
        }

        Ok(todo!())
    }
}
```

**Step 2: Commit**

```bash
git add crates/allskills/src/providers/github.rs
git commit -m "feat: implement GitHub provider for remote skill installation"
```

---

## Task 6: Implement Local Provider and Unified Reader

**Files:**
- Create: `crates/allskills/src/providers/local.rs`
- Modify: `crates/allskills/src/lib.rs`

**Step 1: Create local provider**

```rust
// crates/allskills/src/providers/local.rs
use async_trait::async_trait;
use std::path::PathBuf;
use crate::types::{Skill, SkillFormat, SourceType, SkillSource};
use super::SkillProvider;

pub struct LocalProvider;

#[async_trait]
impl SkillProvider for LocalProvider {
    fn name(&self) -> &'static str {
        "Local Directory"
    }

    fn source_type(&self) -> SourceType {
        SourceType::Local
    }

    fn can_handle(&self, source: &SkillSource) -> bool {
        matches!(source, SkillSource::Local { .. })
    }

    async fn list_skills(&self, config: &SourceConfig) -> Result<Vec<Skill>, crate::Error> {
        let path = match &config.source_type {
            SkillSource::Local { path } => path.clone(),
            _ => return Ok(vec![]),
        };

        let mut skills = Vec::new();

        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    let skill = self.parse_skill_dir(entry.path()).await?;
                    skills.push(skill);
                }
            }
        }

        Ok(skills)
    }

    async fn read_skill(&self, skill: &Skill) -> Result<String, crate::Error> {
        let content = std::fs::read_to_string(&skill.path)?;
        Ok(content)
    }

    async fn install(&self, source: SkillSource, target: PathBuf) -> Result<Skill, crate::Error> {
        todo!()
    }
}
```

**Step 2: Update lib.rs with unified reader**

```rust
// crates/allskills/src/lib.rs
pub mod core;
pub mod providers;
pub mod types;

pub use types::{Skill, SourceType, SkillScope, AllSkillsConfig};
pub use providers::{SkillProvider, KnownSources};

use std::path::PathBuf;
use futures::stream::{self, StreamExt};

/// Unified skill reader that queries all configured sources
pub struct SkillReader {
    config: AllSkillsConfig,
    providers: Vec<Box<dyn SkillProvider>>,
}

impl SkillReader {
    pub fn new(config: AllSkillsConfig) -> Self {
        Self {
            config,
            providers: Vec::new(),
        }
    }

    pub fn add_provider<P: SkillProvider + 'static>(&mut self, provider: P) {
        self.providers.push(Box::new(provider));
    }

    pub async fn list_all_skills(&self) -> Result<Vec<Skill>, Error> {
        let futures = self.providers.iter().map(|p| async {
            let config = crate::types::SourceConfig {
                name: p.name().to_string(),
                source_type: crate::types::SkillSource::Local {
                    path: PathBuf::new(),
                },
                enabled: true,
                scope: crate::types::SkillScope::User,
                priority: 0,
            };
            p.list_skills(&config).await
        });

        let results: Vec<Result<Vec<Skill>, _>> = stream::iter(futures)
            .buffer_unordered(10)
            .collect()
            .await;

        let mut all_skills = Vec::new();
        for result in results {
            match result {
                Ok(skills) => all_skills.extend(skills),
                Err(e) => tracing::warn!("Failed to list skills: {}", e),
            }
        }

        Ok(all_skills)
    }

    pub async fn search_skills<F>(&self, predicate: F) -> Result<Vec<Skill>, Error>
    where
        F: Fn(&Skill) -> bool + Send + Sync,
    {
        let all = self.list_all_skills().await?;
        Ok(all.into_iter().filter(predicate).collect())
    }
}
```

**Step 3: Commit**

```bash
git add crates/allskills/src/providers/local.rs crates/allskills/src/lib.rs
git commit -m "feat: add local provider and unified skill reader"
```

---

## Task 6b: Implement OpenClaw Provider

**Files:**
- Create: `crates/allskills/src/providers/openclaw.rs`

**Step 1: Implement OpenClaw skill provider**

```rust
// crates/allskills/src/providers/openclaw.rs
use async_trait::async_trait;
use std::path::PathBuf;
use crate::types::{Skill, SkillFormat, SourceType, SourceConfig, SkillMetadata};
use super::SkillProvider;

pub struct OpenClawProvider;

#[async_trait]
impl SkillProvider for OpenClawProvider {
    fn name(&self) -> &'static str {
        "OpenClaw Skills"
    }

    fn source_type(&self) -> SourceType {
        SourceType::Custom("openclaw".to_string())
    }

    fn can_handle(&self, source: &SkillSource) -> bool {
        matches!(source, SkillSource::Local { path } if path.to_string_lossy().contains("openclaw"))
    }

    async fn list_skills(&self, config: &SourceConfig) -> Result<Vec<Skill>, crate::Error> {
        let path = match &config.source_type {
            SkillSource::Local { path } => path.clone(),
            _ => return Ok(vec![]),
        };

        let mut skills = Vec::new();

        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                let skill = self.parse_skill_dir(entry.path()).await?;
                if let Some(skill) = skill {
                    skills.push(skill);
                }
            }
        }

        Ok(skills)
    }

    async fn read_skill(&self, skill: &Skill) -> Result<String, crate::Error> {
        let content = std::fs::read_to_string(&skill.path)?;
        Ok(content)
    }

    async fn install(&self, source: SkillSource, target: PathBuf) -> Result<Skill, crate::Error> {
        todo!()
    }
}

impl OpenClawProvider {
    async fn parse_skill_dir(&self, path: PathBuf) -> Result<Option<Skill>, crate::Error> {
        // Parse OpenClaw skill.json format
        // OpenClaw uses: skill.json with name, description, commands, prompt
        let config_path = path.join("skill.json");

        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let config: serde_json::Value = serde_json::from_str(&content)?;

            let skill = Skill {
                id: config["id"].as_str().unwrap_or_default().to_string(),
                name: config["name"].as_str().unwrap_or_default().to_string(),
                description: config["description"].as_str().unwrap_or_default().to_string(),
                version: config["version"].as_str().map(|s| s.to_string()),
                source: SkillSource::Local { path: path.clone() },
                source_type: SourceType::Custom("openclaw".to_string()),
                path,
                installed_at: chrono::Utc::now(),
                metadata: SkillMetadata {
                    author: config["author"].as_str().map(|s| s.to_string()),
                    tags: serde_json::from_value(config["tags"].take()).unwrap_or_default(),
                    ..Default::default()
                },
                format: SkillFormat::GenericJson,
            };

            return Ok(Some(skill));
        }

        Ok(None)
    }
}
```

**Step 2: Register provider in mod.rs**

```rust
// crates/allskills/src/providers/mod.rs
pub mod trait_;
pub mod detect;
pub mod claude;
pub mod cline;
pub mod openclaw;  // Add this
pub mod roo;
pub mod github;
pub mod local;
```

**Step 3: Commit**

```bash
git add crates/allskills/src/providers/openclaw.rs
git commit -m "feat: implement OpenClaw skill provider"
```

---

## Task 6c: Self-Exposing Skill Module (CLI as a Skill)

**Files:**
- Create: `crates/allskills-skill/` (standalone skill crate)
- Create: `crates/allskills-cli/src/skill_exporter.rs`

**Step 1: Create skill exporter that generates Claude-compatible skill format**

```rust
// crates/allskills-cli/src/skill_exporter.rs
use allskills::{SkillReader, AllSkillsConfig};

/// Generates a Claude skill that wraps allskills CLI functionality
pub fn generate_allskills_skill() -> String {
    r#"# AllSkills Manager

A skill for managing AI skills from Claude, Cline, Roo Code, OpenClaw, and more.

## Available Commands

### List all skills
```bash
allskills list
```

### Search for skills
```bash
allskills search <query>
```

### Install a new skill
```bash
allskills install --from <source>
```

### Get skill information
```bash
allskills info <skill-name>
```

## Configuration

You can configure allskills via `~/.config/allskills/allskills.toml`:

```toml
default_scope = "user"
install_dir = ".allskills"
cache_dir = ".allskills/cache"
```

## Environment Variables

- `ALLSKILLS_CONFIG_DIR` - Override config directory
- `ALLSKILLS_SKILLS_DIR` - Override skills directory
"#.to_string()
}
```

**Step 2: Create skill.json template**

```rust
// crates/allskills-cli/src/skill_exporter.rs (continued)

pub fn generate_skill_json(name: &str, description: &str) -> String {
    format!(r#"{{
  "name": "{}",
  "description": "{}",
  "version": "0.1.0",
  "author": "AllSkills",
  "commands": [
    {{
      "name": "list-skills",
      "description": "List all installed skills"
    }},
    {{
      "name": "search-skills",
      "description": "Search for skills"
    }},
    {{
      "name": "install-skill",
      "description": "Install a new skill from GitHub or local path"
    }}
  ]
}}"#, name, description)
}
```

**Step 3: Create CLI command to export as skill**

```rust
// crates/allskills-cli/src/commands/export_skill.rs
use std::path::PathBuf;

pub async fn export_as_skill(output_dir: Option<String>) -> Result<(), anyhow::Error> {
    let output = output_dir.unwrap_or_else(|| ".allskills/skill".to_string());
    let output_path = PathBuf::from(&output);

    // Create skill directory
    std::fs::create_dir_all(&output_path)?;

    // Write README.md
    let readme = super::skill_exporter::generate_allskills_skill();
    std::fs::write(output_path.join("README.md"), readme)?;

    // Write skill.json
    let skill_json = super::skill_exporter::generate_skill_json(
        "allskills-manager",
        "Manage AI skills from various sources"
    );
    std::fs::write(output_path.join("claude.json"), skill_json)?;

    println!("Skill exported to: {}", output);
    Ok(())
}
```

**Step 4: Add export command to main.rs**

```rust
#[derive(Subcommand)]
enum Commands {
    // ... existing commands ...
    /// Export allskills as a Claude skill
    ExportAsSkill {
        #[arg(short, long)]
        output: Option<String>,
    },
}
```

**Step 5: Commit**

```bash
git add crates/allskills-cli/src/skill_exporter.rs crates/allskills-cli/src/commands/export_skill.rs
git commit -m "feat: add self-exposing skill module for CLI"
```

---

## Task 7: Implement CLI Commands

**Files:**
- Modify: `crates/allskills-cli/src/main.rs`
- Create: `crates/allskills-cli/src/commands/list.rs`
- Create: `crates/allskills-cli/src/commands/install.rs`
- Create: `crates/allskills-cli/src/commands/search.rs`
- Create: `crates/allskills-cli/src/commands/info.rs`

**Step 1: Implement list command**

```rust
// crates/allskills-cli/src/commands/list.rs
use allskills::{SkillReader, AllSkillsConfig, SkillScope};

pub async fn list_skills(scope: Option<SkillScope>, source: Option<String>) -> Result<(), anyhow::Error> {
    let config = AllSkillsConfig::default();
    let mut reader = SkillReader::new(config);

    let skills = reader.list_all_skills().await?;

    for skill in skills {
        println!(
            "[{}] {} - {}",
            skill.source_type.as_str(),
            skill.name,
            skill.description
        );
    }

    Ok(())
}
```

**Step 2: Implement install command**

```rust
// crates/allskills-cli/src/commands/install.rs
use allskills::{SkillReader, AllSkillsConfig, SkillSource};

pub async fn install_skill(source: &str, target: &str) -> Result<(), anyhow::Error> {
    let source = if source.starts_with("https://github.com/") {
        // Parse GitHub URL
        SkillSource::GitHub { owner, repo, subdir: None, branch: None }
    } else {
        SkillSource::Local { path: source.into() }
    };

    println!("Installing skill from: {}", source);
    Ok(())
}
```

**Step 3: Implement search command**

```rust
// crates/allskills-cli/src/commands/search.rs
use allskills::{SkillReader, AllSkillsConfig};

pub async fn search_skills(query: &str) -> Result<(), anyhow::Error> {
    let config = AllSkillsConfig::default();
    let reader = SkillReader::new(config);

    let skills = reader.search_skills(|s| {
        s.name.to_lowercase().contains(&query.to_lowercase())
            || s.description.to_lowercase().contains(&query.to_lowercase())
    }).await?;

    for skill in skills {
        println!("{}: {}", skill.name, skill.description);
    }

    Ok(())
}
```

**Step 4: Update main.rs**

```rust
// crates/allskills-cli/src/main.rs
use clap::{Parser, Subcommand};
use commands::{list, install, search, info};

mod commands;

#[derive(Parser)]
#[command(name = "allskills")]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all installed skills
    List {
        #[arg(short, long)]
        scope: Option<String>,
        #[arg(short, long)]
        source: Option<String>,
    },
    /// Install a new skill
    Install {
        #[arg(short, long)]
        from: String,
        #[arg(short, long)]
        to: Option<String>,
    },
    /// Search for skills
    Search {
        query: String,
    },
    /// Show skill information
    Info {
        name: String,
    },
    /// Add a new skill source
    AddSource {
        #[arg(short, long)]
        path: String,
        #[arg(short, long)]
        source_type: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    match args.command {
        Commands::List { scope, source } => {
            list::list_skills(scope, source).await?;
        }
        Commands::Install { from, to } => {
            install::install_skill(&from, to.as_deref()).await?;
        }
        Commands::Search { query } => {
            search::search_skills(&query).await?;
        }
        Commands::Info { name } => {
            info::info_skill(&name).await?;
        }
        Commands::AddSource { path, source_type } => {
            println!("Adding source: {} ({})", path, source_type);
        }
    }

    Ok(())
}
```

**Step 5: Commit**

```bash
git add crates/allskills-cli/src/
git commit -m "feat: implement CLI commands for list, install, search, info"
```

---

## Task 8: Add Configuration and Source Management

**Files:**
- Create: `crates/allskills-cli/src/config.rs`
- Modify: `crates/allskills/src/types/config.rs`

**Step 1: Implement config file operations**

```rust
// crates/allskills-cli/src/config.rs
use std::path::PathBuf;
use allskills::{AllSkillsConfig, SourceConfig, SkillScope, SourceType};

const CONFIG_FILENAME: &str = "allskills.toml";

pub fn get_config_dir() -> PathBuf {
    // Use appropriate config dir based on OS
    #[cfg(unix)]
    {
        PathBuf::from(std::env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| "~/.config".to_string()))
    }
    #[cfg(windows)]
    {
        PathBuf::from(std::env::var("APPDATA").unwrap_or_else(|_| "~\\AppData".to_string()))
    }
}

pub fn load_config() -> Result<AllSkillsConfig, anyhow::Error> {
    let config_path = get_config_dir().join("allskills").join(CONFIG_FILENAME);

    if config_path.exists() {
        let content = std::fs::read_to_string(&config_path)?;
        Ok(toml::from_str(&content)?)
    } else {
        let config = AllSkillsConfig::default();
        save_config(&config)?;
        Ok(config)
    }
}

pub fn save_config(config: &AllSkillsConfig) -> Result<(), anyhow::Error> {
    let config_dir = get_config_dir().join("allskills");
    std::fs::create_dir_all(&config_dir)?;

    let content = toml::to_string(config)?;
    std::fs::write(config_dir.join(CONFIG_FILENAME), content)?;

    Ok(())
}

pub fn add_source(
    config: &mut AllSkillsConfig,
    name: &str,
    path: &str,
    source_type: &str,
    scope: SkillScope,
) {
    let source_config = SourceConfig {
        name: name.to_string(),
        source_type: SourceType::Custom(source_type.to_string()),
        enabled: true,
        scope,
        priority: config.sources.len() as i32,
    };
    config.sources.push(source_config);
}
```

**Step 2: Commit**

```bash
git add crates/allskills-cli/src/config.rs
git commit -m "feat: add configuration file management"
```

---

## Task 9: Add Error Handling and Error Types

**Files:**
- Create: `crates/allskills/src/error.rs`
- Modify: `crates/allskills/src/lib.rs`

**Step 1: Define error types**

```rust
// crates/allskills/src/error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {source}")]
    Io { source: std::io::Error },

    #[error("Git error: {source}")]
    Git { source: git2::Error },

    #[error("Parse error: {message}")]
    Parse { message: String },

    #[error("Source error: {source}")]
    Source { source: Box<Error> },

    #[error("Skill not found: {name}")]
    NotFound { name: String },

    #[error("Unsupported format: {format}")]
    UnsupportedFormat { format: String },

    #[error("Configuration error: {message}")]
    Config { message: String },

    #[error("Installation failed: {reason}")]
    Install { reason: String },
}

impl From<std::io::Error> for Error {
    fn from(source: std::io::Error) -> Self {
        Self::Io { source }
    }
}

impl From<git2::Error> for Error {
    fn from(source: git2::Error) -> Self {
        Self::Git { source }
    }
}
```

**Step 2: Update lib.rs**

```rust
// crates/allskills/src/lib.rs
pub mod core;
pub mod providers;
pub mod types;
pub mod error;

pub use types::{Skill, SourceType, SkillScope, AllSkillsConfig};
pub use providers::{SkillProvider, KnownSources};
pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;
```

**Step 3: Commit**

```bash
git add crates/allskills/src/error.rs crates/allskills/src/lib.rs
git commit -m "feat: add comprehensive error handling"
```

---

## Task 10: Create Example Usage and Documentation

**Files:**
- Create: `examples/basic_usage.rs`
- Create: `examples/README.md`
- Create: `README.md`

**Step 1: Create basic usage example**

```rust
// examples/basic_usage.rs
use allskills::{SkillReader, AllSkillsConfig, SkillScope};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Load configuration
    let config = AllSkillsConfig::default();

    // Create skill reader with all providers
    let mut reader = SkillReader::new(config);

    // List all skills from all sources
    println!("Listing all skills...");
    let skills = reader.list_all_skills().await?;

    for skill in skills {
        println!(
            "[{}] {} - {}",
            skill.source_type.as_str(),
            skill.name,
            skill.description
        );
    }

    // Search for skills
    println!("\nSearching for 'git' skills...");
    let git_skills = reader
        .search_skills(|s| s.name.to_lowercase().contains("git"))
        .await?;

    for skill in git_skills {
        println!("Found: {}", skill.name);
    }

    Ok(())
}
```

**Step 2: Create README**

```markdown
# AllSkills

A Rust library and CLI for reading and installing AI skills from various sources.

## Supported Sources

- Claude Code (`~/.claude/skills/`)
- Cline (`~/.cline/skills/`)
- OpenClaw (`~/.openclaw/skills/`)
- Roo Code (`~/.roo/skills/`)
- OpenAI Codex
- Kilo Code
- GitHub repositories
- Local directories

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

# Add a custom source
allskills add-source --path /path/to/skills --source-type claude
```

## Library Usage

```rust
use allskills::{SkillReader, AllSkillsConfig};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let config = AllSkillsConfig::default();
    let reader = SkillReader::new(config);

    let skills = reader.list_all_skills().await?;
    println!("Found {} skills", skills.len());

    Ok(())
}
```

## License

MIT
```

**Step 3: Commit**

```bash
git add examples/ README.md
git commit -m "docs: add examples and documentation"
```

---

## Task 11: Write Tests

**Files:**
- Create: `crates/allskills/tests/skills_test.rs`
- Create: `crates/allskills-cli/tests/cli_test.rs`

**Step 1: Create library tests**

```rust
// crates/allskills/tests/skills_test.rs
use allskills::{Skill, SkillFormat, SourceType, SkillMetadata, SkillScope};
use std::path::PathBuf;

#[test]
fn test_skill_creation() {
    let skill = Skill {
        id: "test-skill".to_string(),
        name: "Test Skill".to_string(),
        description: "A test skill".to_string(),
        version: Some("1.0.0".to_string()),
        source: allskills::SkillSource::Local {
            path: PathBuf::from("/test"),
        },
        source_type: SourceType::Local,
        path: PathBuf::from("/test/skill"),
        installed_at: chrono::Utc::now(),
        metadata: SkillMetadata::default(),
        format: SkillFormat::GenericMarkdown,
    };

    assert_eq!(skill.name, "Test Skill");
    assert_eq!(skill.source_type, SourceType::Local);
}

#[test]
fn test_skill_scope_ordering() {
    assert!(SkillScope::Global > SkillScope::User);
    assert!(SkillScope::User > SkillScope::Project);
}
```

**Step 2: Create CLI tests**

```rust
// crates/allskills-cli/tests/cli_test.rs
use assert_cmd::Command;

#[test]
fn test_cli_list_help() {
    let mut cmd = Command::cargo_bin("allskills").unwrap();
    cmd.arg("list").arg("--help").assert().success();
}

#[test]
fn test_cli_install_help() {
    let mut cmd = Command::cargo_bin("allskills").unwrap();
    cmd.arg("install").arg("--help").assert().success();
}

#[test]
fn test_cli_search_help() {
    let mut cmd = Command::cargo_bin("allskills").unwrap();
    cmd.arg("search").arg("--help").assert().success();
}
```

**Step 3: Add test dependencies to Cargo.toml**

```toml
[dev-dependencies]
assert_cmd = "2.0"
predicates = "3.0"
chrono = { version = "0.4", features = ["std"] }
```

**Step 4: Commit**

```bash
git add crates/allskills/tests/ crates/allskills-cli/tests/
git commit -m "test: add unit and integration tests"
```

---

## Task 12: Add GitHub Actions CI/CD

**Files:**
- Create: `.github/workflows/ci.yml`
- Create: `.github/workflows/release.yml`

**Step 1: Create CI workflow**

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install Rust
        run: rustup default stable
      - name: Build
        run: cargo build --workspace
      - name: Run tests
        run: cargo test --workspace
      - name: Check formatting
        run: cargo fmt --check
      - name: Clippy
        run: cargo clippy --workspace -- -D warnings
```

**Step 2: Create release workflow**

```yaml
name: Release

on:
  release:
    types: [created]

jobs:
  build-release:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    steps:
      - uses: actions/checkout@v4
      - name: Install Rust
        run: rustup default stable
      - name: Build release
        run: cargo build --release --package allskills-cli
      - name: Upload artifacts
        uses: actions/upload-artifact@v4
        with:
          name: allskills-cli-${{ matrix.os }}
          path: target/release/allskills
```

**Step 3: Commit**

```bash
git add .github/workflows/
git commit -m "ci: add GitHub Actions workflows"
```

---

## Task 13: Final Polish and Cleanup

**Files:**
- Modify: `Cargo.toml` (add workspace metadata)
- Create: `.gitignore`
- Create: `rust-toolchain.toml`

**Step 1: Create .gitignore**

```
target/
.env
*.log
.DS_Store
.allskills/
```

**Step 2: Create rust-toolchain**

```
[toolchain]
channel = "stable"
edition = "2024"
```

**Step 3: Commit**

```bash
git add .gitignore rust-toolchain.toml
git commit -m "chore: add gitignore and rust toolchain config"
```

---

## Plan Summary

| Task | Description | Files Created/Modified |
|------|-------------|----------------------|
| 1 | Initialize workspace | Cargo.toml, lib.rs, main.rs |
| 2 | Define core types | skill.rs, source.rs, config.rs |
| 3 | Create provider trait | trait.rs, detect.rs, mod.rs |
| 4 | Implement Claude provider | claude.rs |
| 5 | Implement GitHub provider | github.rs |
| 6 | Implement Local provider | local.rs, lib.rs |
| 6b | Implement OpenClaw provider | openclaw.rs |
| 6c | Self-exposing skill module | skill_exporter.rs, export_skill.rs |
| 7 | Implement CLI commands | main.rs, list.rs, install.rs, search.rs, info.rs |
| 8 | Configuration management | config.rs, types/config.rs |
| 9 | Error handling | error.rs |
| 10 | Examples and docs | examples/, README.md |
| 11 | Tests | tests/ |
| 12 | CI/CD | .github/workflows/ |
| 13 | Final polish | .gitignore, rust-toolchain.toml |

---

## Execution Options

**Plan complete and saved to `docs/plans/2026-02-03-ai-skills-consolidator.md`. Two execution options:**

**1. Subagent-Driven (this session)** - I dispatch fresh subagent per task, review between tasks, fast iteration

**2. Parallel Session (separate)** - Open new session with executing-plans, batch execution with checkpoints

**Which approach?**
