import { program } from 'commander'
import { ChildProcess, spawn } from 'child_process'
import { red } from 'chalk'

export const initCommand = () =>
    program
        .command('init-project')
        .alias('i')
        .description('Init new project')
        .action(() => {
            const child: ChildProcess = spawn(
                'npx @angular-devkit/schematics-cli',
                ['./schematics:init-project', '--debug=false'],
                { shell: true, stdio: 'inherit' }
            )

            child.on('error', error => {
                console.error(
                    red(
                        'failed',
                        error
                    )
                )
            })

            child.on('close', code => {
                if (code !== 0) {
                    console.error(
                        red(
                            'canceled'
                        )
                    )
                }

                return
            })
        })
