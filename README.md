# LisPort

LisPort 是一个基于 Rust 的轻量级 **本地端口监听检测工具**，旨在为系统管理员、安全研究人员及开发者提供一种直观、高效的方式来查看本地主机上正在监听（LISTEN）的 TCP 端口信息。

它不仅提供了交互式的 TUI 文本界面，还支持生成结构化报告（JSON、Markdown、纯文本），方便用于自动化分析和日志记录。

---

## 🚀 主要功能

- **本地监听端口检测**：扫描并列出本地主机上所有处于 `LISTEN` 状态的 TCP 端口
- **服务识别**：根据熟知端口号自动识别服务（如 HTTP、SSH、FTP 等）
- **进程信息获取**：显示监听端口的进程信息（PID、进程名称、完整路径等）
- **用户识别**：支持 Unix-like 系统，显示绑定端口的用户信息
- **权限提示**：提示端口是否需要管理员权限（如 < 1024 的知名端口）
- **多格式输出**：支持输出为 `text`、`json`、`md` 格式，便于自动化处理与展示
- **终端 TUI 界面**：提供交互式界面，方便快速浏览和查找端口信息

---

## 📋 使用方式

```bash
lisport [OPTIONS]
```

### 选项说明：

| 选项                    | 说明                                       |
| ----------------------- | ------------------------------------------ |
| `-f, --format <FORMAT>` | 指定输出格式，可选值：`text`, `json`, `md` |
| `-o, --output <OUTPUT>` | 输出路径，默认为标准输出（`/dev/stdout`）  |
| `-h, --help`            | 显示帮助信息                               |
| `-V, --version`         | 显示版本信息                               |

---

## 🧪 示例用法

- 以默认格式查看端口信息（使用 TUI 界面）：

```bash
lisport
```

- 输出为 JSON 格式：

```bash
lisport --format json
```

- 输出为 Markdown 并保存到文件：

```bash
lisport --format md --output report.md
```

---

## 🛠️ 开发与构建

### 构建方式

```bash
# 构建调试版本
cargo build

# 构建发布版本（优化）
cargo build --release
```

### 安装为系统命令

```bash
cargo install --path .
```

安装后即可使用：

```bash
lisport [OPTIONS]
```

---

## ⚠️ 权限说明

- 端口号 < 1024的端口需要管理员权限（`root` / `sudo`）才能访问。
- 在非管理员权限下运行可能会导致部分信息缺失。

---

## 📚 注意事项

- 本工具**仅支持本地监听端口的扫描**，不适用于远程主机端口扫描。
- TUI 界面在某些终端中可能显示异常，推荐使用现代支持 ANSI 的终端。
- 输出格式为 `json` 或 `md` 时，可以用于自动化分析或生成报告。

---

## 🤝 贡献指南

我们欢迎社区贡献！你可以提交 bug 修复、功能增强或文档改进：

1. Fork 本仓库
2. 创建新分支 (`git checkout -b feature/your-feature`)
3. 提交更改 (`git commit -m 'Add some feature'`)
4. 推送分支 (`git push origin feature/your-feature`)
5. 提交 Pull Request

---

## 📄 许可证

本项目采用 [MIT License](LICENSE)，欢迎自由使用与修改。

---

## ❤️ 致谢

感谢 Rust 社区提供的强大工具链和开源精神。
如果你喜欢这个项目，请在 GitHub 上点 ⭐ 支持！

---

## 📞 如何获取帮助？

- 查看帮助信息：`lisport --help`
- 提交 Issue：[GitHub Issues](https://github.com/your-repo-url/issues)