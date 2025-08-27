class QuickmarkCli < Formula
  desc "Lightning-fast Markdown/CommonMark linter CLI tool with tree-sitter based parsing"
  homepage "https://github.com/ekropotin/quickmark"
  license "MIT"
  version "1.0.0"

  on_macos do
    if Hardware::CPU.intel?
      url "https://github.com/ekropotin/quickmark/releases/download/quickmark-cli%40#{version}/qmark-x86_64-apple-darwin"
      sha256 "309161921d26ea93f1b8f3f6738346bcf032e42a12b600363b43f76f87158bba"
    else
      url "https://github.com/ekropotin/quickmark/releases/download/quickmark-cli%40#{version}/qmark-aarch64-apple-darwin"
      sha256 "c6cc057df011d1df9ee2d0a60d6f2634d78561b57d3afd85cbd89715d737649d"
    end
  end

  def install
    if Hardware::CPU.intel?
      bin.install "qmark-x86_64-apple-darwin" => "qmark"
    else
      bin.install "qmark-aarch64-apple-darwin" => "qmark"
    end
  end

  test do
    # Create a test markdown file
    (testpath/"test.md").write("# Test\n\nThis is a test.")

    # Run qmark on the test file
    system "#{bin}/qmark", "#{testpath}/test.md"
  end
end
