# Package Management

This directory contains package management configurations for various package managers and distribution platforms.

## Structure

```
pkg/
├── README.md           # This file
└── homebrew/          # Homebrew tap configuration
    └── Formula/
        └── quickmark-cli.rb
```

## Available Package Managers

### Homebrew (macOS)

Location: `pkg/homebrew/`
Documentation: See [HOMEBREW.md](../HOMEBREW.md)

**Installation:**
```bash
brew tap ekropotin/quickmark
brew install quickmark-cli
```

## Future Package Managers

This structure is designed to support additional package managers:

- **APT/DEB** (`pkg/debian/`) - Debian/Ubuntu packages
- **RPM** (`pkg/rpm/`) - RedHat/Fedora packages  
- **AUR** (`pkg/aur/`) - Arch User Repository
- **Chocolatey** (`pkg/chocolatey/`) - Windows package manager
- **Scoop** (`pkg/scoop/`) - Windows package manager
- **npm** (`pkg/npm/`) - Node.js package manager (if creating wrapper)
- **Snap** (`pkg/snap/`) - Universal Linux packages
- **Flatpak** (`pkg/flatpak/`) - Universal Linux packages

## Contributing

When adding support for a new package manager:

1. Create a new subdirectory under `pkg/`
2. Add the package configuration files
3. Update this README with installation instructions
4. Add documentation to the main project README