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
- 默认分支名称是 `master`

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
- `-N` 或 `--intent-to-add`：将文件标记为“即将添加”，仅记录路径，后续仍需真正添加内容，否者commit可能会报错【我觉得大概不会考】{因为似乎只和diff有有效配合}
- `-A` 或 `--all`：添加包括新文件、已跟踪文件的修改和删除在内的所有工作区的修改
- `--ignore-removal`：忽略已删除的文件

以下略：
```
 --refresh
 --ignore-errors
 --ignore-missing
 --sparse
 --chmod (+|-)x
 --pathspec-from-file <file>
 --pathspec-file-nul
```


### 特殊情况：

注意`add -u`、`add .`与`add -A`之间的区别{以下图标来自于deepseek}
```
命令        新增文件  修改文件  删除文件  作用范围
git add -A  ✅      ✅       ✅       整个仓库
git add .   ✅      ✅       ❌       当前目录及子目录
git add -u  ❌      ✅       ✅       整个仓库（仅已跟踪文件）
```
- 可以添加多个文件。
---

## `rm` 命令

删除指定文件或目录，并从暂存区移除相关记录，同时更新索引文件

### 支持的命令选项：

- `-h` 或 `--help`：显示帮助信息
- `-n` 或 `--dry-run`：模拟执行，只会显示会删除的文件，但不会真正修改暂存区
- `-q` 或 `--quiet`：静默模式，不输出信息
- `--cached`：仅从版本控制中移除文件，保留本地文件
- `-f` 或 `--force`：强制删除文件，即使文件已被修改或暂存
- `-r`：递归删除目录


以下略：
```
--ignore-unmatch //没有匹配的文件也返回成功
--sparse 
--pathspec-from-file <file>
--pathspec-file-nul
```

### 特殊情况：

- 可以删除多个文件。
---

## `commit` 命令

将暂存区的内容提交到仓库，创建一个新的提交对象，记录提交信息，更新分支引用指向新的提交。

### 支持的命令选项：
- `-h` 或 `--help`：显示帮助信息
- `-q` 或 `--quiet`：静默模式，不输出信息

  以下是修改提交信息的：
- `--author=<author>`：覆盖提交作者，格式：`Name <email>`
- `--date=<date>`：覆盖提交作者，格式：`YYYY-MM-DD HH:MM:SS`
- `-m <msg>` 或 `--message=<msg>`：直接指定提交信息，跳过编辑器

  以下是修改提交内容的：
- `-a` 或 `--all`：提交所有已修改/删除的文件，也即自动进行`git add`
- `-o` 或 `--only`：仅提交选定的文件，而忽略其他的，文件名跟在此命令后
- `-n` 或 `--no-verify`：绕过 pre-commit 和 commit-msg 钩子 {我不确定是否要做}
- `--dry-run`：模拟执行
- `--short`：简洁显示提交状态
- `--branch`：在提交信息模板中显示分支信息
- `--ahead-behind`：计算本地分支与远程分支的领先/落后数量
- `--long`：完整显示提交状态
- `--amend`：修改上一次提交 {我不确定是否要做}
- `--no-post-rewrite`：修改上一次提交 {我不确定是否要做}
- `u[<mode>]` 或 `--untracked-files[=<mode>]`：显示未跟踪文件的状态 {我不确定是否要做}

以下略：
```
修改提交信息的：
-v, --verbose //在提交信息模板中显示差异内容（git diff）
-F <file>, --file=<file>
-c <commit>, --reedit-message=<commit>
-C <commit>, --reuse-message=<commit>
--fixup=[(amend|reword):]<commit>
--squash=<commit>
--reset-author //将作者重置为当前用户
--trailer <trailer> //添加自定义尾注
-s, --signoff
-t <file>, --template=<file>
-e, --edit
--cleanup=<mode>
--status //在提交模板中包含git status信息
-S[<key-id>], --gpg-sign[=<key-id>]
修改提交内容的：
-i, --include
--interactive
-p, --patch
--porcelain
-z, --null //文件路径以 NUL 字符分隔
--pathspec-from-file=<file> //从文件读取要提交的路径
--pathspec-file-nul
```

---

## `branch` 命令

创建新分支或删除分支

### 支持的命令选项：

- `-h` 或 `--help`：显示帮助信息
- `-v` 或 `--verbose`：显示提交哈希和主题，`-vv`显示上游分支信息
- `-q` 或 `--quiet`：静默模式，不输出信息
- `-t` 或 `--track[=(direct|inherit)]`：创建一个新本地分支并直接配置它跟踪指定的远程分支{我不确定是否要做}
- `-u` 或 `--set-upstream-to <upstream>`：将已存在的本地分支关联到指定的远程分支{我不确定是否要做}
- `--unset-upstream`：移除本地分支与远程分支的跟踪关系
- `-r` 或 `--remotes`：操作远程跟踪分支
- `--contains`：查找包含指定提交的所有本地分支，和`git log`配合

分支操作的：
- `-a` 或 `--all`：列出本地和远程所有分支
- `-d` 或 `--delete`：删除已完全合并的分支
- `-D`：强制删除分支
- `-m` 或 `--move`：移动/重命名分支
- `-M`：强制移动/重命名分支
- `-c` 或 `--copy`：复制分支
- `-C`：强制复制分支
- `-l` 或 `--list`：列出分支名
- `--show-current`：显示当前分支名
- `-f` 或 `--force`：强制创建/移动/删除
- `--merged`：仅显示已合并到某提交的分支


以下略：
```
--color[=<when>] //彩色输出
--abbrev[=<n>] //设置显示哈希的缩写位数，默认为7
--omit-empty
--create-reflog
--edit-description //编辑分支描述
 --column //以列格式显示分支
 --sort <key> //按字段排序
 --points-at <object>
 -i, --ignore-case //排序和过滤时忽略大小写
--recurse-submodules //递归处理子模块
--format <format>
```
### 特殊情况：

- 注意`-r`需要配合其它命令选项
- 显示分支时当前分支会以`*`标出
- `-d`(或`-D`)可以删除多个分支。

---

## `checkout` 命令

用于切换分支、恢复工作区文件或查看历史版本

### 支持的命令选项：

- `-h` 或 `--help`：显示帮助信息
- `-b <branch>`：创建并切换到新分支
- `-B <branch>`：创建或强制重置并切换到新分支
- `-q` 或 `--quiet`：静默模式，不输出信息
- `-t` 或 `--track[=(direct|inherit)]`：配置本地分支指定的远程分支{我不确定是否要做}
- `-f` 或 `--force`：强制丢弃本地修改并切换分支


以下略：
```
-l //为新分支创建引用日志
--guess //是否自动推测远程分支
--overlay //控制是否覆盖未跟踪文件
--recurse-submodules //递归处理子模块
--progress //强制显示
-m, --merge //切换分支时尝试三路合并
--conflict=<style>
-d, --[no-]detach
--orphan <new-branch> //创建无父提交的新分支
--overwrite-ignore //是否覆盖被忽略的文件
--ignore-other-worktrees 
-2, --ours /-3, --theirs //检出冲突文件中的“我方”或“对方”版本
-p, --patch //交互式界面
--ignore-skip-worktree-bits
--pathspec-from-file=<file> //从文件读取路径
--pathspec-file-nul
```

### 特殊情况：
- `-b`只会用来创造分支，而`-B`还可能用于把分支重置
- 似乎可以恢复多个分支？
---

## `merge` 命令

合并分支，将指定分支的更改合并到当前分支，处理冲突

### 支持的命令选项：

- `-h` 或 `--help`：显示帮助信息
- `--stat` 或 `--summary`：合并结束后显示差异统计
- `-n`：`--stat`的相反操作
- `--squash`：创建单个提交而不是执行合并 {考虑到我们助教的要求，似乎不会考}
- `--commit`：如果合并成功则执行提交 {考虑到我们助教的要求，似乎不会考}
- `--ff`：允许快进合并
- `--ff-only`：如果不能快进则中止合并
- `-m`或`--message <message>`：为非快进合并指定提交消息
- `-q` 或 `--quiet`：静默模式，不输出信息
- `--abort`：中止当前进行中的合并 {考虑到我们助教的要求，似乎不会考}
- `--quit`：类似`--abort`但保留索引和工作树不变 {考虑到我们助教的要求，似乎不会考}
- `--continue`：继续当前进行中的合并 {考虑到我们助教的要求，似乎不会考}
- `--no-verify`：绕过 pre-merge-commit 和 commit-msg 钩子{我不确定是否要做}

以下略：
```
--log[=<n>] //在合并提交信息中添加最多<n>条短日志条目
-e, --edit //在提交前编辑消息
--cleanup <mode> //指定如何从消息中去除空格和#注释
--rerere-autoupdate //如果可能，使用重用的冲突解决方案更新索引
--verify-signatures
-s, --strategy <strategy> //指定要使用的合并策略
-X, --strategy-option <option=value> //为选定合并策略指定选项
-F, --file <path> //从文件读取提交消息
--into-name <name>
-v, --verbose //显示更详细的信息
--allow-unrelated-histories //允许合并不相关的历史记录
--progress
-S, --gpg-sign[=<key-id>]
--autostash
--overwrite-ignore //更新被忽略的文件
--signoff
```

### 特殊情况：

- 可以合并多个文件。

---

## `fetch` 命令

从远程仓库下载最新数据但不自动合并到本地工作区

### 支持的命令选项：

- `-h` 或 `--help`：显示帮助信息
- `-q` 或 `--quiet`：静默模式，不输出信息
- `--all`：从所有已配置的远程仓库下载数据
- `--set-upstream`：为当前分支设置上游分支
- `-f` 或 `--force`：强制覆盖本地引用
- `-m` 或 `--multiple`：允许从多个远程仓库获取
- `-t` 或 `--tags`：下载所有标签及其关联对象{我不确定是否要做}
- `-n`：禁用标签下载{我不确定是否要做}
- `-p` 或 `--prune`：删除本地已不存在的远程跟踪分支
- `--dry-run`：模拟执行
- `--write-fetch-head`：将获取的引用写入`.git/FETCH_HEAD`文件
- `--shallow-since <time>`：仅获取指定时间之后的提交


以下略：
```
-v, --verbose //显示更详细的信息
-a, --append //将新获取的引用追加到 .git/FETCH_HEAD 文件
--atomic
--upload-pack <path>
-j, --jobs <n> //并行获取子模块的数量(默认1)
--prefetch
--recurse-submodules[=<on-demand>] //递归获取子模块更新
-P, --prune-tags //删除本地已不存在的远程标签
--porcelain //生成机器可读的输出格式
-k, --keep //保留下载的包文件，因为默认会清理临时文件
-u, --update-head-ok
--progress
--depth <depth> //限制克隆历史深度，即浅克隆
--shallow-exclude <revision> //排除特定提交及其历史
--deepen <n> //深化浅克隆的历史深度
--unshallow //将浅克隆转换为完整仓库
--refetch //重新获取数据
--refmap <refmap>
-o, --server-option <option>
-4, --ipv4; -6, --ipv6 //强制使用 IPv4 或 IPv6 网络协议
--negotiation-tip <revision>
--negotiate-only 
--filter <args> //启用对象过滤
--auto-maintenance; --auto-gc
--show-forced-updates
--write-commit-graph
--stdin
```

### 特殊情况：

- 注意这里的`-n`不代表模拟执行了。
- 支持同时拉取多个分支
---

## `pull` 命令
要求改变，不写了

---

## `push` 命令
要求改变，不写了