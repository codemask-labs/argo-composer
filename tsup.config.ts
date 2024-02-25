import { defineConfig } from 'tsup'

const vendorEntry = 'src/index.ts'
const binEntry = 'src/bin/argo-composer.ts'

export default defineConfig({
    name: 'tsup',
    target: 'esnext',
    clean: true,
    splitting: true,
    cjsInterop: true,
    format: ['esm'],
    dts: {
        resolve: true,
        entry: vendorEntry
    },
    esbuildOptions: options => {
        // eslint-disable-next-line functional/immutable-data
        options.chunkNames = 'vendor'
    },
    entry: [
        vendorEntry,
        binEntry
    ]
})
