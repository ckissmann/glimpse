use std::fs;
use std::os::unix::fs::PermissionsExt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📦 Installing git hooks...");

    // Create pre-commit hook
    let hook_content = r#"#!/bin/bash
set -e

echo "🔍 Running pre-commit checks..."

# Format
echo "📝 Checking format..."
cargo fmt -- --check || {
    echo "❌ Format check failed! Run: cargo fmt"
    exit 1
}

# Lint
echo "🔍 Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings || {
    echo "❌ Clippy failed!"
    exit 1
}

echo "✅ Pre-commit checks passed!"
"#;

    // Write hook
    let hook_path = ".git/hooks/pre-commit";
    fs::write(hook_path, hook_content)?;

    // Make executable
    let metadata = fs::metadata(hook_path)?;
    let mut permissions = metadata.permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(hook_path, permissions)?;

    println!("✅ Git hooks installed!");
    println!();
    println!("To skip hooks: git commit --no-verify");

    Ok(())
}
