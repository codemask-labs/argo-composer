type GetImageUpdaterAnnotations = {
    imageName: string
    useImageUpdater?: boolean
}

export const getImageUpdaterAnnotations = ({ imageName, useImageUpdater }: GetImageUpdaterAnnotations): Record<string, string> => {
    if (!useImageUpdater) {
        return {}
    }

    return {
        'argocd-image-updater.argoproj.io/image-list': `i=${imageName}`,
        'argocd-image-updater.argoproj.io/i.update-strategy': 'latest',
        'argocd-image-updater.argoproj.io/i.allow-tags': 'regexp:([a-z0-9]+)-([0-9]+)',
        'argocd-image-updater.argoproj.io/i.force-update': 'true',
        'argocd-image-updater.argoproj.io/git-branch': 'main',
        'argocd-image-updater.argoproj.io/write-back-method': 'git',
        'argocd-image-updater.argoproj.io/write-back-target': 'kustomization'
    }
}
