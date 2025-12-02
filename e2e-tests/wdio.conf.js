import { spawn } from 'child_process';
import { resolve, join } from 'path';
import { homedir } from 'os';

const __dirname = new URL('.', import.meta.url).pathname;

export const config = {
  specs: ['./e2e-tests/specs/**/*.js'],
  maxInstances: 1,
  capabilities: [
    {
      maxInstances: 1,
      'tauri:options': {
        application: resolve(
          __dirname,
          '../src-tauri/target/release/honnyaku',
        ),
      },
    },
  ],
  reporters: ['spec'],
  framework: 'mocha',
  mochaOpts: {
    timeout: 60000,
  },
  logLevel: 'info',

  // tauri-driverプロセス
  beforeSession: () =>
    (globalThis.__TAURI_DRIVER__ = spawn(
      join(homedir(), '.cargo', 'bin', 'tauri-driver'),
      [],
      { stdio: [null, process.stdout, process.stderr] },
    )),

  // セッション後のクリーンアップ
  afterSession: () => globalThis.__TAURI_DRIVER__.kill(),

  // テスト準備：Rustプロジェクトのビルド
  onPrepare: () => {
    return new Promise((resolve, reject) => {
      const buildProcess = spawn('npm', ['run', 'tauri', 'build'], {
        stdio: 'inherit',
        shell: true,
        cwd: resolve(__dirname, '..'),
      });

      buildProcess.on('close', (code) => {
        if (code === 0) {
          resolve();
        } else {
          reject(new Error(`Build failed with code ${code}`));
        }
      });
    });
  },
};
