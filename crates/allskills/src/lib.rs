use std::path::PathBuf;
use futures::stream::{self, StreamExt};

pub mod core;
pub mod providers;
pub mod types;
pub mod error;

pub use types::{Skill, SourceType, SkillScope, AllSkillsConfig};
pub use providers::{SkillProvider, KnownSources};
pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;

/// Unified skill reader that queries all configured sources
pub struct SkillReader {
    config: AllSkillsConfig,
    providers: Vec<Box<dyn crate::providers::SkillProvider>>,
}

impl SkillReader {
    pub fn new(config: AllSkillsConfig) -> Self {
        Self {
            config,
            providers: Vec::new(),
        }
    }

    pub fn add_provider<P: crate::providers::SkillProvider + 'static>(&mut self, provider: P) {
        self.providers.push(Box::new(provider));
    }

    pub async fn list_all_skills(&self) -> Result<Vec<Skill>> {
        let futures = self.providers.iter().map(|p| async {
            let config = crate::types::SourceConfig {
                name: p.name().to_string(),
                source_type: SourceType::Local,
                enabled: true,
                scope: crate::types::SkillScope::User,
                priority: 0,
            };
            p.list_skills(&config).await
        });

        let results: Vec<Result<Vec<Skill>>> = stream::iter(futures)
            .buffer_unordered(10)
            .collect()
            .await;

        let mut all_skills = Vec::new();
        for result in results {
            match result {
                Ok(skills) => all_skills.extend(skills),
                Err(e) => eprintln!("Failed to list skills: {}", e),
            }
        }

        Ok(all_skills)
    }

    pub async fn search_skills<F>(&self, predicate: F) -> Result<Vec<Skill>>
    where
        F: Fn(&Skill) -> bool + Send + Sync,
    {
        let all = self.list_all_skills().await?;
        Ok(all.into_iter().filter(predicate).collect())
    }
}
