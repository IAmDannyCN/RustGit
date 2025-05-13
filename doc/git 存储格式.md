### index
- 使用 base64 序列化
- 前四个字符为 `DIRC`
- 之后每行存储两项（用 \0 分隔）代表一个文件
	- 路径：从项目根目录起的相对路径。例：`folder/a.txt`
	- hash：文件内容的 hash
### blob
- 使用 base64 序列化
- 前四个字符为 `BLOB`
- hash 计算只使用 `blob.data`，不涉及 `BLOB`
- 之后为文件原始内容
### tree
- 使用 base64 序列化
- 前四个字符为 `TREE`
- hash 计算只使用 `tree.data`，不涉及 `TREE`
- 之后每行存储三项（用 \0 分隔）代表一个 Entry
	- 种类：为字面量 `BLOB` 或 `TREE`
	- 名称：文件名/文件夹名
	- hash：内容的 hash
### commit
- 使用 base64 序列化
- 前四个字符为 `CMIT`
- hash 计算只使用 `commit.data`，不涉及 `CMIT`
- 之后四行代表一个 Commit
	- Commit Message
	- Commit User
	- Commit Time
	- 根目录 hash
### ref
- 直接存储 SHA1
### HEAD
- 常规模式：`ref: refs/heads/<branch_name>`
- DETACH HEAD 模式：对应 commit 的 SHA1