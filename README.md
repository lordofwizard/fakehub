# FakeHub
![Preview](https://github.com/user-attachments/assets/843a70ee-3110-4d78-8b41-e4d74b67d176)
![Latest Release](https://img.shields.io/github/v/release/lordofwizard/fakehub?style=flat)
![Build](https://img.shields.io/github/actions/workflow/status/lordofwizard/fakehub/build_test.yml?branch=main&style=flat)
![License](https://img.shields.io/github/license/lordofwizard/fakehub?style=flat)
![GitHub issues](https://img.shields.io/github/issues/lordofwizard/fakehub?style=flat)
![GitHub stars](https://img.shields.io/github/stars/lordofwizard/fakehub?style=flat)

---

## 🚀 Introduction
FakeHub is a **fake Git/GitHub history generator** powered by [**libgit2**](https://libgit2.org/) and written entirely in **Rust**🦀. It creates a repository (`fakehub_repo` by default) and fills it with a **completely artificial commit history**.

⚠ **This project is made as a joke!** Use it at your own risk. We take no responsibility for whatever nonsense you do with it. 😆

---

## 📦 Installation

### Install via Cargo
```sh
cargo install --path .
```

### Install from Releases
Prebuilt binaries for Windows and Linux are available on the [Releases Page](https://github.com/lordofwizard/fakehub/releases).

#### **Windows**
1. Download the latest `.zip` file from the [Releases Page](https://github.com/lordofwizard/fakehub/releases).
2. Extract the contents.
3. Run `fakehub.exe` from the extracted folder.

#### **Linux**
1. Download the latest `.tar.gz` file from the [Releases Page](https://github.com/lordofwizard/fakehub/releases).
2. Extract the archive:
   ```sh
   tar -xvzf fakehub-x86_64-unknown-linux-gnu.tar.gz
   ```
3. Navigate to the extracted folder and run:
   ```sh
   ./fakehub
   ```

---

## 🛠 Usage

FakeHub provides multiple options to customize commit generation.

### **Basic Usage**
```sh
fakehub [OPTIONS]
```

### **Options**
```
  -d, --dir <DIRECTORY>          Directory path for git repository
  -s, --start <START_DATE>       Start date for commit generation (format: DD-MM-YYYY)
  -e, --end <END_DATE>           End date for commit generation (format: DD-MM-YYYY)
  -b, --back <DAYS_BACK>         Generate commits for specified number of days back from current date
  -r, --range-start <COMMIT_RANGE_START>  Minimum number of range of commits to generate per day (default 5)
  -x, --range-end <COMMIT_RANGE_END>      Maximum number of range of commits to generate per day (default 10)
  -q, --fixed-number <FIXED_NUMBER>       Fixed number of commits (default 5)
  -h, --help                      Print help
  -V, --version                   Print version
```

---

## 📜 License
This project is released under the **MIT License**. See the [LICENSE](LICENSE) file for details.

---

## ⭐ Contribute & Support
- If you like this project, **star the repo!** ⭐
- Open an issue if you find bugs or have suggestions.

Enjoy your fake Git history! 😉

