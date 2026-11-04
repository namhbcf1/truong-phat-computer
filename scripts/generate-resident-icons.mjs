/**
 * Derive compact system-surface icons from the approved public vector artwork.
 * Run with `node scripts/generate-resident-icons.mjs`; Tauri supplies the rasterizer
 * and ICO encoder, so icon maintenance needs no extra image-tool dependency.
 */
import { mkdtempSync, readFileSync, writeFileSync, mkdirSync, copyFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { execFileSync } from 'node:child_process';

const root = fileURLToPath(new URL('../', import.meta.url));
const source = readFileSync(join(root, 'public/truongphatcomputer.svg'), 'utf8');
if (!source.includes('viewBox="0 0 1024 1024"') || !source.includes('url(#tp-background)')) {
  throw new Error('The approved logo structure changed; review the compact icon derivation.');
}

// macOS template surfaces need a transparent silhouette: drop the tile and the
// accent bar, then flatten the monogram so the system can tint it.
const templateBody = source
  .replace(/<title>[\s\S]*?<\/title>/u, '')
  .replace(/<defs>[\s\S]*?<\/defs>/u, '')
  .replace(/<rect width="1024" height="1024"[^>]*\/>/u, '')
  .replace(/<rect x="148" y="816"[^>]*\/>/u, '')
  .replace(/\s*fill="#ffffff"/gu, ' fill="#000000"')
  .replace(/^[\s\S]*?<svg[^>]*>/u, '')
  .replace(/<\/svg>\s*$/u, '');
const template = `<svg xmlns="http://www.w3.org/2000/svg" width="36" height="36" viewBox="128 272 710 480">${templateBody}</svg>`;
const temporary = mkdtempSync(join(tmpdir(), 'truongphat-icons-'));
const output = join(root, 'src-tauri/icons');
const cli = join(root, 'node_modules/@tauri-apps/cli/tauri.js');
function generate(input, directory, sizes = []) {
  execFileSync(
    process.execPath,
    [cli, 'icon', input, '--output', directory, ...sizes.flatMap(size => ['--png', String(size)])],
    {
      cwd: root,
      stdio: 'inherit',
    }
  );
}
try {
  const colorSource = join(temporary, 'color.svg');
  const templateSource = join(temporary, 'template.svg');
  writeFileSync(colorSource, source);
  writeFileSync(templateSource, template);
  generate(templateSource, join(temporary, 'template'), [36]);
  generate(colorSource, join(temporary, 'color'), [64]);
  generate(colorSource, join(temporary, 'application'));
  copyFileSync(join(temporary, 'template/36x36.png'), join(output, 'tray-template.png'));
  copyFileSync(join(temporary, 'color/64x64.png'), join(output, 'tray-color.png'));
  // Canonical application icon set consumed by the Tauri bundler and the
  // Windows executable resources.
  for (const name of ['32x32.png', '128x128.png', '128x128@2x.png', 'icon.png', 'icon.ico', 'icon.icns']) {
    copyFileSync(join(temporary, 'application', name), join(output, name));
  }
  mkdirSync(join(output, 'windows'), { recursive: true });
  // Tauri embeds the first ICO entry as the live window icon. Put the largest
  // raster first to avoid enlarging 32 px pixels on high-DPI taskbars. Windows
  // still selects the appropriate size from the full executable icon directory.
  const ico = readFileSync(join(temporary, 'application/icon.ico'));
  const count = ico.readUInt16LE(4);
  const entries = Array.from({ length: count }, (_, index) => ico.subarray(6 + index * 16, 22 + index * 16));
  entries.sort((left, right) => (right[0] || 256) - (left[0] || 256));
  writeFileSync(
    join(output, 'windows/icon.ico'),
    Buffer.concat([ico.subarray(0, 6), ...entries, ico.subarray(6 + count * 16)])
  );
  console.log('Generated macOS template, color tray, and Windows application icons.');
} finally {
  rmSync(temporary, { recursive: true, force: true });
}
