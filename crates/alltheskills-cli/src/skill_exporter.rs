/// Generates a Claude skill README for the alltheskills manager
pub fn generate_alltheskills_skill_readme() -> String {
    r#"# AllSkills Manager

A skill for managing AI skills from Claude, Cline, OpenClaw, Roo Code, and more.

## Available Commands

### List all skills
```bash
alltheskills list
```

### Search for skills
```bash
alltheskills search <query>
```

### Install a new skill
```bash
alltheskills install --from <source>
```

### Get skill information
```bash
alltheskills info <skill-name>
```

## Configuration

You can configure alltheskills via `~/.config/alltheskills/alltheskills.toml`:

```toml
default_scope = "user"
install_dir = ".alltheskills"
cache_dir = ".alltheskills/cache"
```

## Environment Variables

- `ALLSKILLS_CONFIG_DIR` - Override config directory
- `ALLSKILLS_SKILLS_DIR` - Override skills directory
"#
    .to_string()
}

/// Generates a claude.json for the alltheskills skill
pub fn generate_claude_json() -> String {
    r#"{
  "name": "alltheskills-manager",
  "description": "Manage AI skills from various sources including Claude, Cline, OpenClaw, Roo Code, and GitHub",
  "version": "0.1.0",
  "author": "AllSkills",
  "commands": [
    {
      "name": "list-skills",
      "description": "List all installed skills from all configured sources"
    },
    {
      "name": "search-skills",
      "description": "Search for skills by name or description"
    },
    {
      "name": "install-skill",
      "description": "Install a new skill from GitHub or local path"
    },
    {
      "name": "info-skill",
      "description": "Get detailed information about a specific skill"
    },
    {
      "name": "add-source",
      "description": "Add a new skill source directory"
    }
  ]
}"#.to_string()
}
