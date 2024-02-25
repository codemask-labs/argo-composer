import { defineConfig } from 'tsup'

export default defineConfig({
    name: 'tsup',
    target: 'esnext',
    clean: true,
    splitting: true,
    cjsInterop: true,
    format: ['esm'],
    dts: {
        resolve: true,
        entry: 'src/index.ts'
    },
    esbuildOptions: options => {
        // eslint-disable-next-line functional/immutable-data
        options.chunkNames = 'vendor'
    },
    entry: [
        'src/index.ts',
        'src/bin/argo-composer.ts'
    ]
})
