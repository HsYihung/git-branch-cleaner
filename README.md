# Git Branch Cleaner

一個用 Rust 編寫的命令行工具，用於清理 Git 倉庫中的分支。

## 功能特點

- 📂 目錄導航
  - 瀏覽並選擇要清理的 Git 倉庫
  - 支持返回上層目錄
  - 支持返回根目錄
  - 顯示目錄是否為 Git 倉庫

- 🔍 分支狀態顯示
  - 顯示當前分支
  - 顯示受保護分支（main、master、dev、develop）
  - 顯示已合併分支
  - 顯示未合併分支
  - 顯示超過 30 天未更新的分支
  - 顯示每個分支的最後更新時間

- 🗑️ 分支清理選項
  - 刪除已合併分支
  - 刪除超過 30 天未更新的分支
  - 刪除所有分支（除受保護分支外）
  - 選擇性刪除單個分支

## 安裝

確保你的系統已安裝 Rust 和 Cargo。然後執行：

```bash
cargo install --path .
```

## 使用方法

1. 在終端機中運行程序：
   ```bash
   cargo run
   ```

2. 使用方向鍵和 Enter 鍵進行導航和選擇：
   - 選擇要清理的 Git 倉庫目錄
   - 選擇要執行的清理操作
   - 確認要刪除的分支

## Building from Source

### Prerequisites

- Docker Desktop

### Building with Docker

使用 Docker 进行跨平台构建，支持 Windows 和 macOS：

```bash
# 构建 Docker 镜像
docker build -t git-branch-cleaner-builder .

# 运行构建容器
docker run --rm -v ${PWD}:/usr/src/git-branch-cleaner git-branch-cleaner-builder "0.1.0"
```

构建完成后，可执行文件将在 `release` 目录中：
- Windows: `git-branch-cleaner-v0.1.0-windows-x64.exe`
- macOS x64: `git-branch-cleaner-v0.1.0-macos-x64`
- macOS ARM: `git-branch-cleaner-v0.1.0-macos-arm64`

## 注意事項

- 受保護分支（main、master、dev、develop）不會被刪除
- 建議在刪除分支前先確認分支狀態
- 未合併的分支需要使用強制刪除選項
