import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

// ES 模块中没有 __dirname，需要手动构造
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// 从 package.json 读取版本号
const pkgPath = path.join(__dirname, '..', 'package.json');
const pkg = JSON.parse(fs.readFileSync(pkgPath, 'utf8'));
const version = pkg.version;

console.log(`Syncing version to ${version}...`);

// 更新 tauri.conf.json
const tauriConfigPath = path.join(__dirname, '..', 'src-tauri', 'tauri.conf.json');
const tauriConfig = JSON.parse(fs.readFileSync(tauriConfigPath, 'utf8'));
tauriConfig.version = version;
fs.writeFileSync(tauriConfigPath, JSON.stringify(tauriConfig, null, 2));
console.log('✓ Updated src-tauri/tauri.conf.json');

// 更新 Cargo.toml
const cargoPath = path.join(__dirname, '..', 'src-tauri', 'Cargo.toml');
let cargoToml = fs.readFileSync(cargoPath, 'utf8');
cargoToml = cargoToml.replace(/^version = ".*"/m, `version = "${version}"`);
fs.writeFileSync(cargoPath, cargoToml);
console.log('✓ Updated src-tauri/Cargo.toml');

console.log(`\n✓ Version ${version} synced successfully!`);
