import { writeFileSync } from 'fs';
import { resolve as resolvePath } from 'path';
import glob from 'glob';
import mainPackageJson from './package.json';
import packageJson from './pkg/package.json';

const pkgDir = resolvePath(__dirname, 'pkg');

packageJson.browser = 'node_jsxn_browser.js';

packageJson.files = glob.sync('*.+(js|ts|wasm)', { cwd: pkgDir });

packageJson.repository = mainPackageJson.repository;
packageJson.bugs = mainPackageJson.bugs;
packageJson.homepage = mainPackageJson.homepage;

writeFileSync(
  resolvePath(pkgDir, 'package.json'),
  JSON.stringify(packageJson, null, '  '),
);
