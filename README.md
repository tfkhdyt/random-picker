# Random Picker

A simple, fast CLI tool written in Go for randomly selecting items from configured groups.

## Features

- Randomly pick items from configured groups
- Track chosen and unchosen items
- List items by different states:
  - All items in a group
  - Previously chosen items
  - Remaining unchosen items
- Reset chosen items history
- Multiple item groups support
- Configuration via YAML
- Shell completion support (Bash, Zsh, Fish, PowerShell)
- Cross-platform (Linux, macOS, Windows)

## Installation

### From Releases

Download the [latest release](https://github.com/tfkhdyt/random-picker-go/releases/latest) for your platform.

### Using Go Install

```bash
go install github.com/tfkhdyt/random-picker-go/cmd/random-picker@latest
```

### From Source

```bash
# Clone the repository
git clone https://github.com/tfkhdyt/random-picker-go.git
cd random-picker-go

# Build the binary
go build -o rp ./cmd/random-picker
```

## Quick Start

```bash
# Generate a sample config
rp gen-config

# Pick a random item from the default group
rp

# Pick from a specific group
rp -g games

# List items that have been chosen already
rp list

# List all items in a group
rp list all

# List items that haven't been chosen yet
rp list unchosen

# Reset history of chosen items
rp reset
```

## Configuration

The application uses a YAML configuration file located at:

- Linux/macOS: `~/.config/random-picker/config.yaml`
- Windows: `%APPDATA%\random-picker\config.yaml`

Sample configuration:

```yaml
default_group: games
groups:
  - name: games
    items:
      - "Balatro"
      - "Marvel Rivals"
      - "Stardew Valley"
      - "Minecraft"
  - name: movies
    items:
      - "The Shawshank Redemption"
      - "The Godfather"
      - "Inception"
      - "Pulp Fiction"
```

## Commands

| Command                 | Description                      |
| ----------------------- | -------------------------------- |
| `rp`                    | Pick a random item               |
| `rp list [type]`        | List items (all/chosen/unchosen) |
| `rp reset`              | Reset chosen items history       |
| `rp gen-config`         | Generate sample config           |
| `rp completion [shell]` | Generate shell completion        |

Global flags:

- `-g, --group`: Specify group name

## Development

### Requirements

- Go 1.22 or higher

### Build

```bash
go build -o rp ./cmd/random-picker
```

## Releasing

This project uses GitHub Actions to automatically build and publish releases.

To create a new release:

1. Create and push a new tag following semantic versioning:

   ```bash
   git tag -a v0.1.0 -m "First release"
   git push origin v0.1.0
   ```

2. The GitHub Action will automatically:
   - Build the application for multiple platforms (Linux, macOS, Windows)
   - Create a GitHub Release with the binaries attached
   - Generate a changelog based on the commit history

## License

[MIT](LICENSE)
