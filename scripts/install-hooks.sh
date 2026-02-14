#!/bin/bash
set -e

echo "📦 Installing git hooks..."

cat > .git/hooks/pre-commit << 'EOF'
#!/bin/bash
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
EOF

chmod +x .git/hooks/pre-commit
echo "✅ Git hooks installed!"
echo ""
echo "To skip: git commit --no-verify"