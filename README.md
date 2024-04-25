# Byte Healer

Byte Healer is a command-line utility for Windows designed to fix various system problems using Windows's built-in tools. It automates the process of running diagnostic and repair commands to resolve common issues.

## Features

- **CMD Loader Installation**: Ensures the presence of a crucial component, the CMD Loader, which contains JSON files with lists of commands necessary for system fixes.

- **System AutoFix**: Detects and attempts to fix issues related to process loading and system stability.

- **Process Selection**: Provides a menu-based interface for users to choose specific processes or system optimizations to run.

## Installation

### Option 1: Install from Releases (Recommended)

1. Go to the [Releases](https://github.com/ammardevz/byte_healer/releases) page.
2. Download the latest version of the executable (`byte_healer.zip`).
3. Extract the contents of `byte_healer.zip` to a location on your computer.
4. Navigate to the extracted folder.
5. Run the `byte_healer.exe` file and follow the on-screen instructions for installation.

### Option 2: Compile source code

1. Clone the repository:
   ```bash
   git clone https://github.com/ammardevz/byte_healer.git
   ```
2. Navigate to the project directory:
   ```bash
   cd byte_healer
   ```
3. Compile the program:
   ```bash
   cargo build --release
   ```
4. Run the program:
   ```bash
   cargo run
   ```

## Usage

Upon running Byte Healer, follow the on-screen instructions to navigate through various options and select the processes or repairs you wish to execute. Make sure to run the program with elevated privileges (as an administrator) for full functionality.

### CMD Loader

The CMD Loader is a crucial component of Byte Healer that facilitates the execution of essential commands for system repair and optimization. It utilizes JSON files (`cmd.json`) located in the project directory to define lists of commands.

#### Adding Commands to CMD Loader

You can edit the `cmd.json` file to add or modify commands as needed for your system fixes. Each command entry should follow this structure:

```json
[
  {
    "long_name": "System File Checker",
    "short_name": "SFC",
    "cmd": "sfc /scannow"
  },
  {
    "long_name": "Deployment Image Servicing and Management",
    "short_name": "DISM",
    "cmd": "dism /online /cleanup-image /restorehealth"
  },
  {
    "long_name": "Check Disk",
    "short_name": "CHKDSK",
    "cmd": "chkdsk C: /f"
  },
  {
    "long_name": "DISM Cleanup",
    "short_name": "DISM",
    "cmd": "dism /online /cleanup-image /startcomponentcleanup"
  },
  {
    "long_name": "Network Troubleshooting",
    "short_name": "Network Troubleshooting",
    "cmd": "netsh winsock reset"
  }
]
```

By adding commands to the CMD Loader, you can customize Byte Healer's capabilities to address specific system issues efficiently.

## Disclaimer

Byte Healer utilizes Windows built-in tools and commands for system optimization and repair. Users are advised to proceed with caution, as any actions performed with Byte Healer are at their own discretion and risk.

## Contributing

Contributions to Byte Healer are welcome! Feel free to fork the repository, make improvements, and submit pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
