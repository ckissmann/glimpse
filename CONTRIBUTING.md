# Contributing to Glimpse

Thank you for your interest in contributing! This document provides guidelines and instructions for contributing to this project.

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Commit Convention](#commit-convention)
- [Pull Request Process](#pull-request-process)
- [Coding Standards](#coding-standards)
- [Testing](#testing)
- [Documentation](#documentation)

## 🤝 Code of Conduct

### Our Pledge

We are committed to providing a welcoming and inclusive environment for all contributors, regardless of experience level, gender, gender identity and expression, sexual orientation, disability, personal appearance, body size, race, ethnicity, age, religion, or nationality.

### Expected Behavior

- Be respectful and considerate
- Provide constructive feedback
- Accept constructive criticism gracefully
- Focus on what is best for the community
- Show empathy towards other community members

### Unacceptable Behavior

- Harassment, intimidation, or discrimination
- Trolling, insulting/derogatory comments, and personal or political attacks
- Public or private harassment
- Publishing others' private information without explicit permission
- Other conduct which could reasonably be considered inappropriate

## 🚀 Getting Started

### Prerequisites

- Rust 1.70 or higher
- Git
- A GitHub account

### Fork and Clone

1. **Fork the repository**
   - Navigate to https://github.com/ckissmann/glimpse
   - Click the "Fork" button in the top right

2. **Clone your fork**
   ```bash
   git clone https://github.com/YOUR_USERNAME/glimpse.git
   cd glimpse
   ```

3. **Add upstream remote**
   ```bash
   git remote add upstream https://github.com/ckissmann/glimpse.git
   git remote -v
   ```

### Setup Development Environment

```bash
# Install dependencies
cargo build

# Run tests
cargo test

# Run the application
cargo run

# Install for local testing
cargo install --path .
```

## 🔄 Development Workflow

### 1. Sync Your Fork

Always start with the latest code:

```bash
git checkout main
git fetch upstream
git merge upstream/main
git push origin main
```

### 2. Create a Feature Branch

Use descriptive branch names:

```bash
# For new features
git checkout -b feature/add-color-themes

# For bug fixes
git checkout -b fix/handle-empty-repos

# For documentation
git checkout -b docs/improve-installation-guide

# For refactoring
git checkout -b refactor/simplify-git-parser
```

### 3. Make Your Changes

- Write clean, readable code
- Follow Rust best practices
- Add tests for new features
- Update documentation as needed

### 4. Commit Your Changes

Follow our [commit convention](#commit-convention):

```bash
git add .
git commit -m "feat: add color theme support"
```

### 5. Push and Create PR

```bash
git push origin feature/add-color-themes
```

Then open a Pull Request on GitHub.

## 📝 Commit Convention

We use [Conventional Commits](https://www.conventionalcommits.org/) for automated versioning and changelog generation.

### Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

| Type | Description | Version Bump |
|------|-------------|--------------|
| `feat` | New feature | Minor (0.1.0 → 0.2.0) |
| `fix` | Bug fix | Patch (0.1.0 → 0.1.1) |
| `docs` | Documentation only | None |
| `style` | Formatting, white-space | None |
| `refactor` | Code restructuring | None |
| `perf` | Performance improvement | Patch |
| `test` | Adding/updating tests | None |
| `chore` | Maintenance, dependencies | None |
| `ci` | CI/CD changes | None |
| `build` | Build system changes | None |

### Breaking Changes

For breaking changes, use `!` or add `BREAKING CHANGE:` in the footer:

```bash
# Option 1: Using !
git commit -m "feat!: change default output format to JSON"

# Option 2: Using footer
git commit -m "feat: change output format" -m "
BREAKING CHANGE: Default output is now JSON. 
Use --format=table for previous behavior.
"
```

This triggers a major version bump (0.1.0 → 1.0.0).

### Examples

#### Simple Commits

```bash
# New feature
git commit -m "feat: add --depth flag to limit recursion"

# Bug fix
git commit -m "fix: handle symlinks correctly"

# Documentation
git commit -m "docs: add Windows installation instructions"

# Code style
git commit -m "style: format code with rustfmt"

# Refactoring
git commit -m "refactor: extract git operations into module"

# Performance
git commit -m "perf: optimize repository scanning"

# Tests
git commit -m "test: add integration tests for branch detection"

# Chores
git commit -m "chore: update dependencies"

# CI/CD
git commit -m "ci: add Windows build to GitHub Actions"
```

#### Commits with Scope

```bash
git commit -m "feat(cli): add --quiet flag for silent operation"
git commit -m "fix(parser): handle detached HEAD state"
git commit -m "docs(readme): update installation section"
git commit -m "test(integration): add tests for nested repositories"
```

#### Multi-line Commits

```bash
git commit -m "feat: add filtering by branch pattern" -m "
- Implement glob pattern matching
- Add --pattern flag to CLI
- Support multiple patterns
- Add documentation and examples

Closes #42
"
```

### Commit Message Guidelines

**Subject Line:**
- Use imperative mood ("add" not "added" or "adds")
- Don't capitalize first letter
- No period at the end
- Maximum 72 characters
- Be specific and descriptive

**Body:**
- Separate from subject with blank line
- Explain what and why, not how
- Wrap at 72 characters
- Use bullet points for multiple changes

**Footer:**
- Reference issues: `Closes #123`, `Fixes #456`
- Note breaking changes: `BREAKING CHANGE: description`

### Good vs Bad Examples

✅ **Good:**
```
feat: add support for custom color schemes

- Added Color struct to handle RGB values
- Implemented theme loading from config file
- Added three built-in themes: dark, light, solarized
- Updated CLI to accept --theme flag

Closes #78
```

❌ **Bad:**
```
Added colors

- did stuff
- updated things
```

## 🔀 Pull Request Process

### Before Submitting

Run these checks locally:

```bash
# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings

# Run tests
cargo test

# Build release
cargo build --release

# Test manually
./target/release/glimpse
```

### PR Title

Use conventional commit format:

```
feat: add color theme support
fix: handle repositories with no commits
docs: improve README installation section
```

### PR Description Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update

## How Has This Been Tested?
- [ ] Unit tests
- [ ] Integration tests
- [ ] Manual testing

## Checklist
- [ ] My code follows the style guidelines of this project
- [ ] I have performed a self-review of my own code
- [ ] I have commented my code, particularly in hard-to-understand areas
- [ ] I have made corresponding changes to the documentation
- [ ] My changes generate no new warnings
- [ ] I have added tests that prove my fix is effective or that my feature works
- [ ] New and existing unit tests pass locally with my changes
- [ ] Any dependent changes have been merged and published

## Related Issues
Closes #(issue number)
```

### PR Review Process

1. **Automated checks** must pass:
   - CI/CD builds successfully
   - All tests pass
   - Code meets style guidelines
   - No security vulnerabilities

2. **Code review** by maintainer:
   - Code quality and readability
   - Test coverage
   - Documentation completeness
   - Adherence to project standards

3. **Changes requested**: Address feedback and push updates

4. **Approval**: Once approved, maintainer will merge

### After Your PR is Merged

1. Delete your feature branch:
   ```bash
   git branch -d feature/your-feature
   git push origin --delete feature/your-feature
   ```

2. Sync your fork:
   ```bash
   git checkout main
   git pull upstream main
   git push origin main
   ```

## 💻 Coding Standards

### Rust Style Guide

Follow the [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/):

```rust
// Use rustfmt for automatic formatting
cargo fmt

// Run clippy for linting
cargo clippy -- -D warnings
```

### Code Organization

```
src/
├── main.rs          # Entry point, CLI handling
├── lib.rs           # Library code, public API
├── git.rs           # Git operations
├── display.rs       # Output formatting
├── config.rs        # Configuration handling
└── tests/           # Integration tests
    └── integration_test.rs
```

### Naming Conventions

- **Functions**: `snake_case`
- **Types/Structs**: `PascalCase`
- **Constants**: `SCREAMING_SNAKE_CASE`
- **Modules**: `snake_case`

### Error Handling

Use `Result<T, E>` and `?` operator:

```rust
// Good
fn read_config() -> Result<Config, ConfigError> {
    let contents = fs::read_to_string("config.toml")?;
    let config = toml::from_str(&contents)?;
    Ok(config)
}

// Avoid unwrap() in library code
// Only use unwrap() in examples or when panic is intentional
```

### Documentation

Document public APIs:

```rust
/// Scans a directory for git repositories.
///
/// # Arguments
///
/// * `path` - The directory path to scan
/// * `max_depth` - Maximum recursion depth (None for unlimited)
///
/// # Returns
///
/// A vector of repository information
///
/// # Errors
///
/// Returns an error if the directory cannot be read
///
/// # Examples
///
/// ```
/// use glimpse::scan_repositories;
///
/// let repos = scan_repositories(".", Some(3))?;
/// ```
pub fn scan_repositories(path: &Path, max_depth: Option<usize>) -> Result<Vec<Repo>, Error> {
    // Implementation
}
```

## 🧪 Testing

### Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_branch_detection

# With output
cargo test -- --nocapture

# Integration tests only
cargo test --test '*'

# Doc tests
cargo test --doc
```

### Writing Tests

#### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_branch_name() {
        let result = parse_branch_name("ref: refs/heads/main");
        assert_eq!(result, Some("main".to_string()));
    }

    #[test]
    fn test_parse_detached_head() {
        let result = parse_branch_name("abc123def456");
        assert_eq!(result, None);
    }
}
```

#### Integration Tests

```rust
// tests/integration_test.rs
use assert_cmd::Command;

#[test]
fn test_basic_scan() {
    let mut cmd = Command::cargo_bin("glimpse").unwrap();
    cmd.arg("--help")
        .assert()
        .success();
}
```

### Test Coverage

Aim for:
- 80%+ code coverage
- All public APIs tested
- Edge cases covered
- Error paths tested

## 📚 Documentation

### README

Update the README if you:
- Add new features
- Change CLI interface
- Modify installation steps
- Add new requirements

### Code Comments

```rust
// Explain complex logic
// Describe non-obvious decisions
// Reference issues or discussions when relevant

// Good comment:
// Use BTreeMap to maintain sorted order for deterministic output

// Bad comment:
// Create a map
```

### Changelog

We use [conventional-changelog](https://github.com/conventional-changelog/conventional-changelog) to automatically generate changelogs from commit messages.

Your commits will appear in the changelog based on their type:

- `feat:` → Features section
- `fix:` → Bug Fixes section
- `perf:` → Performance section
- Breaking changes → Breaking Changes section

## 🏷️ Release Process

Releases are automated based on conventional commits:

1. Commits are analyzed
2. Version is bumped (major/minor/patch)
3. Changelog is generated
4. Git tag is created
5. Binaries are built and published
6. Release notes are created

### Version Bumping

| Commit Type | Version Change | Example |
|-------------|---------------|---------|
| `fix:` | Patch | 0.1.0 → 0.1.1 |
| `feat:` | Minor | 0.1.0 → 0.2.0 |
| `feat!:` or `BREAKING CHANGE:` | Major | 0.1.0 → 1.0.0 |

## 🆘 Getting Help

- 📖 Check [existing documentation](docs/)
- 🐛 Search [existing issues](https://github.com/ckissmann/glimpse/issues)
- 💬 Start a [discussion](https://github.com/ckissmann/glimpse/discussions)
- 📧 Contact maintainers

## 🎉 Recognition

Contributors are recognized:
- In release notes
- On the Contributors page
- In the project README (for significant contributions)

## 📄 License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to Glimpse! 🚀