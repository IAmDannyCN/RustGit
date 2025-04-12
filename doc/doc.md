# Git 项目文档

## `init` 命令

初始化一个新的 Git 仓库，创建一个名为 `.git` 的子目录，内含以下内容：
```
├── HEAD
├── branches
├── config
├── description
├── hooks
│   ├── applypatch-msg.sample
│   ├── commit-msg.sample
│   ├── fsmonitor-watchman.sample
│   ├── post-update.sample
│   ├── pre-applypatch.sample
│   ├── pre-commit.sample
│   ├── pre-merge-commit.sample
│   ├── pre-push.sample
│   ├── pre-rebase.sample
│   ├── pre-receive.sample
│   ├── prepare-commit-msg.sample
│   └── update.sample
├── info
│   └── exclude
├── objects
│   ├── info
│   └── pack
└── refs
    ├── heads
    └── tags
```
### 支持的命令选项：
- `-h` 或 `--help`：显示帮助信息
- `-q` 或 `--quiet`：静默模式，不输出信息
- `-b <分支名>` 或 `--initial-branch=<分支名>`：指定初始分支名称
- `--separate-git-dir=<git目录>`：将 `.git` 目录放在指定位置（而不是项目根目录）
- `--template=<模板目录>`：使用指定的模板目录初始化仓库（模板中的文件会被复制到 `.git` 目录）【我觉得大概不会考】{行为模式有点奇怪}
- `--bare`：创建一个裸仓库（没有工作区）【我觉得大概不会考】{行为模式有点奇怪}
- `--object-format <hash算法>`：规定使用的 hash 算法【我觉得大概不会考】{助教大概不会使用复数hash算法}

### 特殊情况：
- 如果目录中已存在 `.git` 文件，重新生成 `.git` 目录。若未启用 `-q`，则返回：  
  `Reinitialized existing Git repository in <目录名>`
- 默认分支名称是 `main` 而不是 `master`

---

## `add` 命令

将指定文件或目录的内容添加到暂存区

### 支持的命令选项：
- `-h` 或 `--help`：显示帮助信息
- `-n` 或 `--dry-run`：模拟执行，只会显示会添加的文件，但不会真正修改暂存区
- `-v` 或 `--verbose`：显示会添加的文件，即显示详细信息
- `-i` 或 `--interactive`：进入交互模式，逐个选择要暂存的文件【我觉得大概不会考】{交互式界面不太好评测}
- `-p` 或 `--patch`：进入交互模式，逐块选择要暂存的文件变更【我觉得大概不会考】{交互式界面不太好评测}
- `-e` 或 `--edit`：进入编辑模式，编辑要暂存的文件变更【我觉得大概不会考】{编辑界面不太好评测}
（以上三条命令精细化程度依次提高）
- `-f` 或 `--force`：允许添加被.gitignore忽略的文件
- `-u` 或 `--update`：仅添加更新修改或删除的文件，即仅添加已跟踪的文件，不添加新文件
- `--renormalize`：重新规范化行尾和内容（根据 .gitattributes 规则）【我觉得大概不会考】{助教不会用多系统……吧}

---

## `rm` 命令
（暂无详细说明）

---

## `commit` 命令
- 使用 `-a` 选项时，Git 会自动将所有已跟踪的文件暂存并提交，跳过 `git add` 步骤。

---

## `branch` 命令
（暂无详细说明）

---

## `checkout` 命令
（暂无详细说明）

---

## `merge` 命令
- 可以合并多个文件。

---

## `fetch` 命令
（暂无详细说明）

---

## `pull` 命令
（暂无详细说明）

---

## `push` 命令
（暂无详细说明）