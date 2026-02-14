class Glimpse < Formula
  desc "Deine App Beschreibung"
  homepage "https://github.com/ckissmann/glimpse"
  version "2026.2.14.14"
  url "https://github.com/ckissmann/glimpse/archive/refs/tags/v2026.2.14.14.tar.gz"
  sha256 "143f478c029b1534fa56cb7b69b1da6d274ea8498d231222de99e4132eb94844"
  license "MIT"
  
  depends_on "rust" => :build

  def install
    system "cargo", "install", "--locked", "--root", prefix, "--path", "."
  end

  test do
    system "#{bin}/glimpse", "--version"
  end
end
