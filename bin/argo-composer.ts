#!/usr/bin/env node
import { program } from 'commander'
import { runtime } from '../runtime'
import { commandLoader } from '../commands'

program.version(runtime.package.version, '-v, --version', 'Output the current version.')

commandLoader()

program.parse(process.argv)
