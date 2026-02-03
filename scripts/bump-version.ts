import { readFileSync, writeFileSync } from 'fs';
import { join } from 'path';

const newVersion = process.argv[2];

if (!newVersion) {
  console.error('Usage: bun run version <new_version>');
  process.exit(1);
}

// 1. Update package.json
const packageJsonPath = join(process.cwd(), 'package.json');
const packageJson = JSON.parse(readFileSync(packageJsonPath, 'utf-8'));
packageJson.version = newVersion;
writeFileSync(packageJsonPath, JSON.stringify(packageJson, null, 2) + '\n');
console.log(`Updated package.json to ${newVersion}`);

// 2. Update src-tauri/tauri.conf.json
const tauriConfPath = join(process.cwd(), 'src-tauri', 'tauri.conf.json');
const tauriConf = JSON.parse(readFileSync(tauriConfPath, 'utf-8'));
tauriConf.version = newVersion;
writeFileSync(tauriConfPath, JSON.stringify(tauriConf, null, 2) + '\n');
console.log(`Updated tauri.conf.json to ${newVersion}`);

// 3. Update src-tauri/Cargo.toml
const cargoTomlPath = join(process.cwd(), 'src-tauri', 'Cargo.toml');
let cargoToml = readFileSync(cargoTomlPath, 'utf-8');
// Replace the first occurrence of version = "..."
cargoToml = cargoToml.replace(/^version = "[^"]+"/m, `version = "${newVersion}"`);
writeFileSync(cargoTomlPath, cargoToml);
console.log(`Updated Cargo.toml to ${newVersion}`);

console.log('All versions updated successfully!');
