use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📦 Installing git hooks...\n");

    if !std::path::Path::new(".git").exists() {
        eprintln!("❌ Not a git repository!");
        eprintln!("   Run this from the project root.");
        std::process::exit(1);
    }

    // Pre-commit hook
    let pre_commit_hook = r#"#!/bin/bash
set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Get staged files
STAGED=$(git diff --cached --name-only --diff-filter=ACM)

if [ -z "$STAGED" ]; then
    echo -e "${RED}❌ No files staged for commit!${NC}"
    echo ""
    echo "You have unstaged changes. Stage them first:"
    echo -e "  ${YELLOW}git add <files>${NC}"
    echo ""
    echo "Current status:"
    git status --short
    exit 1
fi

echo -e "${BLUE}📦 Staged files:${NC}"
echo "$STAGED" | sed 's/^/  /'
echo ""

# Check if we have Rust files
RUST_FILES=$(echo "$STAGED" | grep '\.rs$' || true)

if [ -z "$RUST_FILES" ]; then
    echo -e "${YELLOW}⚠️  No Rust files staged, skipping checks${NC}"
    exit 0
fi

echo -e "${BLUE}🔍 Running checks on staged Rust files...${NC}"
echo ""

# Format check
echo -e "${YELLOW}📝 Format check...${NC}"
if ! cargo fmt -- --check >/dev/null 2>&1; then
    echo -e "${RED}❌ Format check failed!${NC}"
    echo "Run: ${YELLOW}cargo fmt${NC}"
    exit 1
fi
echo -e "${GREEN}✅ Format OK${NC}"

# Clippy
echo -e "${YELLOW}🔍 Clippy...${NC}"
if ! cargo clippy --all-targets --all-features -- -D warnings 2>&1 | \
     grep -v "warning: unused manifest key" | \
     grep -q "Finished"; then
    echo -e "${RED}❌ Clippy failed!${NC}"
    exit 1
fi
echo -e "${GREEN}✅ Clippy OK${NC}"

echo ""
echo -e "${GREEN}✅ All checks passed!${NC}"
"#;

    // Commit-msg hook
    let commit_msg_hook = r#"#!/bin/bash

COMMIT_MSG_FILE=$1
COMMIT_MSG=$(cat "$COMMIT_MSG_FILE")

# Skip merge commits
if echo "$COMMIT_MSG" | grep -q "^Merge"; then
    exit 0
fi

# Semantic commit pattern
PATTERN="^(feat|fix|docs|style|refactor|perf|test|build|ci|chore|revert)(\([a-z0-9-]+\))?!?: .{1,100}$"

if ! echo "$COMMIT_MSG" | head -n 1 | grep -qE "$PATTERN"; then
    echo "❌ Invalid commit message format!"
    echo ""
    echo "Format: <type>[optional scope]: <description>"
    echo ""
    echo "Types: feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert"
    echo ""
    echo "Examples:"
    echo "  feat: add user authentication"
    echo "  fix(api): handle null pointer"
    echo "  feat!: breaking change"
    echo ""
    echo "Your message:"
    echo "  $COMMIT_MSG"
    exit 1
fi

exit 0
"#;

    // Create hooks directory
    fs::create_dir_all(".git/hooks")?;

    // Write pre-commit hook
    fs::write(".git/hooks/pre-commit", pre_commit_hook)?;
    #[cfg(unix)]
    {
        let metadata = fs::metadata(".git/hooks/pre-commit")?;
        let mut permissions = metadata.permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(".git/hooks/pre-commit", permissions)?;
    }

    // Write commit-msg hook
    fs::write(".git/hooks/commit-msg", commit_msg_hook)?;
    #[cfg(unix)]
    {
        let metadata = fs::metadata(".git/hooks/commit-msg")?;
        let mut permissions = metadata.permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(".git/hooks/commit-msg", permissions)?;
    }

    println!("✅ Git hooks installed!");
    println!();
    println!("Hooks:");
    println!("  • pre-commit:  checks format & lint");
    println!("  • commit-msg:  validates semantic commit format");
    println!();
    println!("To skip hooks: git commit --no-verify");

    Ok(())
}
