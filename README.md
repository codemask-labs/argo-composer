# Argo Composer

An interactive command-line interface for initializing, creating and maintaining your ArgoCD resources repository in a configurable, templated and structured way.

## Features
- Easy to use interractive commands for creating, copying and removing Argo Application manifests
- Designed to be compatibile with `apps-of-apps` pattern
- Set of ready-to-use templates for Applications

## Installation

```bash
$ npm install -g @codemaskinc/argo-composer
```

```bash
$ yarn global add @codemaskinc/argo-composer
```

## Initializing your argocd resources repository

After successful installation, we can now proceed to initialize the `argocd-resources` repository by calling the following command:

```bash
$ argo-composer init-project
```

The argo composer will take you through the interractive process of asking few questions on how you want your project to look like.

### 1. What name would you like to use for the project?
> [!TIP]
> The `name of the project` is simply a directory name where the `argo composer` will initialize to.
> Also, leaving this option empty will initialize the project in the `current working directory`.

### 2. What is the base URL of GitHub repository?
> [!TIP]
> The GitHub repository is then saved in root configuration, to be used as a default value when creating your Application.

### 3. What will be the environment inside your cluster?

> [!TIP]
> The current default environment configuration is `dev` and `prod`, used for creating your Application overlays.
> Choose the environment to your project needs, for example it could be: `integration`, `staging`, `production`.

### 4. Do you want to install any additional components?
> [!TIP]
> Select needed components (aka addons) to your project - by default it is `infra` - creating an example applications for each selected component and installs via Helm.

### 5. What name would you like to use for addons?
> [!TIP]
> Configurable to your liking, choose a project name for example `common` (by default `infra`).

[](./examples/example-init-project.png)

## Commands

To begin working with `argo-composer` command line, we recommend to use a help command first by calling:

```bash
$ argo-composer --help
```

[](./examples/example-help-command.png)

### Adding project

To add a another `argocd project`, use:

```bash
$ argo-composer add-project
```

### Removing project

To remove project, use:

```bash
$ argo-composer remove-project
```

### Adding application to a project

To add a another `argocd application` to a existing project, use:

```bash
$ argo-composer add-app
```

### Removing application for a project

To remove application, use:

```bash
$ argo-composer remove-app
```

### Copying existing application to a project

To copy application, use:

```bash
$ argo-composer copy-app
```

## Contribution and development

[How to start in development guide](https://github.com/codemaskinc/argo-composer/blob/main/development.md)
