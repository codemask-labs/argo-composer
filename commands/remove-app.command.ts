import { program } from 'commander'
import { ChildProcess, spawn } from 'child_process'
import { red } from 'chalk'

export const removeAppCommand = () =>
    program
        .command('remove-app')
        .alias('ra')
        .description('Remove app from project')
        .action(() => {
            const child: ChildProcess = spawn(
                'npx @angular-devkit/schematics-cli',
                ['../schematics:remove-app', '--debug=false'],
                { shell: true, stdio: 'inherit' }
            )

            child.on('error', (error) => {
                console.error(
                    red(
                        'failed',
                        error
                    ),
                )
            })

            child.on('close', (code) => {
                if (code === 0) {
                    // resolve(null)
                    return
                } else {
                    console.error(
                        red(
                            'failed',
                        ),
                    )

                    return
                }
            })
        })
