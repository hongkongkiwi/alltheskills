use std::fs;

use std::path::{Path, PathBuf};

/// Initialize a new skill with boilerplate files
pub async fn init_skill(
    name: &str,
    source_type: Option<&str>,
    path: Option<&str>,
) -> Result<(), anyhow::Error> {
    let skill_dir = if let Some(p) = path {
        PathBuf::from(p).join(name)
    } else {
        PathBuf::from(name)
    };

    if skill_dir.exists() {
        anyhow::bail!("Directory already exists: {}", skill_dir.display());
    }

    // Determine skill type (default to claude)
    let skill_type = source_type.unwrap_or("claude").to_lowercase();

    println!("Creating new {} skill: {}", skill_type, name);

    // Create directory structure
    fs::create_dir_all(&skill_dir)?;

    // Generate files based on skill type
    match skill_type.as_str() {
        "claude" => create_claude_skill(&skill_dir, name).await?,
        "cline" => create_cline_skill(&skill_dir, name).await?,
        "cursor" => create_cursor_skill(&skill_dir, name).await?,
        "roo" | "roocode" => create_roo_skill(&skill_dir, name).await?,
        "openclaw" => create_openclaw_skill(&skill_dir, name).await?,
        "moltbot" => create_moltbot_skill(&skill_dir, name).await?,
        "codex" => create_codex_skill(&skill_dir, name).await?,
        "kilo" | "kilocode" => create_kilo_skill(&skill_dir, name).await?,
        "vercel" => create_vercel_skill(&skill_dir, name).await?,
        "cloudflare" => create_cloudflare_skill(&skill_dir, name).await?,
        _ => {
            // Generic skill
            create_generic_skill(&skill_dir, name).await?;
        }
    }

    println!("✅ Created skill at: {}", skill_dir.display());
    println!("\nNext steps:");
    println!("  1. Edit the generated files to add your skill logic");
    println!("  2. Install the skill: alltheskills install {}", skill_dir.display());
    println!("  3. Or add it as a source: alltheskills add-source {} {} --source-type {}",
        name, skill_dir.display(), skill_type);

    Ok(())
}

async fn create_claude_skill(dir: &Path, name: &str) -> Result<(), anyhow::Error> {
    // Create claude.json
    let claude_json = format!(
        r#"{{
  "name": "{}",
  "description": "A Claude skill for ...",
  "version": "0.1.0",
  "author": "Your Name",
  "tags": ["claude", "skill"]
}}"#,
        name
    );
    fs::write(dir.join("claude.json"), claude_json)?;

    // Create README.md
    let readme = format!(
        r#"# {}

A Claude skill for ...

## Description

Describe what this skill does and how it helps Claude assist users.

## Usage

Explain how to use this skill:

```
Example usage here
```

## Configuration

Any configuration options or environment variables.

## Author

Your Name
"#,
        name
    );
    fs::write(dir.join("README.md"), readme)?;

    // Create skill.md (optional, for older format support)
    let skill_md = format!(
        r#"# {} Skill

## Instructions

Your detailed instructions for Claude here.

## Examples

### Example 1

Input: ...
Output: ...
"#,
        name
    );
    fs::write(dir.join("skill.md"), skill_md)?;

    Ok(())
}

async fn create_cline_skill(dir: &Path, name: &str) -> Result<(), anyhow::Error> {
    // Create cline.json
    let cline_json = format!(
        r#"{{
  "name": "{}",
  "description": "A Cline skill for ...",
  "version": "0.1.0",
  "author": "Your Name",
  "tags": ["cline", "skill"]
}}"#,
        name
    );
    fs::write(dir.join("cline.json"), cline_json)?;

    // Create custom-instructions.md
    let instructions = format!(
        r#"# {} Custom Instructions

## Role

Describe the role this skill should take.

## Instructions

Detailed instructions for Cline.

## Context

Any context that should be provided.
"#,
        name
    );
    fs::write(dir.join("custom-instructions.md"), instructions)?;

    // Create README.md
    let readme = format!("# {}\n\nA Cline skill.\n", name);
    fs::write(dir.join("README.md"), readme)?;

    Ok(())
}

async fn create_cursor_skill(dir: &Path, name: &str) -> Result<(), anyhow::Error> {
    // Create .cursorrules
    let cursor_rules = format!(
        r#"# {} Cursor Rules

## Instructions

Your custom instructions for Cursor here.

## Code Style

- Prefer explicit over implicit
- Add comments for complex logic
- Use meaningful variable names

## Context

Project-specific context and guidelines.
"#,
        name
    );
    fs::write(dir.join(".cursorrules"), cursor_rules)?;

    // Create cursor.json (optional)
    let cursor_json = format!(
        r#"{{
  "name": "{}",
  "description": "Cursor rules for ...",
  "version": "0.1.0"
}}"#,
        name
    );
    fs::write(dir.join("cursor.json"), cursor_json)?;

    // Create README.md
    let readme = format!(
        r#"# {} Cursor Rules

Custom rules for the Cursor editor.

## Installation

Copy `.cursorrules` to your project root or Cursor rules directory.
"#,
        name
    );
    fs::write(dir.join("README.md"), readme)?;

    Ok(())
}

async fn create_roo_skill(dir: &Path, name: &str) -> Result<(), anyhow::Error> {
    // Create roo.json
    let roo_json = format!(
        r#"{{
  "name": "{}",
  "description": "A Roo Code skill for ...",
  "version": "0.1.0",
  "author": "Your Name",
  "tags": ["roo", "skill"]
}}"#,
        name
    );
    fs::write(dir.join("roo.json"), roo_json)?;

    // Create .roomodes (optional)
    let roomodes = format!(
        r#"# {} Roo Modes

mode:default {{
  instructions: "Your default instructions here"
}}
"#,
        name
    );
    fs::write(dir.join(".roomodes"), roomodes)?;

    // Create README.md
    let readme = format!(
        r#"# {}

A Roo Code skill.

## Installation

Place in `~/.roo/skills/{}/`
"#,
        name, name
    );
    fs::write(dir.join("README.md"), readme)?;

    Ok(())
}

async fn create_openclaw_skill(dir: &Path, name: &str) -> Result<(), anyhow::Error> {
    // Create skill.json
    let skill_json = format!(
        r#"{{
  "name": "{}",
  "description": "An OpenClaw skill for ...",
  "version": "0.1.0",
  "author": "Your Name"
}}"#,
        name
    );
    fs::write(dir.join("skill.json"), skill_json)?;

    // Create README.md
    let readme = format!(
        r#"# {}

An OpenClaw skill.
"#,
        name
    );
    fs::write(dir.join("README.md"), readme)?;

    Ok(())
}

async fn create_moltbot_skill(dir: &Path, name: &str) -> Result<(), anyhow::Error> {
    // Create manifest.json
    let manifest = format!(
        r#"{{
  "name": "{}",
  "description": "A Moltbot skill for ...",
  "version": "0.1.0",
  "author": "Your Name",
  "commands": [
    {{
      "name": "example",
      "description": "An example command"
    }}
  ]
}}"#,
        name
    );
    fs::write(dir.join("manifest.json"), manifest)?;

    // Create SKILL.md
    let skill_md = format!(
        r#"# {} Skill

## Description

Describe what this Moltbot skill does.

## Commands

### example

Description of the example command.

## Usage

How to use this skill.
"#,
        name
    );
    fs::write(dir.join("SKILL.md"), skill_md)?;

    // Create README.md
    let readme = format!(
        r#"# {}

A Moltbot skill.
"#,
        name
    );
    fs::write(dir.join("README.md"), readme)?;

    Ok(())
}

async fn create_codex_skill(dir: &Path, name: &str) -> Result<(), anyhow::Error> {
    // Create codex.json
    let codex_json = format!(
        r#"{{
  "name": "{}",
  "description": "An OpenAI Codex skill for ...",
  "version": "0.1.0",
  "author": "Your Name",
  "model": "gpt-4o"
}}"#,
        name
    );
    fs::write(dir.join("codex.json"), codex_json)?;

    // Create instructions.md
    let instructions = format!(
        r#"# {} Instructions

## System Prompt

Your system prompt for Codex here.

## Guidelines

- Guideline 1
- Guideline 2

## Context

Additional context for Codex.
"#,
        name
    );
    fs::write(dir.join("instructions.md"), instructions)?;

    // Create README.md
    let readme = format!(
        r#"# {}

An OpenAI Codex skill.
"#,
        name
    );
    fs::write(dir.join("README.md"), readme)?;

    Ok(())
}

async fn create_kilo_skill(dir: &Path, name: &str) -> Result<(), anyhow::Error> {
    // Create kilo.yaml
    let kilo_yaml = format!(
        r#"name: {}
description: A Kilo Code skill for ...
version: 0.1.0
author: Your Name
tags:
  - kilo
  - skill
language: typescript
"#,
        name
    );
    fs::write(dir.join("kilo.yaml"), kilo_yaml)?;

    // Create instructions.md
    let instructions = format!(
        r#"# {} Instructions

Your instructions for Kilo Code here.
"#,
        name
    );
    fs::write(dir.join("instructions.md"), instructions)?;

    // Create README.md
    let readme = format!(
        r#"# {}

A Kilo Code skill.
"#,
        name
    );
    fs::write(dir.join("README.md"), readme)?;

    Ok(())
}

async fn create_vercel_skill(dir: &Path, name: &str) -> Result<(), anyhow::Error> {
    // Create skill.json
    let skill_json = format!(
        r#"{{
  "id": "{}",
  "name": "{}",
  "description": "A Vercel AI SDK skill for ...",
  "version": "0.1.0",
  "author": "Your Name"
}}"#,
        name, name
    );
    fs::write(dir.join("skill.json"), skill_json)?;

    // Create ai.config.json
    let ai_config = r#"{
  "model": "openai/gpt-4o",
  "systemPrompt": "Your system prompt here"
}"#
    .to_string();
    fs::write(dir.join("ai.config.json"), ai_config)?;

    // Create README.md
    let readme = format!(
        r#"# {}

A Vercel AI SDK skill.
"#,
        name
    );
    fs::write(dir.join("README.md"), readme)?;

    Ok(())
}

async fn create_cloudflare_skill(dir: &Path, name: &str) -> Result<(), anyhow::Error> {
    // Create wrangler.toml
    let wrangler_toml = format!(
        r#"name = "{}"
main = "src/index.ts"
compatibility_date = "2024-01-01"

[ai]
binding = "AI"
"#,
        name
    );
    fs::write(dir.join("wrangler.toml"), wrangler_toml)?;

    // Create src directory and index.ts
    fs::create_dir(dir.join("src"))?;
    let index_ts = r#"export interface Env {
  AI: any;
}

export default {
  async fetch(request: Request, env: Env): Promise<Response> {
    // Your skill logic here
    return new Response("Hello from skill!");
  },
};
"#;
    fs::write(dir.join("src/index.ts"), index_ts)?;

    // Create README.md
    let readme = format!(
        r#"# {}

A Cloudflare Workers AI skill.

## Development

```bash
npm install
wrangler dev
```

## Deployment

```bash
wrangler deploy
```
"#,
        name
    );
    fs::write(dir.join("README.md"), readme)?;

    Ok(())
}

async fn create_generic_skill(dir: &Path, name: &str) -> Result<(), anyhow::Error> {
    // Create a generic skill structure
    let readme = format!(
        r#"# {}

A generic AI skill.

## Description

Describe what this skill does.

## Files

- `README.md` - This file
- Add your skill files here

## Usage

Explain how to use this skill.
"#,
        name
    );
    fs::write(dir.join("README.md"), readme)?;

    // Create a config file
    let config = format!(
        r#"{{
  "name": "{}",
  "description": "A generic skill",
  "version": "0.1.0"
}}"#,
        name
    );
    fs::write(dir.join("skill.json"), config)?;

    Ok(())
}
