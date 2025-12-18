use super::*;
use yaml::Yaml;

#[test]
fn test_application_default() {
    let app = Application::default();

    assert_eq!(app.api_version, "argoproj.io/v1alpha1");
    assert_eq!(app.kind, "Application");
    assert!(app.metadata.is_none());

    // Verify that optional None fields are not serialized (including their comments)
    let yaml_output = Yaml::serialize_to_string(app).expect("Failed to serialize");
    println!("Minimal Application YAML:\n{}", yaml_output);

    // metadata should not appear (it's None)
    assert!(!yaml_output.contains("metadata"));
    // But required fields should appear
    assert!(yaml_output.contains("apiVersion"));
    assert!(yaml_output.contains("spec"));
}

#[test]
fn test_application_serialization() {
    let mut app = Application::default();

    // Set metadata
    app.metadata = Some(ObjectMeta {
        name: Some("my-app".to_string()),
        namespace: Some("argocd".to_string()),
        labels: None,
        annotations: None,
        finalizers: None,
    });

    // Set spec
    app.spec.project = Some("default".to_string());
    app.spec.source = Some(ApplicationSource {
        repo_url: "https://github.com/argoproj/argocd-example-apps".to_string(),
        path: "guestbook".to_string(),
        target_revision: "HEAD".to_string(),
        ..Default::default()
    });
    app.spec.destination = Some(ApplicationDestination {
        server: Some("https://kubernetes.default.svc".to_string()),
        namespace: Some("default".to_string()),
        name: None,
    });

    let yaml_output = Yaml::serialize_to_string(app).expect("Failed to serialize");

    println!("Serialized YAML:\n{}", yaml_output);

    // Verify key fields are present in YAML (with camelCase field names)
    assert!(yaml_output.contains("apiVersion"));
    assert!(yaml_output.contains("kind"));
    assert!(yaml_output.contains("Application"));
}

#[test]
fn test_application_with_sync_policy() {
    let mut app = Application::default();

    app.spec.sync_policy = Some(SyncPolicy {
        automated: Some(SyncPolicyAutomated {
            prune: Some(true),
            self_heal: Some(true),
            allow_empty: Some(false),
        }),
        sync_options: vec![
            "CreateNamespace=true".to_string(),
            "PruneLast=true".to_string(),
        ],
        retry: Some(RetryStrategy {
            limit: Some(5),
            backoff: Some(Backoff {
                duration: Some("5s".to_string()),
                factor: Some(2),
                max_duration: Some("3m".to_string()),
            }),
        }),
        managed_namespace_metadata: None,
    });

    let yaml_output = Yaml::serialize_to_string(app).expect("Failed to serialize");

    assert!(yaml_output.contains("syncPolicy"));
    assert!(yaml_output.contains("automated"));
}

#[test]
fn test_application_with_helm_source() {
    let source = ApplicationSource {
        repo_url: "https://charts.example.com".to_string(),
        chart: "nginx".to_string(),
        target_revision: "1.2.3".to_string(),
        helm: Some(HelmOptions {
            value_files: vec!["values-prod.yaml".to_string()],
            values: Some("replicas: 3\nimage:\n  tag: v1.2.3".to_string()),
            parameters: vec![HelmParameter {
                name: "service.type".to_string(),
                value: "LoadBalancer".to_string(),
                force_string: Some(false),
            }],
            release_name: Some("my-nginx".to_string()),
            ..Default::default()
        }),
        ..Default::default()
    };

    let mut app = Application::default();
    app.spec.source = Some(source);

    let yaml_output = Yaml::serialize_to_string(app).expect("Failed to serialize");

    assert!(yaml_output.contains("helm"));
    assert!(yaml_output.contains("nginx"));
}

#[test]
fn test_application_with_kustomize_source() {
    let source = ApplicationSource {
        repo_url: "https://github.com/myorg/myapp".to_string(),
        path: "overlays/production".to_string(),
        target_revision: "main".to_string(),
        kustomize: Some(KustomizeOptions {
            images: vec!["myapp=myapp:v1.2.3".to_string()],
            name_prefix: Some("prod-".to_string()),
            namespace: Some("production".to_string()),
            common_labels: Some(
                [("env".to_string(), "production".to_string())]
                    .iter()
                    .cloned()
                    .collect(),
            ),
            ..Default::default()
        }),
        ..Default::default()
    };

    let mut app = Application::default();
    app.spec.source = Some(source);

    let yaml_output = Yaml::serialize_to_string(app).expect("Failed to serialize");

    assert!(yaml_output.contains("kustomize"));
}

#[test]
fn test_app_project_default() {
    let project = AppProject::default();

    assert_eq!(project.api_version, "argoproj.io/v1alpha1");
    assert_eq!(project.kind, "AppProject");
    assert!(project.metadata.is_none());
}

#[test]
fn test_app_project_serialization() {
    let mut project = AppProject::default();

    project.metadata = Some(ObjectMeta {
        name: Some("my-project".to_string()),
        namespace: Some("argocd".to_string()),
        labels: None,
        annotations: None,
        finalizers: None,
    });

    project.spec.description = Some("Production project".to_string());
    project.spec.source_repos = vec![
        "https://github.com/myorg/*".to_string(),
        "https://charts.example.com".to_string(),
    ];
    project.spec.destinations = vec![ApplicationDestination {
        server: Some("https://kubernetes.default.svc".to_string()),
        namespace: Some("production".to_string()),
        name: None,
    }];

    let yaml_output = Yaml::serialize_to_string(project).expect("Failed to serialize");

    println!("AppProject YAML:\n{}", yaml_output);

    assert!(yaml_output.contains("apiVersion"));
    assert!(yaml_output.contains("AppProject"));
    assert!(yaml_output.contains("sourceRepos"));
}

#[test]
fn test_app_project_with_roles() {
    let mut project = AppProject::default();

    project.spec.roles = vec![ProjectRole {
        name: "admin".to_string(),
        description: Some("Admin role".to_string()),
        policies: vec![
            "p, proj:my-project:admin, applications, *, my-project/*, allow".to_string(),
        ],
        groups: vec!["my-org:admins".to_string()],
        jwt_tokens: vec![],
    }];

    let yaml_output = Yaml::serialize_to_string(project).expect("Failed to serialize");

    assert!(yaml_output.contains("roles"));
    assert!(yaml_output.contains("admin"));
}

#[test]
fn test_app_project_with_resource_restrictions() {
    let mut project = AppProject::default();

    project.spec.cluster_resource_whitelist = vec![
        GroupKind {
            group: Some("".to_string()),
            kind: "Namespace".to_string(),
        },
        GroupKind {
            group: Some("rbac.authorization.k8s.io".to_string()),
            kind: "ClusterRole".to_string(),
        },
    ];

    project.spec.namespace_resource_blacklist = vec![GroupKind {
        group: Some("".to_string()),
        kind: "ResourceQuota".to_string(),
    }];

    let yaml_output = Yaml::serialize_to_string(project).expect("Failed to serialize");

    assert!(yaml_output.contains("clusterResourceWhitelist"));
    assert!(yaml_output.contains("namespaceResourceBlacklist"));
}

#[test]
fn test_app_project_with_sync_windows() {
    let mut project = AppProject::default();

    project.spec.sync_windows = vec![
        SyncWindow {
            kind: Some("allow".to_string()),
            schedule: Some("0 9 * * 1-5".to_string()), // Weekdays 9 AM
            duration: Some("8h".to_string()),
            applications: vec![],
            namespaces: vec![],
            clusters: vec![],
            manual_sync: None,
        },
        SyncWindow {
            kind: Some("deny".to_string()),
            schedule: Some("0 0 * * 0,6".to_string()), // Weekends
            duration: Some("24h".to_string()),
            applications: vec![],
            namespaces: vec![],
            clusters: vec![],
            manual_sync: Some(true),
        },
    ];

    let yaml_output = Yaml::serialize_to_string(project).expect("Failed to serialize");

    assert!(yaml_output.contains("syncWindows"));
}

#[test]
fn test_application_round_trip() {
    let mut original = Application::default();
    original.metadata = Some(ObjectMeta {
        name: Some("test-app".to_string()),
        namespace: Some("argocd".to_string()),
        labels: None,
        annotations: None,
        finalizers: None,
    });
    original.spec.project = Some("default".to_string());
    original.spec.source = Some(ApplicationSource {
        repo_url: "https://github.com/test/repo".to_string(),
        path: "manifests".to_string(),
        target_revision: "HEAD".to_string(),
        ..Default::default()
    });

    let yaml_string = Yaml::serialize_to_string(original.clone()).expect("Failed to serialize");
    let deserialized: Application = Yaml::from_string(&yaml_string).expect("Failed to deserialize");

    assert_eq!(deserialized.api_version, "argoproj.io/v1alpha1");
    assert_eq!(deserialized.kind, "Application");
    assert!(deserialized.metadata.is_some());
}

#[test]
fn test_app_project_round_trip() {
    let mut original = AppProject::default();
    original.metadata = Some(ObjectMeta {
        name: Some("test-project".to_string()),
        namespace: Some("argocd".to_string()),
        labels: None,
        annotations: None,
        finalizers: None,
    });
    original.spec.description = Some("Test project".to_string());
    original.spec.source_repos = vec!["*".to_string()];

    let yaml_string = Yaml::serialize_to_string(original.clone()).expect("Failed to serialize");
    let deserialized: AppProject = Yaml::from_string(&yaml_string).expect("Failed to deserialize");

    assert_eq!(deserialized.api_version, "argoproj.io/v1alpha1");
    assert_eq!(deserialized.kind, "AppProject");
    assert!(deserialized.metadata.is_some());
}
