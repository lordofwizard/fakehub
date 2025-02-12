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

### **Options Explained**

- `-d, --dir <DIRECTORY>`  
  Specifies the directory where the fake Git repository will be created.
  
- `-s, --start <START_DATE>`  
  Defines the start date for commit generation. Must be in the format `DD-MM-YYYY`.
  
- `-e, --end <END_DATE>`  
  Specifies the end date for commit generation, also in `DD-MM-YYYY` format.
  
- `-b, --back <DAYS_BACK>`  
  Instead of setting explicit start and end dates, this option generates commits for a specified number of days back from the current date.
  
- `-r, --range-start <COMMIT_RANGE_START>`  
  Sets the minimum number of commits to generate per day (default: `5`).
  
- `-x, --range-end <COMMIT_RANGE_END>`  
  Sets the maximum number of commits per day (default: `10`).
  
- `-q, --fixed-number <FIXED_NUMBER>`  
  Generates a fixed number of commits per day, overriding `--range-start` and `--range-end` (default: `5`).
  
- `-h, --help`  
  Displays help information.
  
- `-V, --version`  
  Prints the current version of FakeHub.
  
### **Example Usages**

1. **Generate commits from a fixed date range:**
   ```sh
   fakehub --start 01-01-2023 --end 31-12-2023
   ```

2. **Generate commits for the last 90 days with a variable range of 3-8 commits per day:**
   ```sh
   fakehub --back 90 --range-start 3 --range-end 8
   ```

3. **Generate exactly 10 commits per day for the past 30 days:**
   ```sh
   fakehub --back 30 --fixed-number 10
   ```

---

## 📜 License
This project is released under the **MIT License**. See the [LICENSE](LICENSE) file for details.

---

## ⭐ Contribute & Support
- If you like this project, **star the repo!** ⭐
- Open an [issue](https://github.com/lordofwizard/fakehub/issues) if you find bugs or have suggestions.

Enjoy your fake Git history! 😉

