![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)

# Test_Examer 🚀

A high-performance, cross-platform Desktop Examination System built with **Rust** and the **eframe (egui)** framework. This project is designed to provide a lightweight and secure environment for taking tests and exams.

## ✨ Features

- **Cross-Platform:** Native support for both **Linux** (.deb) and **Windows** (.exe).
- **Blazing Fast:** Built with Rust for memory safety and top-tier performance.
- **Modern UI:** Clean and intuitive user interface powered by `egui`.
- **Cybersecurity Focused:** Designed with secure data handling and integrity in mind.
- **Custom Fonts:** Integrated stylish typography for better readability.

## 🛠️ Tech Stack

- **Language:** [Rust](https://www.rust-lang.org/)
- **GUI Framework:** [eframe / egui](https://github.com/emilk/egui)
- **Compilation:** Multi-target cross-compilation (x86_64-unknown-linux-gnu & x86_64-pc-windows-gnu)
- **Resource Management:** `winres` for Windows metadata and icons.


## 🚀 Future Plans & Roadmap

I am actively working on enhancing **Test_Examer** with the following features:

- [ ] **Advanced PDF Templates:** Adding support for diverse PDF structures and layouts beyond the current single template.
- [ ] **Custom Exam Settings:** Users will be able to select a specific number of questions and set custom time limits.
- [ ] **Timer System:** Integration of a countdown timer to simulate real-world exam conditions.
- [ ] **Security Hardening:** Implementing secure PDF parsing to prevent potential malicious script execution (Sandboxing).
- [ ] **Dark Mode Support:** Improving the UI for better user experience during long study sessions.



## 🚀 Getting Started

### Prerequisites

- Rust toolchain (Cargo)
- (Optional) Mingw-w64 for Windows cross-compilation on Linux.

### Installation & Build

1. **Clone the repository:**
   ```bash
   git clone [https://github.com/cavidibra48/Test_Examer.git](https://github.com/cavidibra48/Test_Examer.git)
   cd Test_Examer
2. You can download the pre-compiled binaries from the **[Releases](https://github.com/cavidibra48/Test_Examer/releases)** section.

### 🐧 For Linux (Debian/Ubuntu/Mint)
1. Download the **[.deb](https://github.com/cavidibra48/Test_Examer/releases/download/v1.0.0/test-examer_1.0.0-1_amd64.deb)** package.
2. Open your terminal in the download folder.
3. Install using `apt`:
   ```bash
   sudo apt ./test_examer_1.0.0_amd64.deb
4. Or install using `dpkg`:
   ```bash
   sudo dpkg -i test_examer_1.0.0_amd64.deb
   #If there are missing dependencies, run:
   sudo apt-get install -f
5. You can now launch Test Examer from your application menu.
  
### 🪟 For Windows

1.Download the Test_Examer_Windows.zip file.

2.Extract the ZIP folder to your desired location.

3.Ensure the assets/ folder is in the same directory as Test_Examer.exe.

4.Double-click Test_Examer.exe to run.

    Note: Since the binary is not digitally signed, Windows SmartScreen may show a warning. Click "More info" -> "Run anyway".






## ⚖️ License

This project is licensed under the **GNU GPLv3** License. 

- You are free to run, study, share, and modify the software.
- If you modify the core logic and redistribute it, you **must** keep it under the same license (Copyleft).
- See the [LICENSE](LICENSE) file for the full text.
  
