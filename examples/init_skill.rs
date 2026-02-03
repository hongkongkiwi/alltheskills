/// Example: Creating a new skill using the init command pattern
///
/// This example shows how to programmatically create a new skill
/// with boilerplate files for different skill types.

use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let skill_name = "my-awesome-skill";
    let skill_type = "claude";
    let output_dir = PathBuf::from("./example-output");

    println!("Creating new {} skill: {}", skill_type, skill_name);

    // Create skill directory
    let skill_dir = output_dir.join(skill_name);
    fs::create_dir_all(&skill_dir)?;

    // Create claude.json
    let claude_json = format!(
        r#"{{
  "name": "{}",
  "description": "A custom Claude skill for ...",
  "version": "0.1.0",
  "author": "Your Name",
  "tags": ["claude", "custom"],
  "dependencies": [
    {{
      "name": "base-utils",
      "version": "^1.0.0",
      "source": "https://github.com/example/base-utils"
    }}
  ]
}}"#,
        skill_name
    );
    fs::write(skill_dir.join("claude.json"), claude_json)?;

    // Create README.md
    let readme = format!(
        r#"# {}

A custom Claude skill.

## Description

Describe what this skill does.

## Installation

```bash
alltheskills install {}
```

## Usage

Explain how to use this skill.

## Dependencies

- base-utils (^1.0.0)
"#,
        skill_name, skill_dir.display()
    );
    fs::write(skill_dir.join("README.md"), readme)?;

    // Create skill.md with instructions
    let skill_md = format!(
        r#"# {} Instructions

## System Prompt

You are a helpful assistant specialized in ...

## Capabilities

- Capability 1
- Capability 2

## Guidelines

1. Always do X before Y
2. Be concise and clear
3. Ask for clarification when needed
"#,
        skill_name
    );
    fs::write(skill_dir.join("skill.md"), skill_md)?;

    println!("✅ Created skill at: {}", skill_dir.display());
    println!("\nFiles created:");
    println!("  - claude.json");
    println!("  - README.md");
    println!("  - skill.md");

    // Cleanup
    fs::remove_dir_all(&output_dir)?;
    println!("\nCleaned up example output.");

    Ok(())
}
