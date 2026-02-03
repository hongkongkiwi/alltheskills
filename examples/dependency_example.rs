/// Example: Working with skill dependencies
///
/// This example demonstrates how to use the DependencyResolver
/// to check and resolve skill dependencies.

use alltheskills::dependencies::DependencyResolver;
use alltheskills::types::{Skill, SkillDependency, SkillMetadata, SourceType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a sample skill with dependencies
    let skill = Skill {
        id: "my-skill".to_string(),
        name: "My Skill".to_string(),
        description: "A skill with dependencies".to_string(),
        version: Some("1.0.0".to_string()),
        source: alltheskills::types::SkillSource::Local {
            path: std::path::PathBuf::from("./my-skill"),
        },
        source_type: SourceType::Claude,
        path: std::path::PathBuf::from("./my-skill"),
        installed_at: chrono::Utc::now(),
        metadata: SkillMetadata {
            author: Some("Example Author".to_string()),
            tags: vec!["example".to_string()],
            dependencies: vec![
                SkillDependency {
                    name: "base-utils".to_string(),
                    version_req: Some("^1.0.0".to_string()),
                    source: Some("https://github.com/example/base-utils".to_string()),
                    optional: false,
                },
                SkillDependency {
                    name: "optional-helper".to_string(),
                    version_req: None,
                    source: None,
                    optional: true, // This one is optional
                },
            ],
            ..Default::default()
        },
        format: alltheskills::types::SkillFormat::ClaudeSkill,
    };

    println!("Skill: {} (v{})", skill.name, skill.version.as_ref().unwrap());
    println!("Dependencies:");
    for dep in &skill.metadata.dependencies {
        let version = dep.version_req.as_deref().unwrap_or("any");
        let optional = if dep.optional { " (optional)" } else { "" };
        println!("  - {} @ {}{}", dep.name, version, optional);
    }

    // Create resolver with some pre-installed skills
    let installed_skill = Skill {
        id: "base-utils".to_string(),
        name: "base-utils".to_string(),
        description: "Base utilities".to_string(),
        version: Some("1.2.0".to_string()),
        source: alltheskills::types::SkillSource::Local {
            path: std::path::PathBuf::from("./base-utils"),
        },
        source_type: SourceType::Local,
        path: std::path::PathBuf::from("./base-utils"),
        installed_at: chrono::Utc::now(),
        metadata: SkillMetadata::default(),
        format: alltheskills::types::SkillFormat::GenericJson,
    };

    let mut resolver = DependencyResolver::with_installed(vec![installed_skill]);

    // Check which dependencies are satisfied
    println!("\nDependency Status:");
    for dep in &skill.metadata.dependencies {
        let status = if resolver.is_satisfied(dep) {
            "✅ Satisfied"
        } else if dep.optional {
            "⚪ Optional (not installed)"
        } else {
            "❌ Missing"
        };
        println!("  {}: {}", dep.name, status);
    }

    // Resolve dependencies (get list of what needs to be installed)
    let to_install = resolver.resolve_dependencies(&skill)?;
    println!("\nDependencies to install:");
    if to_install.is_empty() {
        println!("  (none - all satisfied)");
    } else {
        for dep in to_install {
            let source = dep.source.as_deref().unwrap_or("unknown source");
            println!("  - {} from {}", dep.name, source);
        }
    }

    Ok(())
}
