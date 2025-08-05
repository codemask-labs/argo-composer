import { codemaskConfig, codemaskImportConfig, codemaskStylisticConfig } from 'eslint-config-codemask'

export default [
    ...codemaskConfig,
    ...codemaskImportConfig,
    ...codemaskStylisticConfig
]
