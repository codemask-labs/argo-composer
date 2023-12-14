# Getting Started With Schematics

This repository is a basic Schematic implementation that serves as a starting point to create and publish Schematics to NPM.

### Testing

To test locally, install `@angular-devkit/schematics-cli` globally and use the `schematics` command line tool. That tool acts the same as the `generate` command of the Angular CLI, but also has a debug mode.

Check the documentation with

```bash
schematics --help
```

### Unit Testing

`npm run test` will run the unit tests, using Jasmine as a runner and test framework.

### Publishing

To publish, simply do:

```bash
npm run build
npm publish
```

That's it!

### TODO:
1. Add commands to remove application, project
2. Add initial config.yaml to root folder
3. Validate inputs dynamically
4. More and more...

### How to test:
1. Init project: 
```shell
npx @angular-devkit/schematics-cli ./schematics:init-project --debug=false
```
2. Enter to created project:
```shell
cd your_project_name
```
3. Create a argo project or app:
```shell
npx @angular-devkit/schematics-cli ../schematics:add-project --debug=false
npx @angular-devkit/schematics-cli ../schematics:add-app --debug=false
```
