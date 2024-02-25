export type SecurityContext = {
    allowPrivilegeEscalation: boolean
    runAsNonRoot?: boolean
    runAsUser?: number
}
