const esbuild = require('esbuild');

esbuild.build({
    entryPoints: ['./src/extension.ts'],
    bundle: true,
    external: ['vscode'],
    platform: 'node',
    target: 'node16',
    outfile: 'out/extension.js',
    format: 'cjs',
    sourcemap: true,
    minify: false
}).catch(() => process.exit(1));