# LisPort

**LisPort** is a lightweight, Rust-based tool for detecting **locally listening TCP ports**. It offers an intuitive and efficient way to inspect TCP ports currently in the `LISTEN` state on the local host.

---

## 🚀 Key Features

- **Detect Listening Ports**: Lists all TCP ports in the `LISTEN` state on the local machine.
- **Service Recognition**: Identifies common services by port number (e.g., HTTP, SSH, FTP).
- **Process Details**: Displays associated process information (PID, name, and full path).
- **User Identification**: Shows the user associated with each bound port (Unix-like systems only).
- **Privilege Indication**: Highlights ports requiring administrative access (ports < 1024).
- **Multiple Output Formats**: Supports `text`, `json`, and `md` outputs for flexible usage.
- **Interactive TUI**: Enables real-time browsing and filtering of port data directly in the terminal.

---

## 📋 Usage

```bash
lisport [OPTIONS]
```

### Available Options

| Option                  | Description                                |
| ----------------------- | ------------------------------------------ |
| `-f, --format <FORMAT>` | Set output format (`text`, `json`, `md`)   |
| `-o, --output <OUTPUT>` | Specify output file path (default: stdout) |
| `-h, --help`            | Show help message                          |
| `-V, --version`         | Show version information                   |

---

## 🧪 Example Commands

- **Launch interactive TUI:**

```bash
lisport
```

- **Output as JSON:**

```bash
lisport --format json
```

- **Generate Markdown report:**

```bash
lisport --format md --output report.md
```

---

## 🛠️ Development & Build

### Build Instructions

```bash
# Build debug version
cargo build

# Build optimized release version
cargo build --release
```

### Install as a System Command

```bash
cargo install --path .
```

Once installed, run:

```bash
lisport [OPTIONS]
```

---

## ⚠️ Privilege Requirements

- **Ports below 1024** require elevated privileges (e.g., `root` or `sudo`) to access.
- Running without sufficient permissions may result in incomplete or missing data.

---

## 📚 Notes

- **Local-only scanning**: This tool is designed for inspecting local listening ports only.
- **TUI compatibility**: Some terminals may not render the TUI correctly. Use a modern ANSI-compatible terminal for best results.
- **Automation-ready**: JSON and Markdown outputs are ideal for scripting and documentation workflows.

---

## 🤝 Contributing

We welcome contributions from the community! Whether it's bug fixes, new features, or improvements to documentation:

1. Fork the repository
2. Create your feature branch: `git checkout -b feature/your-feature`
3. Commit your changes: `git commit -m 'Add some feature'`
4. Push to the branch: `git push origin feature/your-feature`
5. Open a Pull Request

---

## 📄 License

This project is licensed under the [MIT License](LICENSE). Feel free to use, modify, and distribute it as needed.

---

## ❤️ Acknowledgments

A big thank you to the Rust community for providing an incredible ecosystem and fostering the spirit of open source.

If you find this project useful, please give it a ⭐ on GitHub!

---

## 📞 Need Help?

- View help: `lisport --help`
- Report issues or suggest features: [GitHub Issues](https://github.com/moyanj/lisport/issues)