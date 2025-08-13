export const DEFAULT_PRESET_CONFIG = {
    inputs: [
        { name: 'APP_NAME', required: true, description: 'Name of the application (DNS-safe).' },
        { name: 'NAMESPACE', required: true, description: 'Kubernetes namespace for the app.' },
        { name: 'REPO_URL', required: true, description: 'Git repository URL with manifests.' },
        { name: 'REVISION', required: false, default: 'main', description: 'Git revision/branch/tag.' },
        { name: 'IMAGE_NAME', required: true, description: 'Container image (e.g. ghcr.io/org/app:tag).' },
        { name: 'CONTAINER_PORT', required: true, description: 'Container port exposed by the app.' },
        { name: 'SERVICE_PORT', required: false, description: 'Service port; defaults to CONTAINER_PORT if unset.' },
        { name: 'ENV', required: false, default: 'dev', description: 'Environment name for overlays (e.g., dev).' },
    ],
    help: {
        summary: 'Default preset for a generic app.',
        usage: 'argo-composer create app --profile default --preset default \\\n      --set APP_NAME=myapp --set NAMESPACE=apps --set REPO_URL=https://example/repo.git',
    },
}
