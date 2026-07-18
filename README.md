# [wiiuse c-lib](https://wiiuse.net/?nav=api) bindings for rust programming language

Building works out of the box, pre installed wiiuse lib is not needed.

Except for initaliasing the submodule with:

```bash
git submodule update --init --recursive
```

## System Requirements

But Depdending on the System u need to install the C development headers for Bluetooth. 
On Linux the Linux Bluetooth stack (BlueZ) is required. 

For Windows Systems, the native Bluetooth API is used, 
so no additional third-party Bluetooth libraries are required.

Install the appropriate package for your Linux distribution:

*   **Debian / Ubuntu / Linux Mint:**
    ```bash
    sudo apt install libbluetooth-dev
    ```

*   **Arch Linux / Manjaro:**
    ```bash
    sudo pacman -S bluez-libs
    ```

*   **Fedora:**
    ```bash
    sudo dnf install bluez-libs-devel
    ```

*   **openSUSE:**
    ```bash
    sudo zypper install bluez-devel
    ```

## Build & Run

Once the system requirements are met, you can build and run the project using Cargo as usual:

```bash
cargo build
```
