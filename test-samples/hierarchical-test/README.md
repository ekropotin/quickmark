# Hierarchical Config Discovery Test Suite

This directory contains integration tests for the hierarchical config discovery feature implemented for issue-43.

## Directory Structure

```
hierarchical-test/
├── project-root/                    # Main project with relaxed config
│   ├── quickmark.toml              # MD001=off, MD013=warn(100 chars)
│   ├── README.md                   # Uses project-root config
│   ├── src/                        
│   │   ├── quickmark.toml          # MD001=warn, MD013=err(80 chars)
│   │   ├── api.md                  # Uses src/ config
│   │   └── docs/
│   │       └── guide.md            # Inherits src/ config
│   └── tests/
│       └── integration.md          # Inherits project-root config
└── cargo-project/                  # Demonstrates Cargo.toml boundary
    ├── Cargo.toml                  # Project root marker
    ├── quickmark.toml              # setext_with_atx style
    └── src/
        └── lib.md                  # Uses cargo-project config
```

## Test Scenarios

1. **Hierarchical Inheritance**: Files inherit the closest config in their ancestor directories
2. **Different Configurations**: Each directory level can have different rule severities and settings
3. **Project Boundaries**: Discovery stops at common project markers like `Cargo.toml`
4. **Git Boundaries**: Discovery stops at `.git` directories to respect repository boundaries

## Expected Behavior

- `project-root/README.md`: MD001 disabled, 100-char line limit warnings
- `project-root/src/api.md`: MD001 warnings, 80-char line limit errors  
- `project-root/src/docs/guide.md`: Inherits src/ config (MD001 warnings, 80-char errors)
- `project-root/tests/integration.md`: Inherits project-root config (MD001 disabled, 100-char warnings)
- `cargo-project/src/lib.md`: Uses setext_with_atx style from cargo-project/quickmark.toml

This demonstrates the full hierarchical config discovery working as specified in the LSP Phase 2 requirements.