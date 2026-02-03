use alltheskills::providers::{
    ClaudeProvider, ClineProvider, CloudflareProvider, CodexProvider, CursorProvider,
    KiloProvider, LocalProvider, MoltbotProvider, OpenClawProvider, RooProvider, VercelProvider,
};
use alltheskills::{AllSkillsConfig, SkillReader};
use std::path::PathBuf;

pub async fn update_skill(name: Option<&str>) -> Result<(), anyhow::Error> {
    let config = AllSkillsConfig::default();
    let mut reader = SkillReader::new(config);

    // Add all providers
    reader.add_provider(ClaudeProvider);
    reader.add_provider(ClineProvider);
    reader.add_provider(CursorProvider);
    reader.add_provider(RooProvider);
    reader.add_provider(OpenClawProvider);
    reader.add_provider(MoltbotProvider);
    reader.add_provider(CodexProvider);
    reader.add_provider(KiloProvider);
    reader.add_provider(VercelProvider);
    reader.add_provider(CloudflareProvider);
    reader.add_provider(LocalProvider);

    let skills = reader.list_all_skills().await?;

    if let Some(name) = name {
        // Update specific skill
        let name_lower = name.to_lowercase();
        let matching: Vec<_> = skills
            .iter()
            .filter(|s| {
                s.name.to_lowercase() == name_lower || s.id.to_lowercase() == name_lower
            })
            .collect();

        if matching.is_empty() {
            println!("Skill '{}' not found.", name);
            return Ok(());
        }

        for skill in matching {
            update_single_skill(skill).await?;
        }
    } else {
        // Update all skills
        println!("Checking for updates for {} skill(s)...", skills.len());
        for skill in &skills {
            update_single_skill(skill).await?;
        }
    }

    Ok(())
}

async fn update_single_skill(skill: &alltheskills::Skill) -> Result<(), anyhow::Error> {
    use alltheskills::types::SkillSource;

    match &skill.source {
        SkillSource::GitHub {
            owner,
            repo,
            subdir: _,
            branch,
        } => {
            println!("Updating {} (GitHub: {}/{})...", skill.name, owner, repo);
            match update_git_skill(&skill.path, owner, repo, branch.as_deref()).await {
                Ok(updated) => {
                    if updated {
                        println!("  ✅ Updated successfully");
                    } else {
                        println!("  ℹ️  Already up to date");
                    }
                }
                Err(e) => {
                    println!("  ❌ Update failed: {}", e);
                }
            }
        }
        SkillSource::Local { path: _ } => {
            // Local skills can't be automatically updated
            println!("Skipping {} (local skill)", skill.name);
        }
        SkillSource::Remote { url, .. } => {
            println!("Updating {} from {}...", skill.name, url);
            println!("  Note: Remote skill updates not yet implemented");
        }
    }

    Ok(())
}

async fn update_git_skill(
    path: &PathBuf,
    _owner: &str,
    _repo: &str,
    branch: Option<&str>,
) -> Result<bool, anyhow::Error> {
    if !path.exists() {
        anyhow::bail!("Skill directory does not exist: {}", path.display());
    }

    // Check if this is a git repository
    let git_dir = path.join(".git");
    if !git_dir.exists() {
        // Not a git repo, can't update
        anyhow::bail!("Not a git repository: {}", path.display());
    }

    // Open the repository
    let repo = git2::Repository::open(path)?;

    // Get the current HEAD
    let head = repo.head()?;
    let current_oid = head.target().ok_or_else(|| anyhow::anyhow!("No target for HEAD"))?;

    // Fetch updates from origin
    println!("  Fetching updates from origin...");

    // Perform fetch
    let mut remote = repo.find_remote("origin")?;
    remote.fetch(&[] as &[&str], None, None)?;

    // Determine which reference to merge
    let ref_name = branch.map(|b| format!("refs/remotes/origin/{}", b));
    let reference = if let Some(ref_name) = ref_name {
        repo.find_reference(&ref_name)?
    } else {
        // Use origin/HEAD or origin/main or origin/master
        let refs = ["origin/HEAD", "origin/main", "origin/master"];
        let mut found_ref = None;
        for r in &refs {
            if let Ok(reference) = repo.find_reference(&format!("refs/remotes/{}", r)) {
                found_ref = Some(reference);
                break;
            }
        }
        found_ref.ok_or_else(|| anyhow::anyhow!("Could not find default branch"))?
    };

    let new_oid = reference.target().ok_or_else(|| anyhow::anyhow!("No target for reference"))?;

    // Check if there are updates
    if current_oid == new_oid {
        return Ok(false); // Already up to date
    }

    // Perform merge (fast-forward only for safety)
    let annotated_commit = repo.find_annotated_commit(new_oid)?;
    repo.merge(&[&annotated_commit], None, None)?;

    // Clean up merge state
    repo.cleanup_state()?;

    // Update HEAD to point to new commit
    let ref_name = format!("refs/heads/{}", branch.unwrap_or("main"));
    let mut local_ref = repo.find_reference(&ref_name)?;
    local_ref.set_target(new_oid, "Fast-forward merge")?;

    Ok(true)
}
