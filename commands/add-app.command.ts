import { program } from 'commander'
import { ChildProcess, spawn } from 'child_process'
import { red } from 'chalk'

export const addAppCommand = () =>
    program
        .command('add-app')
        .alias('aa')
        .description('Add new app to project')
        .action(() => {
            const child: ChildProcess = spawn(
                'npx @angular-devkit/schematics-cli',
                ['../schematics:add-app', '--debug=false'],
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
