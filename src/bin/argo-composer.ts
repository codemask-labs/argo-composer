#!/usr/bin/env node

import { getPackageJson, program, usage, version } from '@codemaskjs/node-cli-toolkit'
import { ADD_COMMAND, APPLY_COMMAND, COPY_COMMAND, INIT_COMMAND, MOVE_COMMAND, REMOVE_COMMAND, RENAME_COMMAND } from 'commands'
import { ROOT_COMPOSER_DIR } from 'lib/common'

const { version: programVersion } = getPackageJson(ROOT_COMPOSER_DIR)

program([
    usage('argo-composer'),
    version(programVersion),
    INIT_COMMAND,
    ADD_COMMAND,
    REMOVE_COMMAND,
    MOVE_COMMAND,
    COPY_COMMAND,
    RENAME_COMMAND,
    APPLY_COMMAND
    // commands([
    //     INIT_COMMAND,
    //     ADD_COMMAND,
    //     REMOVE_COMMAND,
    //     MOVE_COMMAND,
    //     COPY_COMMAND,
    //     RENAME_COMMAND,
    //     APPLY_COMMAND
    // ])
])
