const fs = require('fs');
const path = require('path');

// This script copies a locally built binary for development/testing
// Usage: node scripts/copy-local-binary.js

const projectRoot = path.resolve(__dirname, '../..');
const binDir = path.join(__dirname, '..', 'bin');

// Determine current platform binary
const platform = process.platform;
const arch = process.arch;

let binaryName;
let sourceBinary;

switch (platform) {
    case 'win32':
        binaryName = arch === 'x64' ? 'quickmark-server-x86_64-pc-windows-msvc.exe' : 'quickmark-server-i686-pc-windows-msvc.exe';
        sourceBinary = path.join(projectRoot, 'target', 'release', 'quickmark-server.exe');
        break;
    case 'darwin':
        binaryName = arch === 'arm64' ? 'quickmark-server-aarch64-apple-darwin' : 'quickmark-server-x86_64-apple-darwin';
        sourceBinary = path.join(projectRoot, 'target', 'release', 'quickmark-server');
        break;
    case 'linux':
        binaryName = arch === 'arm64' ? 'quickmark-server-aarch64-unknown-linux-gnu' : 'quickmark-server-x86_64-unknown-linux-gnu';
        sourceBinary = path.join(projectRoot, 'target', 'release', 'quickmark-server');
        break;
    default:
        console.error(`Unsupported platform: ${platform}-${arch}`);
        process.exit(1);
}

// Ensure bin directory exists
if (!fs.existsSync(binDir)) {
    fs.mkdirSync(binDir, { recursive: true });
}

const destBinary = path.join(binDir, binaryName);

if (!fs.existsSync(sourceBinary)) {
    console.error(`Source binary not found: ${sourceBinary}`);
    console.error('Please build the quickmark-server first with: cargo build --release --bin quickmark-server');
    process.exit(1);
}

// Copy the binary
fs.copyFileSync(sourceBinary, destBinary);

// Make executable on Unix-like systems
if (platform !== 'win32') {
    fs.chmodSync(destBinary, '755');
}

console.log(`✓ Copied ${sourceBinary} to ${destBinary}`);
console.log('Binary is ready for extension development!');
