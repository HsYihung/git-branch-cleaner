use git2::{BranchType, Repository};
use chrono::{DateTime, TimeZone, Utc};
use std::path::Path;
use std::process::Command;

#[derive(Debug)]
pub struct GitBranch {
    pub name: String,
    pub last_commit_date: DateTime<Utc>,
    pub is_merged: bool,
}

pub struct BranchManager {
    repo: Repository,
    protected_branches: Vec<String>,
}

impl BranchManager {
    pub fn new(path: &Path, protected_branches: &[String]) -> Result<Self, git2::Error> {
        Ok(Self {
            repo: Repository::open(path)?,
            protected_branches: protected_branches.to_vec(),
        })
    }

    pub fn get_current_branch(&self) -> Option<String> {
        self.repo
            .head()
            .ok()?
            .shorthand()
            .map(String::from)
    }

    fn fetch_from_remote(&self) -> Result<(), git2::Error> {
        if let Ok(mut remote) = self.repo.find_remote("origin") {
            let mut fetch_options = git2::FetchOptions::new();
            fetch_options.download_tags(git2::AutotagOption::None);
            fetch_options.update_fetchhead(true);

            remote.fetch(&[] as &[&str], Some(&mut fetch_options), None)?;
        }
        Ok(())
    }

    pub fn list_branches(&self) -> Vec<GitBranch> {
        if let Err(e) = self.fetch_from_remote() {
            eprintln!("Warning: Failed to fetch from remote: {}", e);
        }

        let current = self.get_current_branch();
        let mut branches = Vec::new();

        if let Ok(branch_iter) = self.repo.branches(Some(BranchType::Local)) {
            for branch_result in branch_iter {
                if let Ok((branch, _)) = branch_result {
                    if let Ok(Some(name)) = branch.name() {
                        // Skip protected branches only, show current branch
                        if self.protected_branches.contains(&name.to_string()) {
                            continue;
                        }

                        if let Ok(commit) = branch.get().peel_to_commit() {
                            let timestamp = commit.time().seconds();
                            if let Some(date) = Utc.timestamp_opt(timestamp, 0).earliest() {
                                let is_merged = self.is_branch_merged(&name);
                                branches.push(GitBranch {
                                    name: name.to_string(),
                                    last_commit_date: date,
                                    is_merged,
                                });
                            }
                        }
                    }
                }
            }
        }

        branches
    }

    fn is_branch_merged(&self, branch_name: &str) -> bool {
        // 使用 git 命令来检查分支是否已合并
        let output = Command::new("git")
            .args(&["merge-base", "--is-ancestor", branch_name, "HEAD"])
            .current_dir(self.repo.path().parent().unwrap_or(self.repo.path()))
            .status();

        match output {
            Ok(status) => status.success(),
            Err(_) => false,
        }
    }

    fn is_protected(&self, branch_name: &str) -> bool {
        self.protected_branches.contains(&branch_name.to_string())
    }

    pub fn delete_branch(&self, branch_name: &str, _force: bool) -> Result<(), git2::Error> {
        let mut branch = self.repo.find_branch(branch_name, BranchType::Local)?;
        branch.delete()
    }

    pub fn delete_merged_branches(&self) -> Result<Vec<String>, git2::Error> {
        let mut deleted = Vec::new();
        for branch in self.list_branches() {
            if branch.is_merged && !self.is_protected(&branch.name) {
                if let Ok(_) = self.delete_branch(&branch.name, false) {
                    deleted.push(branch.name);
                }
            }
        }
        Ok(deleted)
    }

    pub fn delete_stale_branches(&self, days: i64) -> Result<Vec<String>, git2::Error> {
        let mut deleted = Vec::new();
        for branch in self.list_branches() {
            if !self.is_protected(&branch.name) {
                let days_old = (Utc::now() - branch.last_commit_date).num_days();
                if days_old > days {
                    if let Ok(_) = self.delete_branch(&branch.name, true) {
                        deleted.push(branch.name);
                    }
                }
            }
        }
        Ok(deleted)
    }

    pub fn delete_all_branches(&self) -> Result<Vec<String>, git2::Error> {
        let mut deleted = Vec::new();
        for branch in self.list_branches() {
            if !self.is_protected(&branch.name) {
                if let Ok(_) = self.delete_branch(&branch.name, true) {
                    deleted.push(branch.name);
                }
            }
        }
        Ok(deleted)
    }
}
