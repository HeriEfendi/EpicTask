import { execFileSync, execSync } from 'node:child_process';
import { readFileSync, writeFileSync, existsSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

// 1. Cari root directory dari file release.mjs
const __dirname = path.dirname(fileURLToPath(import.meta.url));
const rootDir = path.resolve(__dirname, '..');
process.chdir(rootDir);

// 2. Ambil parameter versi dari argumen CLI
// Dukung: npm run release 0.17.1 ATAU npm run release -- 0.17.1 ATAU v0.17.1
const args = process.argv.slice(2);
let rawInput = '';
for (const arg of args) {
  if (!arg.startsWith('-') && /\d+/.test(arg)) {
    rawInput = arg;
    break;
  }
}

if (!rawInput) {
  console.error('\n❌ Masukkan nomor versi untuk rilis!');
  console.error('Contoh: npm run release 0.17.1');
  console.error('     atau: npm run release -- 0.17.1\n');
  process.exit(1);
}

// Normalisasi: hilangkan prefix 'v', lalu ubah 0.17 -> 0.17.0
let version = rawInput.replace(/^v/i, '').trim();
if (/^\d+\.\d+$/.test(version)) {
  version = `${version}.0`;
}

if (!/^\d+\.\d+\.\d+(-[a-zA-Z0-9.]+)?$/.test(version)) {
  console.error(`\n❌ Format versi tidak valid: '${rawInput}'`);
  console.error('Gunakan format Semantic Versioning, misal: 0.17.1 atau 1.0.0\n');
  process.exit(1);
}

const run = (command, cmdArgs) => execFileSync(command, cmdArgs, { stdio: 'inherit', cwd: rootDir });
const runOut = (command, cmdArgs) => execFileSync(command, cmdArgs, { encoding: 'utf8', cwd: rootDir }).trim();

// 3. Deteksi Git Remote & Branch Aktif
const remotes = runOut('git', ['remote']).split(/\s+/).filter(Boolean);
const remote = remotes.includes('origin') ? 'origin' : remotes[0];
if (!remote) throw new Error('Tidak ada Git remote yang terkonfigurasi');

const initialBranch = runOut('git', ['branch', '--show-current']) || 'main';

console.log(`\n🚀 Memulai proses otomatis rilis EpicTask v${version}...`);
console.log(`📌 Branch aktif: ${initialBranch} | Remote: ${remote}\n`);

try {
  // A. Ambil update terbaru dari remote
  console.log(`[1/6] Mengambil update terbaru dari remote '${remote}'...`);
  run('git', ['fetch', remote]);

  // Jika di branch selain main, pindah & merge ke main
  if (initialBranch !== 'main') {
    console.log(`[2/6] Mengalihkan ke branch 'main' dan merge dari '${initialBranch}'...`);
    run('git', ['checkout', 'main']);
    run('git', ['merge', initialBranch, '-X', 'theirs', '--no-edit']);
  }

  // B. Generate Changelog dari commit history
  console.log(`[3/6] Menganalisis histori commit untuk CHANGELOG.md...`);
  const prevTag = runOut('git', ['tag', '--sort=-creatordate']).split(/\s+/).filter(Boolean)[0] || '';
  
  let gitLogRange = prevTag ? `${prevTag}..HEAD` : '-25';
  let rawCommits = '';
  try {
    rawCommits = runOut('git', ['log', gitLogRange, '--pretty=format:%s (%h)']);
  } catch (e) {
    rawCommits = runOut('git', ['log', '-20', '--pretty=format:%s (%h)']);
  }

  const commitLines = rawCommits.split('\n').filter((l) => l.trim() && !/^Release v\d+/i.test(l.trim()));
  
  const features = [];
  const fixes = [];
  const improvements = [];
  const others = [];

  for (const line of commitLines) {
    const clean = line.replace(/^[a-z]+(\([a-z0-9_-]+\))?:\s*/i, '');
    if (/^feat/i.test(line)) {
      features.push(`- ${clean}`);
    } else if (/^fix/i.test(line)) {
      fixes.push(`- ${clean}`);
    } else if (/^(refactor|perf|style)/i.test(line)) {
      improvements.push(`- ${clean}`);
    } else {
      others.push(`- ${line}`);
    }
  }

  const todayStr = new Date().toISOString().split('T')[0];
  let changelogSection = `## [v${version}] - ${todayStr}\n\n`;

  if (features.length > 0) {
    changelogSection += `### 🚀 Fitur Baru (Features)\n${features.join('\n')}\n\n`;
  }
  if (fixes.length > 0) {
    changelogSection += `### 🐛 Perbaikan Masalah (Bug Fixes)\n${fixes.join('\n')}\n\n`;
  }
  if (improvements.length > 0) {
    changelogSection += `### ⚡ Peningkatan & Refactoring\n${improvements.join('\n')}\n\n`;
  }
  if (others.length > 0 && features.length === 0 && fixes.length === 0) {
    changelogSection += `### 📝 Catatan Perubahan\n${others.join('\n')}\n\n`;
  }

  // Perbarui berkas CHANGELOG.md
  const changelogPath = path.resolve(rootDir, 'CHANGELOG.md');
  let currentChangelog = '';
  if (existsSync(changelogPath)) {
    currentChangelog = readFileSync(changelogPath, 'utf8');
  } else {
    currentChangelog = '# EpicTask Changelog\n\nSemua perubahan dan histori rilis dicatat di sini secara otomatis.\n\n';
  }

  const updatedChangelog = currentChangelog.includes('# EpicTask Changelog')
    ? currentChangelog.replace('# EpicTask Changelog\n\n', `# EpicTask Changelog\n\n${changelogSection}`)
    : `# EpicTask Changelog\n\n${changelogSection}${currentChangelog}`;

  writeFileSync(changelogPath, updatedChangelog, 'utf8');
  console.log(`✅ CHANGELOG.md berhasil diperbarui untuk v${version}.`);

  // C. Update nomor versi di semua berkas konfigurasi
  console.log(`[4/6] Memperbarui versi ke v${version} pada berkas proyek...`);

  // 1. Root package.json
  const rootPkgPath = path.resolve(rootDir, 'package.json');
  if (existsSync(rootPkgPath)) {
    const pkg = JSON.parse(readFileSync(rootPkgPath, 'utf8'));
    pkg.version = version;
    writeFileSync(rootPkgPath, `${JSON.stringify(pkg, null, 2)}\n`, 'utf8');
  }

  // 2. Frontend package.json
  const frontendPkgPath = path.resolve(rootDir, 'frontend/package.json');
  if (existsSync(frontendPkgPath)) {
    const fPkg = JSON.parse(readFileSync(frontendPkgPath, 'utf8'));
    fPkg.version = version;
    writeFileSync(frontendPkgPath, `${JSON.stringify(fPkg, null, 2)}\n`, 'utf8');
  }

  // 3. Frontend package-lock.json jika ada
  const frontendLockPath = path.resolve(rootDir, 'frontend/package-lock.json');
  if (existsSync(frontendLockPath)) {
    const fLock = JSON.parse(readFileSync(frontendLockPath, 'utf8'));
    fLock.version = version;
    if (fLock.packages && fLock.packages['']) fLock.packages[''].version = version;
    writeFileSync(frontendLockPath, `${JSON.stringify(fLock, null, 2)}\n`, 'utf8');
  }

  // 4. Tauri config (frontend/src-tauri/tauri.conf.json)
  const tauriConfPath = path.resolve(rootDir, 'frontend/src-tauri/tauri.conf.json');
  if (existsSync(tauriConfPath)) {
    const tauri = JSON.parse(readFileSync(tauriConfPath, 'utf8'));
    tauri.version = version;
    writeFileSync(tauriConfPath, `${JSON.stringify(tauri, null, 2)}\n`, 'utf8');
  }

  // 5. Desktop Tauri Cargo.toml
  const desktopCargoPath = path.resolve(rootDir, 'frontend/src-tauri/Cargo.toml');
  if (existsSync(desktopCargoPath)) {
    const cargo = readFileSync(desktopCargoPath, 'utf8');
    writeFileSync(desktopCargoPath, cargo.replace(/^(version\s*=\s*")[^"]+(")/m, `$1${version}$2`), 'utf8');
  }

  // 6. Backend Cargo.toml
  const backendCargoPath = path.resolve(rootDir, 'backend/Cargo.toml');
  if (existsSync(backendCargoPath)) {
    const cargo = readFileSync(backendCargoPath, 'utf8');
    writeFileSync(backendCargoPath, cargo.replace(/^(version\s*=\s*")[^"]+(")/m, `$1${version}$2`), 'utf8');
  }

  // D. Stage & Commit Release
  console.log(`[5/6] Membuat commit rilis dan git tag v${version}...`);
  run('git', [
    'add',
    'CHANGELOG.md',
    'package.json',
    'frontend/package.json',
    'frontend/src-tauri/tauri.conf.json',
    'frontend/src-tauri/Cargo.toml',
    'backend/Cargo.toml',
    'scripts/',
    '.github/',
  ]);

  if (existsSync(frontendLockPath)) {
    run('git', ['add', 'frontend/package-lock.json']);
  }

  run('git', ['commit', '-m', `Release v${version}`]);
  run('git', ['tag', '-f', '-a', `v${version}`, '-m', `Release v${version}`]);

  // E. Push ke GitHub
  console.log(`[6/6] Mendorong branch 'main' dan tag 'v${version}' ke GitHub (${remote})...`);
  run('git', ['push', remote, 'main']);
  run('git', ['push', '-f', remote, `v${version}`]);

  console.log(`\n🎉 SUKSES! Rilis EpicTask v${version} berhasil diterbitkan.`);
  console.log(`⚡ GitHub Actions workflow telah dipicu secara otomatis untuk membuat build aplikasi multi-platform:`);
  console.log(`   - 🐧 Linux: .deb, .rpm, .AppImage, .pkg.tar.zst (Arch)`);
  console.log(`   - 🪟 Windows: .exe (NSIS Installer), .msi`);
  console.log(`   - 🍏 macOS: .dmg (Apple Silicon & Intel)`);
  console.log(`\nCek progres kompilasi di: https://github.com/HeriEfendi/EpicTask/actions\n`);

} finally {
  // Jika awalnya bukan di branch main, kembalikan user ke branch awalnya
  if (initialBranch !== 'main') {
    console.log(`Mengembalikan ke branch awal '${initialBranch}'...`);
    run('git', ['checkout', initialBranch]);
    try {
      run('git', ['merge', 'main', '-X', 'theirs', '--no-edit']);
    } catch (e) {
      console.warn(`Catatan: Sinkronisasi balik ke '${initialBranch}' memerlukan penyelesaian manual.`);
    }
  }
}
