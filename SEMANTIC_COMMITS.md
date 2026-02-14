# Semantic Release Configuration

This directory contains templates and configuration for semantic releases.

## Conventional Commits Cheatsheet

### Quick Reference

```bash
# Features (0.1.0 → 0.2.0)
git commit -m "feat: add new feature"
git commit -m "feat(scope): add new feature in specific area"

# Fixes (0.1.0 → 0.1.1)
git commit -m "fix: resolve bug"
git commit -m "fix(scope): resolve bug in specific area"

# Breaking Changes (0.1.0 → 1.0.0)
git commit -m "feat!: breaking feature change"
git commit -m "fix!: breaking bug fix"
# OR
git commit -m "feat: new feature" -m "BREAKING CHANGE: description"

# Other (no version bump)
git commit -m "docs: update documentation"
git commit -m "style: format code"
git commit -m "refactor: restructure code"
git commit -m "test: add tests"
git commit -m "chore: update dependencies"
git commit -m "ci: update CI config"
git commit -m "perf: improve performance"
```

### Type Details

| Type | Description | Version Impact | Changelog Section |
|------|-------------|----------------|-------------------|
| `feat` | New feature | MINOR (0.x.0) | Features |
| `fix` | Bug fix | PATCH (0.0.x) | Bug Fixes |
| `docs` | Documentation | None | - |
| `style` | Code style/formatting | None | - |
| `refactor` | Code refactoring | None | - |
| `perf` | Performance improvement | PATCH (0.0.x) | Performance |
| `test` | Tests | None | - |
| `build` | Build system | None | - |
| `ci` | CI/CD | None | - |
| `chore` | Maintenance | None | - |
| `revert` | Revert previous commit | Depends | Reverts |

### Scope Examples

Scope is optional but helpful for organizing changes:

```bash
feat(cli): add --verbose flag
fix(parser): handle empty files
docs(readme): update installation steps
test(integration): add E2E tests
refactor(core): simplify main loop
perf(scanner): optimize directory traversal
```

### Multi-line Commits

```bash
# Template
git commit -m "type(scope): short description" -m "
Detailed explanation of what was changed and why.
Can span multiple lines.

- Bullet points for specific changes
- More details
- Reference to discussions

Closes #123
Fixes #456
"
```

### Breaking Changes

Two ways to indicate breaking changes:

**Option 1: Using `!`**
```bash
git commit -m "feat!: change API response format"
```

**Option 2: Using footer**
```bash
git commit -m "feat: change API response format" -m "
BREAKING CHANGE: API now returns JSON instead of XML.
Update your code to parse JSON responses.
"
```

### Footer Keywords

- `Closes #123` - Closes issue #123
- `Fixes #456` - Fixes issue #456
- `Resolves #789` - Resolves issue #789
- `Refs #111` - References issue #111
- `BREAKING CHANGE:` - Indicates breaking change

### Examples by Scenario

**Adding a new command-line flag:**
```bash
git commit -m "feat(cli): add --quiet flag for silent operation" -m "
Added -q/--quiet flag to suppress all output except errors.
Useful for scripting and automation.

Closes #42
"
```

**Fixing a crash:**
```bash
git commit -m "fix: prevent crash on empty repositories" -m "
Handle case where .git/HEAD doesn't exist or is empty.
Now returns None instead of panicking.

Fixes #89
"
```

**Performance improvement:**
```bash
git commit -m "perf: optimize repository scanning" -m "
Use parallel iteration for directory traversal.
Reduces scan time by ~60% on large directory trees.
"
```

**Breaking API change:**
```bash
git commit -m "feat!: change branch name detection" -m "
BREAKING CHANGE: GitRepo struct now uses Option<String> for branch field.

Previous behavior returned empty string for detached HEAD.
Now returns None to distinguish between no branch and empty branch name.

Migration guide:
- Before: if repo.branch.is_empty() { ... }
- After: if repo.branch.is_none() { ... }
"
```

**Documentation update:**
```bash
git commit -m "docs: add Windows installation guide" -m "
Added step-by-step installation instructions for Windows users.
Includes troubleshooting section for common PATH issues.
"
```

## Semantic Versioning Reference

Given a version number MAJOR.MINOR.PATCH (e.g., 1.2.3):

- **MAJOR** (1.x.x): Breaking changes
  - Commit with `!` or `BREAKING CHANGE:` footer
  - Example: 1.2.3 → 2.0.0

- **MINOR** (x.2.x): New features (backwards compatible)
  - Commit with `feat:` type
  - Example: 1.2.3 → 1.3.0

- **PATCH** (x.x.3): Bug fixes
  - Commit with `fix:` or `perf:` type
  - Example: 1.2.3 → 1.2.4

## Pre-release Versions

For pre-release versions:
- `1.0.0-alpha.1` - Alpha release
- `1.0.0-beta.1` - Beta release
- `1.0.0-rc.1` - Release candidate

## Automation

When using semantic-release:

1. **Analyze commits** since last release
2. **Determine version bump** based on commit types
3. **Generate changelog** from commits
4. **Create git tag** with new version
5. **Publish release** with binaries
6. **Update package managers** (Homebrew, APT, etc.)

## Best Practices

### ✅ Do:
- Write clear, descriptive commit messages
- Use present tense ("add feature" not "added feature")
- Keep subject line under 72 characters
- Separate subject from body with blank line
- Use body to explain what and why, not how
- Reference issues in footer
- One logical change per commit

### ❌ Don't:
- Mix multiple unrelated changes in one commit
- Use vague messages like "fix stuff" or "update"
- Forget to add type prefix
- Make breaking changes without proper notation
- Skip the subject line
- Use past tense

## Tools

### Commitizen
Interactive CLI for creating conventional commits:
```bash
npm install -g commitizen cz-conventional-changelog
git cz
```

### Commitlint
Lint commit messages:
```bash
npm install -g @commitlint/cli @commitlint/config-conventional
echo "module.exports = {extends: ['@commitlint/config-conventional']}" > commitlint.config.js
```

### Husky
Git hooks for enforcing commit conventions:
```bash
npm install -g husky
npx husky install
npx husky add .husky/commit-msg 'npx --no -- commitlint --edit ${1}'
```

## Resources

- [Conventional Commits](https://www.conventionalcommits.org/)
- [Semantic Versioning](https://semver.org/)
- [Keep a Changelog](https://keepachangelog.com/)
- [How to Write a Git Commit Message](https://chris.beams.io/posts/git-commit/)

---

**Remember**: Good commit messages make code review easier, help with debugging, and create a useful project history!
