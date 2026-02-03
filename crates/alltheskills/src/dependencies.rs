//! Skill dependency management
//!
//! This module provides functionality for parsing, resolving, and installing
//! skill dependencies.
//!
//! # Example
//!
//! ```rust
//! use alltheskills::dependencies::DependencyResolver;
//!
//! # fn main() {
//! let resolver = DependencyResolver::new();
//! // Use resolver to check dependencies...
//! # }
//! ```

use crate::types::{Skill, SkillDependency};
use crate::Result;
use std::collections::{HashMap, HashSet};

/// Resolves and manages skill dependencies
pub struct DependencyResolver {
    /// Already installed skills (name -> skill)
    installed: HashMap<String, Skill>,
    /// Skills being resolved (to detect circular dependencies)
    resolving: HashSet<String>,
}

impl DependencyResolver {
    /// Creates a new dependency resolver
    pub fn new() -> Self {
        Self {
            installed: HashMap::new(),
            resolving: HashSet::new(),
        }
    }

    /// Creates a resolver with pre-populated installed skills
    pub fn with_installed(installed: Vec<Skill>) -> Self {
        let mut resolver = Self::new();
        for skill in installed {
            resolver.installed.insert(skill.name.clone(), skill);
        }
        resolver
    }

    /// Resolves all dependencies for a skill
    ///
    /// Returns a list of dependencies that need to be installed,
    /// in the order they should be installed (dependencies first).
    pub fn resolve_dependencies(&mut self, skill: &Skill) -> Result<Vec<SkillDependency>> {
        self.resolving.clear();
        let mut result = Vec::new();
        self.resolve_recursive(skill, &mut result)?;
        Ok(result)
    }

    fn resolve_recursive(
        &mut self,
        skill: &Skill,
        result: &mut Vec<SkillDependency>,
    ) -> Result<()> {
        // Check for circular dependencies
        if self.resolving.contains(&skill.name) {
            return Err(crate::Error::Config {
                message: format!("Circular dependency detected: {}", skill.name),
            });
        }

        // Skip if already processed
        if self.installed.contains_key(&skill.name) {
            return Ok(());
        }

        // Mark as being resolved
        self.resolving.insert(skill.name.clone());

        // Process dependencies
        for dep in &skill.metadata.dependencies {
            // Skip optional dependencies for now
            if dep.optional {
                continue;
            }

            // Check if already installed
            if self.installed.contains_key(&dep.name) {
                // TODO: Check version requirement
                continue;
            }

            // Check if already in result list
            if result.iter().any(|d| d.name == dep.name) {
                continue;
            }

            result.push(dep.clone());
        }

        // Remove from resolving set
        self.resolving.remove(&skill.name);

        Ok(())
    }

    /// Checks if a dependency is satisfied
    pub fn is_satisfied(&self, dep: &SkillDependency) -> bool {
        if let Some(installed) = self.installed.get(&dep.name) {
            // Check version requirement if specified
            if let Some(req) = &dep.version_req {
                if let Some(version) = &installed.version {
                    // Simple version check (could use semver crate for proper parsing)
                    return version_satisfies_req(version, req);
                }
            }
            true
        } else {
            false
        }
    }

    /// Adds an installed skill to the resolver
    pub fn add_installed(&mut self, skill: Skill) {
        self.installed.insert(skill.name.clone(), skill);
    }

    /// Gets all installed skills
    pub fn installed(&self) -> &HashMap<String, Skill> {
        &self.installed
    }
}

impl Default for DependencyResolver {
    fn default() -> Self {
        Self::new()
    }
}

/// Checks if a version satisfies a version requirement
///
/// This is a simplified implementation. For production use,
/// consider using the `semver` crate.
fn version_satisfies_req(version: &str, req: &str) -> bool {
    // Simple exact match
    if version == req {
        return true;
    }

    // Handle caret (^) requirements - compatible with version
    if let Some(req_ver) = req.strip_prefix('^') {
        return is_compatible(version, req_ver);
    }

    // Handle greater-than-equal (>=) requirements
    if let Some(req_ver) = req.strip_prefix(">=") {
        return compare_versions(version, req_ver) >= 0;
    }

    // Handle greater-than (>) requirements
    if let Some(req_ver) = req.strip_prefix('>') {
        return compare_versions(version, req_ver) > 0;
    }

    // Default to exact match
    version == req
}

/// Checks if a version is compatible with a requirement (caret semantics)
fn is_compatible(version: &str, req: &str) -> bool {
    // Parse major version from requirement
    let req_major = req.split('.').next().unwrap_or("0");
    let ver_major = version.split('.').next().unwrap_or("0");

    // Major version must match for caret compatibility
    if req_major != ver_major {
        return false;
    }

    // Version must be >= requirement
    compare_versions(version, req) >= 0
}

/// Compares two version strings
/// Returns:
/// - negative if v1 < v2
/// - zero if v1 == v2
/// - positive if v1 > v2
fn compare_versions(v1: &str, v2: &str) -> i32 {
    let parts1: Vec<u32> = v1
        .split('.')
        .filter_map(|p| p.parse().ok())
        .collect();
    let parts2: Vec<u32> = v2
        .split('.')
        .filter_map(|p| p.parse().ok())
        .collect();

    let max_len = parts1.len().max(parts2.len());

    for i in 0..max_len {
        let p1 = parts1.get(i).copied().unwrap_or(0);
        let p2 = parts2.get(i).copied().unwrap_or(0);

        if p1 != p2 {
            return (p1 as i32) - (p2 as i32);
        }
    }

    0
}

/// Parses dependencies from a skill configuration file
///
/// This function can parse dependencies from various skill formats
pub fn parse_dependencies(value: &serde_json::Value) -> Vec<SkillDependency> {
    let mut deps = Vec::new();

    if let Some(deps_array) = value.get("dependencies").and_then(|d| d.as_array()) {
        for dep_value in deps_array {
            if let Some(name) = dep_value.get("name").and_then(|n| n.as_str()) {
                let dep = SkillDependency {
                    name: name.to_string(),
                    version_req: dep_value
                        .get("version")
                        .or_else(|| dep_value.get("version_req"))
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    source: dep_value
                        .get("source")
                        .and_then(|s| s.as_str())
                        .map(|s| s.to_string()),
                    optional: dep_value
                        .get("optional")
                        .and_then(|o| o.as_bool())
                        .unwrap_or(false),
                };
                deps.push(dep);
            } else if let Some(name) = dep_value.as_str() {
                // Simple string format: just the name
                deps.push(SkillDependency {
                    name: name.to_string(),
                    version_req: None,
                    source: None,
                    optional: false,
                });
            }
        }
    }

    // Also check for simple array of strings format
    if let Some(deps_array) = value.get("dependencies").and_then(|d| d.as_array()) {
        for dep_value in deps_array {
            if let Some(name) = dep_value.as_str() {
                // Check if already added as complex object
                if !deps.iter().any(|d: &SkillDependency| d.name == name) {
                    deps.push(SkillDependency {
                        name: name.to_string(),
                        version_req: None,
                        source: None,
                        optional: false,
                    });
                }
            }
        }
    }

    deps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_versions() {
        assert_eq!(compare_versions("1.0.0", "1.0.0"), 0);
        assert_eq!(compare_versions("2.0.0", "1.0.0"), 1);
        assert_eq!(compare_versions("1.0.0", "2.0.0"), -1);
        assert_eq!(compare_versions("1.1.0", "1.0.0"), 1);
        assert_eq!(compare_versions("1.0.1", "1.0.0"), 1);
    }

    #[test]
    fn test_version_satisfies_req() {
        assert!(version_satisfies_req("1.0.0", "1.0.0"));
        assert!(version_satisfies_req("1.2.0", "^1.0.0"));
        assert!(version_satisfies_req("1.0.0", ">=1.0.0"));
        assert!(!version_satisfies_req("2.0.0", "^1.0.0"));
        assert!(!version_satisfies_req("0.9.0", ">=1.0.0"));
    }

    #[test]
    fn test_parse_dependencies() {
        let json = serde_json::json!({
            "dependencies": [
                "skill-a",
                {
                    "name": "skill-b",
                    "version": "^1.0.0",
                    "source": "https://github.com/user/skill-b"
                },
                {
                    "name": "skill-c",
                    "optional": true
                }
            ]
        });

        let deps = parse_dependencies(&json);
        assert_eq!(deps.len(), 3);

        assert_eq!(deps[0].name, "skill-a");
        assert!(deps[0].version_req.is_none());

        assert_eq!(deps[1].name, "skill-b");
        assert_eq!(deps[1].version_req, Some("^1.0.0".to_string()));
        assert_eq!(deps[1].source, Some("https://github.com/user/skill-b".to_string()));

        assert_eq!(deps[2].name, "skill-c");
        assert!(deps[2].optional);
    }
}
