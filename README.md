# glimpse

A simple command-line tool to view Git branches across multiple repositories in subfolders.

![glimpse demo](assets/img/image.png)

## Overview

`glimpse` recursively scans directories for Git repositories and displays the currently checked-out branch for each one in a clean, tabular format. Perfect for managing multiple projects or microservices.

## Features

- 🔍 Recursively finds all Git repositories in subfolders
- 📊 Clean tabular output showing folder names and branches
- ⚡ Fast and lightweight
- 🎨 Color-coded output for better readability

## Installation

```bash
# Clone the repository
git clone https://github.com/ckissmann/glimpse.git
cd glimpse

# Make the script executable (if applicable)
chmod +x glimpse

# Optional: Add to your PATH
sudo cp glimpse /usr/local/bin/
```

## Usage

Navigate to the parent directory containing your Git repositories and run:

```bash
glimpse
```

### Example Output

```
Folder          | Branch
----------------+----------------
A               | branch-a
anc             | master
C               | branch-C
D               | branch-D
B               | branch-b
```

## How It Works

`glimpse` walks through the current directory and its subdirectories, identifies folders containing a `.git` directory, and extracts the current branch name from each repository.

## Requirements

- Git installed on your system
- Unix-like environment (Linux, macOS, WSL)

## Contributing

We love contributions! ❤️ This project follows semantic versioning and conventional commits to keep our changelog clean and releases automated.

### 🚀 Quick Start

1. **Fork the repository**
   - Click the "Fork" button at the top right of this page
   - This creates your own copy of the project

2. **Clone your fork**
   ```bash
   git clone https://github.com/YOUR_USERNAME/glimpse.git
   cd glimpse
   ```

3. **Add upstream remote**
   ```bash
   git remote add upstream https://github.com/ckissmann/glimpse.git
   ```

4. **Create a feature branch**
   ```bash
   # Keep your main branch clean and sync it with upstream
   git checkout main
   git pull upstream main
   
   # Create a new branch for your feature
   git checkout -b feature/your-feature-name
   # or for a bugfix
   git checkout -b fix/your-bugfix-name
   ```

### 📝 Commit Convention

We use [Conventional Commits](https://www.conventionalcommits.org/) for clear commit messages and automated versioning.

#### Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

#### Types

- **feat**: A new feature (triggers minor version bump)
- **fix**: A bug fix (triggers patch version bump)
- **docs**: Documentation changes only
- **style**: Code style changes (formatting, missing semicolons, etc.)
- **refactor**: Code changes that neither fix bugs nor add features
- **perf**: Performance improvements
- **test**: Adding or updating tests
- **chore**: Maintenance tasks, dependency updates, etc.
- **ci**: Changes to CI/CD configuration
- **build**: Changes to build system or dependencies

#### Examples

```bash
# New feature (0.1.0 → 0.2.0)
git commit -m "feat: add recursive depth limit option"

# Bug fix (0.1.0 → 0.1.1)
git commit -m "fix: handle repositories with detached HEAD state"

# Documentation
git commit -m "docs: update installation instructions for Windows"

# Breaking change (0.1.0 → 1.0.0)
git commit -m "feat!: change output format to JSON

BREAKING CHANGE: default output is now JSON instead of table format.
Use --format=table for the old behavior."

# With scope
git commit -m "feat(cli): add --color flag to disable colored output"
git commit -m "fix(parser): correct branch name extraction for special characters"
```

#### Multi-line Commits

```bash
git commit -m "feat: add support for filtering by branch pattern" -m "
- Added --pattern flag to filter repositories
- Supports glob patterns (e.g., feature/*)
- Updated documentation with examples
"
```

### 🔄 Workflow

1. **Make your changes**
   ```bash
   # Edit files
   vim src/main.rs
   
   # Test your changes
   cargo test
   cargo run
   ```

2. **Commit with semantic messages**
   ```bash
   git add .
   git commit -m "feat: add new feature"
   ```

3. **Keep your branch up to date**
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

4. **Push to your fork**
   ```bash
   git push origin feature/your-feature-name
   ```

5. **Open a Pull Request**
   - Go to https://github.com/ckissmann/glimpse
   - Click "New Pull Request"
   - Select your fork and branch
   - Fill in the PR template with details about your changes

### 🎯 Branch Naming Convention

Use descriptive branch names that match your commit type:

```bash
feature/add-depth-limit
feature/json-output
fix/detached-head-handling
fix/windows-path-support
docs/improve-readme
refactor/simplify-parser
test/add-integration-tests
chore/update-dependencies
```

### ✅ Pull Request Guidelines

- **Keep PRs focused**: One feature/fix per PR
- **Write descriptive titles**: Use conventional commit format
- **Update documentation**: If you change functionality, update the README
- **Add tests**: For new features or bug fixes
- **Keep commits clean**: Squash work-in-progress commits if needed
- **Follow code style**: Run `cargo fmt` and `cargo clippy`

#### PR Title Format

```
feat: add recursive depth limit option
fix: handle repositories with detached HEAD
docs: update installation instructions
```

### 🧪 Testing

Before submitting your PR:

```bash
# Run tests
cargo test

# Check formatting
cargo fmt --check

# Run linter
cargo clippy -- -D warnings

# Build release binary
cargo build --release

# Test the binary
./target/release/glimpse
```

### 📋 PR Checklist

- [ ] Code follows the project's style guidelines (`cargo fmt`)
- [ ] No compiler warnings (`cargo clippy`)
- [ ] All tests pass (`cargo test`)
- [ ] Documentation updated (if needed)
- [ ] Commit messages follow conventional commits
- [ ] Branch is up to date with main
- [ ] Self-review completed

### 🏷️ Semantic Versioning

This project follows [Semantic Versioning](https://semver.org/):

- **Major version** (1.0.0 → 2.0.0): Breaking changes
  - Use `feat!:` or `fix!:` or add `BREAKING CHANGE:` in commit body
- **Minor version** (0.1.0 → 0.2.0): New features (backwards compatible)
  - Use `feat:` commits
- **Patch version** (0.1.0 → 0.1.1): Bug fixes
  - Use `fix:` commits

Releases are automated based on commit messages, so following the convention is important!

### 🤝 Code of Conduct

- Be respectful and inclusive
- Provide constructive feedback
- Focus on the code, not the person
- Help others learn and grow

### 💡 Ideas and Suggestions

Have an idea but don't want to code it yourself? Open an issue!

1. Go to [Issues](https://github.com/ckissmann/glimpse/issues)
2. Click "New Issue"
3. Choose "Feature Request" or "Bug Report"
4. Describe your idea in detail

### 🆘 Need Help?

- Check existing [Issues](https://github.com/ckissmann/glimpse/issues)
- Read the [Documentation](docs/)
- Ask in [Discussions](https://github.com/ckissmann/glimpse/discussions)

### 🎉 Recognition

Contributors are recognized in:
- The [Contributors](https://github.com/ckissmann/glimpse/graphs/contributors) page
- Release notes for their specific contributions
- The project README (for significant contributions)

Thank you for contributing! 🚀

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

Your Name (@ckissmann)

## Acknowledgments

- Inspired by the need to quickly check branch status across multiple repositories
- Built for developers managing multiple Git projects

---

**Note:** Replace `ckissmann` and other placeholder information with your actual GitHub username and details.