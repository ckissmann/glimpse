class Glimpse < Formula
  desc "Deine App Beschreibung"
  homepage "https://github.com/ckissmann/glimpse"
  url "https://github.com/ckissmann/glimpse/archive/refs/tags/refs/heads/release.tar.gz"
  sha256 "d5558cd419c8d46bdc958064cb97f963d1ea793866414c025906ec15033512ed"
  license "MIT"
  
  depends_on "rust" => :build

  def install
    system "cargo", "install", "--locked", "--root", prefix, "--path", "."
  end

  test do
    system "#{bin}/glimpse", "--version"
  end
end