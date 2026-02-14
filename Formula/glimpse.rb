class Glimpse < Formula
  desc "Deine App Beschreibung"
  homepage "https://github.com/ckissmann/glimpse"
  url "https://github.com/ckissmann/glimpse/archive/refs/tags/v1.0.0.tar.gz"
  sha256 "HASH_HIER"
  license "MIT"
  
  depends_on "rust" => :build

  def install
    system "cargo", "install", "--locked", "--root", prefix, "--path", "."
  end

  test do
    system "#{bin}/glimpse", "--version"
  end
end