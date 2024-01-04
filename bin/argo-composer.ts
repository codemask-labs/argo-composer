#!/usr/bin/env node
import { program } from 'commander'
import { commandLoader } from '../commands'

const bootstrap = async () => {
    program
        .version(
            require('../../package.json').version,
            '-v, --version',
            'Output the current version.',
        )

    commandLoader()

     program.parse(process.argv)
}

bootstrap()
