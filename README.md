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
1. Download the `.deb` package.
2. Open your terminal in the download folder.
3. Install using `apt`:
   ```bash
   sudo apt ./test_examer_1.0.0_amd64.deb
4. Or install using `dpkg`:
   ```bash
   sudo dpkg -i test_examer_1.0.0_amd64.deb
   #If there are missing dependencies, run:
   sudo apt-get install -f
5.You can now launch Test Examer from your application menu.
  
### 🪟 For Windows

1.Download the Test_Examer_Windows.zip file.

2.Extract the ZIP folder to your desired location.

3.Ensure the assets/ folder is in the same directory as Test_Examer.exe.

4.Double-click Test_Examer.exe to run.

    Note: Since the binary is not digitally signed, Windows SmartScreen may show a warning. Click "More info" -> "Run anyway".
  
