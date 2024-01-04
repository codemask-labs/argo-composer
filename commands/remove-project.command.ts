import { program } from 'commander'
import { ChildProcess, spawn } from 'child_process'
import { red } from 'chalk'

export const removeProjectCommand = () =>
    program
        .command('remove-project')
        .alias('rp')
        .description('Remove project with applications')
        .action(() => {
            const child: ChildProcess = spawn(
                'npx @angular-devkit/schematics-cli',
                ['../schematics:remove-project', '--debug=false'],
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
                            'failed'
                        )
                    )
                }

                return
            })
        })
