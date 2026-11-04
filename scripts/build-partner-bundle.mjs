/**
 * Prepares the assets for an unsigned partner bundle (for example a shop-branded
 * installer). Official releases use `pnpm build`, which also runs the full test
 * suites and requires an Authenticode-signed privileged helper; this script keeps
 * the same frontend and helper build steps but skips that release-only signing
 * gate so a local Windows installer can be produced without a certificate.
 *
 * The resulting installer and helper are unsigned: Windows SmartScreen warns on
 * first run, and a distributed build should be signed with the partner's
 * certificate before it leaves the shop.
 *
 * Produce the branded installer with:
 *   pnpm tauri build --bundles nsis --config scripts/tauri-shop-build.conf.json
 */
import { execFileSync } from 'node:child_process';
import { copyFileSync, mkdirSync } from 'node:fs';
import { join, resolve } from 'node:path';

const root = resolve(import.meta.dirname, '..');

function run(command, args) {
  execFileSync(command, args, { cwd: root, stdio: 'inherit' });
}

run(process.execPath, [join(root, 'node_modules/vite/bin/vite.js'), 'build']);
run(process.execPath, [join(root, 'scripts/check-build-output.mjs')]);

const target = process.env.CARGO_TARGET_DIR ?? 'target';
run('cargo', ['build', '--release', '--locked', '-p', 'truongphatcomputer-privileged']);

const helperDirectory = join(root, 'src-tauri/resources/privileged');
mkdirSync(helperDirectory, { recursive: true });
copyFileSync(
  resolve(root, target, 'release', 'TruongPhatComputerPrivileged.exe'),
  join(helperDirectory, 'TruongPhatComputerPrivileged.exe')
);
console.log('Prepared frontend assets and the unsigned privileged helper.');
