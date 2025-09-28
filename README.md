# MultiversX Template Smart Contract

A template smart contract for MultiversX blockchain development, providing a foundation for building decentralized applications.

## Features

- **Admin Management**: Built-in admin functionality with role-based access control
- **Pause Mechanism**: Contract can be paused/unpaused by administrators
- **Modular Design**: Clean, modular architecture for easy extension

## Structure

```
src/
├── lib.rs          # Main contract trait
├── admins.rs       # Admin management module
└── pause.rs        # Pause/unpause functionality

common/
├── constants/      # Shared constants
├── errors/         # Error definitions
└── structs/        # Data structures

interaction/        # Deployment and interaction scripts
meta/              # Contract metadata
output/            # Generated ABI and WASM files
wasm/              # WebAssembly build configuration
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.