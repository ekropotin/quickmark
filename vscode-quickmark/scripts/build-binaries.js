const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

// Platform-specific binary targets
const targets = [
    { rust: 'x86_64-pc-windows-msvc', suffix: '.exe' },
    { rust: 'i686-pc-windows-msvc', suffix: '.exe' },
    { rust: 'x86_64-apple-darwin', suffix: '' },
    { rust: 'aarch64-apple-darwin', suffix: '' },
    { rust: 'x86_64-unknown-linux-gnu', suffix: '' },
    { rust: 'aarch64-unknown-linux-gnu', suffix: '' }
];

const projectRoot = path.resolve(__dirname, '../../..');
const binDir = path.join(__dirname, '..', 'bin');

// Ensure bin directory exists
if (!fs.existsSync(binDir)) {
    fs.mkdirSync(binDir, { recursive: true });
}

console.log('Building QuickMark server binaries...');

targets.forEach(target => {
    const binaryName = `quickmark_server-${target.rust}${target.suffix}`;
    const targetDir = path.join(projectRoot, 'target', target.rust, 'release');
    const sourceBinary = path.join(targetDir, `quickmark_server${target.suffix}`);
    const destBinary = path.join(binDir, binaryName);
    
    console.log(`Building for target: ${target.rust}`);
    
    try {
        // Build the binary for this target
        execSync(`cargo build --release --bin quickmark_server --target ${target.rust}`, {
            cwd: projectRoot,
            stdio: 'inherit'
        });
        
        // Copy the binary to our bin directory
        if (fs.existsSync(sourceBinary)) {
            fs.copyFileSync(sourceBinary, destBinary);
            
            // Make executable on Unix-like systems
            if (target.suffix === '') {
                fs.chmodSync(destBinary, '755');
            }
            
            console.log(`✓ Built and copied: ${binaryName}`);
        } else {
            console.warn(`⚠ Binary not found: ${sourceBinary}`);
        }
    } catch (error) {
        console.error(`✗ Failed to build ${target.rust}: ${error.message}`);
    }
});

console.log('Binary build complete!');