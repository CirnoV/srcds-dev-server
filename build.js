const fs = require('fs');
const child_process = require('child_process');

child_process.execSync('cargo build --release', { stdio: 'inherit' });
fs.copyFileSync('target/release/touhou-dev-server.exe', '../touhou-dev-server.exe');
