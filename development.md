# Argo Composer Development

To start Argo Composer in development first we need to understand the structure of this repository. We have currently divided the project into two areas of concern:

- **cli** - a nodejs and commander application for running commands (considered a root directory of this repository)
- **schematics** - a dependency as separate package, containing logic and requirements for commands using (`@angular-devkit/schematics`) [schematics for libraries](https://angular.io/guide/schematics-for-libraries)

## Project structure
/<br>
↳ /bin<br>
↳ /commands<br>
↳ /runtime<br>
↳ /schematics<br>

## Installing, building and linking dependencies in development

1. To install dependencies for `cli`, run:

```bash
$ yarn install
```

2. To install dependencies for Schematics:

Go to `schematics` directory by running:

```bash
$ cd schematics
```

and install dependencies by running:

```bash
$ yarn install
```

3. Build Schematics (in `schematics` directory) by running:

```bash
$ yarn build
```

or, if you would like to run the build in watch mode, run:

> [!NOTE]  
> Please take a note if you are running the build in watch mode, you have to link the cli after every schematics build

```bash
$ yarn build:watch
```

4. Now we can leave the `schematics` directory by returning to root of a repository by running:

```bash
$ cd ..
```

5. Link the cli with built schematics, by running:

```bash
$ yarn link:dev
```

## Running first command

After completing the installation we are now ready to use our `argo-composer` command globally!

Every command currently implements `inquirer.js` ([documentation](https://github.com/SBoudrias/Inquirer.js)) for using the commands interractively.

> [!NOTE]  
> Run all of your commands (except `argo-composer init-project`) from root of your argocd resources repository, alongside the `argo-composer.config.yaml`.

## Help command

Use `argo-composer --help` for more information on available commands, and their descriptions.

```bash
$ argo-composer --help
```

## Project initialization

To create our `root application` for argocd, run:

```bash
$ argo-composer init-project
```

### Addons

During initialization of the project you will be asked if you would like to install additional components to your project. The current supported list of addons, are:
- Ingress Nginx - (https://github.com/kubernetes/ingress-nginx)
- Certificate Manager - (https://cert-manager.io/)
- Reflector - (https://github.com/emberstack/kubernetes-reflector)
- Argocd image updater - (https://argocd-image-updater.readthedocs.io/en/stable/)

## Adding project

To add a another `argocd project` ([documentation](https://argo-cd.readthedocs.io/en/stable/operator-manual/project-specification/)), use:

```bash
$ argo-composer add-project
```

## Removing project

To remove project, use:

```bash
$ argo-composer remove-project
```

## Adding application to a project

To add a another `argocd application` ([documentation](https://argo-cd.readthedocs.io/en/stable/user-guide/application-specification/)) to a existing project, use:

```bash
$ argo-composer add-app
```

## Removing application for a project

To remove application, use:

```bash
$ argo-composer remove-app
```

## Removing application for a project

To copy application, use:

```bash
$ argo-composer copy-app
```
