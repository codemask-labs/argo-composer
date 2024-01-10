import {
    apply,
    Rule,
    SchematicContext,
    Tree,
    url,
    template,
    strings
} from '@angular-devkit/schematics'

interface InitProjectSchematicOptions {
    name: string;
    repoURL: string
}

export const init = (options: InitProjectSchematicOptions): Rule => (_tree: Tree, context: SchematicContext) => {
    const templateSource = apply(url('./files'), [
        template({ ...options, ...strings })
    ])

    return templateSource(context)
}
