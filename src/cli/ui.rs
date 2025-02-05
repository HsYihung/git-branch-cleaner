use dialoguer::{Select, theme::ColorfulTheme, console::Term, MultiSelect};
use colored::*;
use std::path::PathBuf;
use crate::git::GitBranch;
use chrono::Utc;
use super::{DeleteOption, DirectoryChoice};

pub struct Cli {
    theme: ColorfulTheme,
}

impl Cli {
    pub fn new() -> Self {
        Self {
            theme: ColorfulTheme::default(),
        }
    }

    pub fn display_branches(&self, branches: &[GitBranch], current_branch: &str, protected_branches: &[String]) {
        println!("\n{}", "分支列表:".cyan());
        println!("  {}", "當前分支:".cyan());
        println!("  {}", current_branch.green());
        println!("\n{}", "受保護分支:".cyan());
        println!("  {}", protected_branches.join(", ").yellow());

        let merged_branches: Vec<_> = branches.iter()
            .filter(|b| b.is_merged)
            .collect();

        let stale_branches: Vec<_> = branches.iter()
            .filter(|b| !b.is_merged && (Utc::now() - b.last_commit_date).num_days() > 30)
            .collect();

        if !merged_branches.is_empty() {
            println!("\n{}", "已合併的分支:".yellow());
            for branch in merged_branches {
                let days = (Utc::now() - branch.last_commit_date).num_days();
                let last_update = branch.last_commit_date.format("%Y-%m-%d %H:%M:%S").to_string();
                println!("  {} (最後更新: {}, {} 天前)", 
                    branch.name.green(),
                    last_update,
                    days
                );
            }
        }

        if !stale_branches.is_empty() {
            println!("\n{}", "超過 30 天未更新的分支:".yellow());
            for branch in stale_branches {
                let days = (Utc::now() - branch.last_commit_date).num_days();
                let last_update = branch.last_commit_date.format("%Y-%m-%d %H:%M:%S").to_string();
                println!("  {} (最後更新: {}, {} 天未更新)", 
                    branch.name.yellow(), 
                    last_update,
                    days
                );
            }
        }
    }

    pub fn select_branches_to_delete(&self, branches: &[GitBranch]) -> Option<Vec<usize>> {
        let options: Vec<String> = branches
            .iter()
            .map(|b| {
                let days = (Utc::now() - b.last_commit_date).num_days();
                let last_update = b.last_commit_date.format("%Y-%m-%d %H:%M:%S").to_string();
                format!("{} (最後更新: {}, {} 天前{})",
                    b.name,
                    last_update,
                    days,
                    if b.is_merged { ", 已合併" } else { "" }
                )
            })
            .collect();

        if options.is_empty() {
            println!("{}", "沒有可刪除的分支.".green());
            return None;
        }

        println!("\n{}", "選擇要刪除的分支:".cyan());
        MultiSelect::with_theme(&self.theme)
            .items(&options)
            .interact_on(&Term::stderr())
            .ok()
    }

    pub fn select_delete_option(&self) -> Option<DeleteOption> {
        let options = vec![
            "刪除已合併的分支",
            "刪除超過 30 天未更新的分支",
            "刪除所有分支",
            "刪除單一分支",
            "退出"
        ];

        let selection = Select::with_theme(&self.theme)
            .with_prompt("選擇刪除選項")
            .items(&options)
            .default(0)
            .interact_on_opt(&Term::stderr())
            .unwrap();

        match selection {
            Some(0) => Some(DeleteOption::MergedBranches),
            Some(1) => Some(DeleteOption::StaleBranches),
            Some(2) => Some(DeleteOption::AllBranches),
            Some(3) => Some(DeleteOption::SingleBranch),
            _ => None
        }
    }

    pub fn display_repo_type(&self, path: &std::path::Path, is_git: bool) {
        let path_str = path.display().to_string();
        if is_git {
            println!("{} {}", path_str, "(git)".green());
        } else {
            println!("{}", path_str);
        }
    }

    pub fn display_deleted_branches(&self, branches: &[String]) {
        if branches.is_empty() {
            println!("{}", "沒有分支被刪除".yellow());
            return;
        }
        println!("\n已刪除的分支:");
        for branch in branches {
            println!("  {}", branch.red());
        }
    }

    pub fn select_directory(&self, current_dir: &PathBuf, subdirs: &[PathBuf]) -> DirectoryChoice {
        let mut options = vec![
            format!("📂 {}", "..".blue()),  // 返回上層
            format!("🏠 {}", "回到原目錄".blue()),  // 回到原目錄
            format!("🚪 {}", "退出".red()),  // 退出
        ];

        // 添加子目錄
        for dir in subdirs {
            let dir_name = dir.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");
            options.push(format!("📁 {}", dir_name));
        }

        println!("\n當前目錄: {}", current_dir.display().to_string().green());
        
        let selection = Select::with_theme(&self.theme)
            .with_prompt("選擇目錄")
            .items(&options)
            .default(0)
            .interact_on_opt(&Term::stderr())
            .unwrap();

        match selection {
            Some(0) => DirectoryChoice::ParentDirectory,
            Some(1) => DirectoryChoice::RootDirectory,
            Some(2) => DirectoryChoice::Exit,
            Some(i) => DirectoryChoice::SelectDirectory(i - 3),
            None => DirectoryChoice::Exit,
        }
    }
}
