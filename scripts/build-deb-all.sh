#!/bin/sh

# Cargo Zigbuild - Build for All Platforms
# POSIX-compatible version (works with /bin/sh)
# Builds your Rust binary for macOS, Linux, and Windows

cd "$(dirname "$0")" || exit
cd ".."

set -e

echo "🏗️  Building DEB packages..."

# Ensure buildx is set up
docker buildx create --use --name multiarch || docker buildx use multiarch

# Create output directories
mkdir -p dist/linux_amd64 dist/linux_arm64

# Build AMD64
echo ""
echo "📦 Building AMD64..."
docker buildx build \
  --platform linux/amd64 \
  -f Dockerfile.deb \
  --output type=local,dest=dist/tmp-amd64 \
  .

# Rename AMD64 packages
for file in dist/tmp-amd64/*.deb; do
  [ -f "$file" ] || continue
  base=$(basename "$file" .deb | sed 's/_amd64$//')
  mv "$file" "dist/linux_amd64/${base}_amd64.deb"
done
rm -rf dist/tmp-amd64

# Build ARM64
echo ""
echo "📦 Building ARM64..."
docker buildx build \
  --platform linux/arm64 \
  -f Dockerfile.deb \
  --output type=local,dest=dist/tmp-arm64 \
  .

# Rename ARM64 packages
for file in dist/tmp-arm64/*.deb; do
  [ -f "$file" ] || continue
  base=$(basename "$file" .deb | sed 's/_arm64$//')
  mv "$file" "dist/linux_arm64/${base}_arm64.deb"
done
rm -rf dist/tmp-arm64

echo ""
echo "✅ Build complete!"
echo ""
echo "📦 AMD64 packages:"
ls -lh dist/linux_amd64/*.deb
echo ""
echo "📦 ARM64 packages:"
ls -lh dist/linux_arm64/*.deb