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
