mod git;
mod cli;

use std::process;
use colored::*;
use std::path::PathBuf;

use crate::git::{GitFinder, BranchManager};
use crate::cli::{Cli, DeleteOption, DirectoryChoice};

fn process_git_repo(cli: &Cli, repo_path: &PathBuf, protected_branches: &[String]) {
    let manager = match BranchManager::new(repo_path.as_path(), protected_branches) {
        Ok(manager) => manager,
        Err(e) => {
            eprintln!("{}: {}", "Error".red(), e);
            return;
        }
    };

    let current_branch = manager.get_current_branch()
        .unwrap_or_else(|| "unknown".to_string());
    
    let branches = manager.list_branches();
    cli.display_branches(&branches, &current_branch, &protected_branches);
    
    match cli.select_delete_option() {
        Some(DeleteOption::MergedBranches) => {
            if let Ok(deleted) = manager.delete_merged_branches() {
                cli.display_deleted_branches(&deleted);
            }
        },
        Some(DeleteOption::StaleBranches) => {
            if let Ok(deleted) = manager.delete_stale_branches(30) {
                cli.display_deleted_branches(&deleted);
            }
        },
        Some(DeleteOption::AllBranches) => {
            if let Ok(deleted) = manager.delete_all_branches() {
                cli.display_deleted_branches(&deleted);
            }
        },
        Some(DeleteOption::SingleBranch) => {
            if let Some(selected_indices) = cli.select_branches_to_delete(&branches) {
                let mut deleted = Vec::new();
                for &index in &selected_indices {
                    let branch = &branches[index];
                    let force = !branch.is_merged;
                    print!("刪除分支 {}: ", branch.name);
                    match manager.delete_branch(&branch.name, force) {
                        Ok(_) => {
                            println!("{}", "成功".green());
                            deleted.push(branch.name.clone());
                        },
                        Err(e) => println!("{} - {}", "失敗".red(), e),
                    }
                }
                cli.display_deleted_branches(&deleted);
            }
        },
        None => {}
    }
}

fn main() {
    let protected_branches = vec![
        "main".to_string(),
        "master".to_string(),
        "dev".to_string(),
        "develop".to_string(),
    ];

    let cli = Cli::new();
    
    let mut finder = match GitFinder::new() {
        Ok(finder) => finder,
        Err(e) => {
            eprintln!("{}: {}", "Error".red(), e);
            process::exit(1);
        }
    };
    
    loop {
        // 檢查當前目錄是否為 Git 倉庫
        if finder.is_git_repo(finder.get_current_dir()) {
            process_git_repo(&cli, finder.get_current_dir(), &protected_branches);
            println!("\n{}", "分支清理完成！".green());
            return;
        }

        // 獲取子目錄列表
        let subdirs = finder.get_subdirectories();
        
        // 顯示當前目錄下的所有目錄
        for dir in &subdirs {
            cli.display_repo_type(dir, finder.is_git_repo(dir));
        }
        
        // 選擇目錄
        match cli.select_directory(finder.get_current_dir(), &subdirs) {
            DirectoryChoice::SelectDirectory(index) => {
                if index < subdirs.len() {
                    finder.navigate_to(subdirs[index].clone());
                }
            },
            DirectoryChoice::ParentDirectory => {
                if !finder.navigate_to_parent() {
                    println!("{}", "已經在根目錄了".yellow());
                }
            },
            DirectoryChoice::RootDirectory => {
                finder.navigate_to_root();
            },
            DirectoryChoice::Exit => {
                println!("{}", "程式已退出".blue());
                return;
            }
        }
    }
}
