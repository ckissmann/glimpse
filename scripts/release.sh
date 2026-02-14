#!/bin/sh
set -e


cd "$(dirname "$0")" || exit
cd ".."
cd ".github"
cd "workflows"

echo "🚀 Triggering release workflow..."
gh workflow run semantic-release.yml

echo "✅ Workflow triggered!"
echo ""
echo "📊 Check status:"
echo "  gh run list --workflow=semantic-release.yml"
echo ""
echo "📺 Watch logs:"
echo "  gh run watch"