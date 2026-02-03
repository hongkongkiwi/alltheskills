# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

#### Core Library
- **Dependency Management**: New `dependencies` module with `DependencyResolver` for parsing and resolving skill dependencies
- **`SkillDependency` type**: Represents a dependency on another skill with version requirements and source information
- **`copy_dir_recursive` utility**: Helper function for copying directories recursively
- **`is_skill_dir` utility**: Helper to check if a path contains a valid skill

#### CLI Commands
- **`init` command**: Create new skill templates with boilerplate files for all supported skill types
  - Supports: claude, cline, cursor, roo, openclaw, moltbot, codex, kilo, vercel, cloudflare
  - Generates appropriate configuration files for each type
- **`show` command**: Display skill content with formatted or raw output
  - Shows skill metadata in a formatted box
  - Displays the skill's README/instructions content
  - `--raw` flag for piping content

#### Provider Install Methods
All 11 providers now have working `install()` methods:
- ClaudeProvider
- ClineProvider
- CursorProvider
- RooProvider
- OpenClawProvider
- MoltbotProvider
- VercelProvider
- CloudflareProvider
- CodexProvider
- KiloProvider
- GitHubProvider (was already working)
- LocalProvider (was already working)

#### Update Command
- **`update` command now works**: Implemented git pull logic for GitHub-based skills
  - Fetches updates from origin
  - Performs fast-forward merge
  - Handles branch switching
  - Shows status (updated / already up to date / error)

#### Documentation
- Comprehensive module-level documentation for all modules
- Added `dependencies` module documentation with examples
- Added `utils` module documentation
- Updated `STATUS.md` to reflect completed work

#### Examples
- `init_skill.rs`: Example of creating a new skill programmatically
- `dependency_example.rs`: Example of working with skill dependencies

### Changed

- Updated `SkillMetadata` to include `dependencies: Vec<SkillDependency>`
- Claude provider now parses dependencies from `claude.json`
- Improved provider trait documentation with comprehensive examples
- Fixed duplicate crate-level documentation in `lib.rs`

### Fixed

- Fixed bare URL warning in cursor provider documentation
- Removed unused imports in CLI commands
- Fixed all clippy warnings

## [0.1.0] - 2024-XX-XX

### Added

- Initial release
- Core library with SkillReader and provider trait system
- 11 skill providers for various AI assistants
- CLI with basic commands (list, search, info, install, remove, validate, config)
- Configuration management with TOML
- GitHub integration for cloning repositories
- CI/CD workflows for testing and releases

---

## Categories

- **Added**: New features
- **Changed**: Changes to existing functionality
- **Deprecated**: Soon-to-be removed features
- **Removed**: Removed features
- **Fixed**: Bug fixes
- **Security**: Security improvements
