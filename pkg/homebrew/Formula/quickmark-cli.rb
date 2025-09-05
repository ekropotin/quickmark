class QuickmarkCli < Formula
  desc "Lightning-fast Markdown/CommonMark linter CLI tool with tree-sitter based parsing"
  homepage "https://github.com/ekropotin/quickmark"
  license "MIT"
  version "1.1.0"

  on_macos do
    if Hardware::CPU.intel?
      url "https://github.com/ekropotin/quickmark/releases/download/quickmark-cli%40#{version}/qmark-x86_64-apple-darwin.tar.gz"
      sha256 "sha256:90740f9c0632d8b1da4d00c9c6361c01eb9a72c074641f6952723e6583bbdd8d"
    else
      url "https://github.com/ekropotin/quickmark/releases/download/quickmark-cli%40#{version}/qmark-aarch64-apple-darwin.tar.gz"
      sha256 "sha256:4466f54fd304d34d21dba7871a09d4d24df6c23f7cee48fae1f4a6a1f5466855"
    end
  end

  test do
    # Create a test markdown file
    (testpath/"test.md").write("# Test\n\nThis is a test.")

    # Run qmark on the test file
    system "#{bin}/qmark", "#{testpath}/test.md"
  end
end
