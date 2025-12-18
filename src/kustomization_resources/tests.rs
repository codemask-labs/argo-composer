use super::*;
use yaml::Yaml;

#[test]
fn test_kustomization_default() {
    let kustomization = Kustomization::default();

    assert_eq!(kustomization.api_version, "kustomize.config.k8s.io/v1beta1");
    assert_eq!(kustomization.kind, "Kustomization");
    assert!(kustomization.resources.is_empty());
    assert!(kustomization.images.is_none());
}

#[test]
fn test_kustomization_new() {
    let kustomization = Kustomization::new();

    assert_eq!(kustomization.api_version, "kustomize.config.k8s.io/v1beta1");
    assert_eq!(kustomization.kind, "Kustomization");
}

#[test]
fn test_kustomization_serialization() {
    let kustomization = Kustomization {
        api_version: "kustomize.config.k8s.io/v1beta1".to_string(),
        kind: "Kustomization".to_string(),
        resources: vec![
            "deployment.yaml".to_string(),
            "service.yaml".to_string(),
            "configmap.yaml".to_string(),
        ],
        images: None,
    };

    let yaml_output = Yaml::serialize_to_string(kustomization).expect("Failed to serialize");

    assert!(yaml_output.contains("api_version"));
    assert!(yaml_output.contains("kind"));
    assert!(yaml_output.contains("Kustomization"));
    assert!(yaml_output.contains("resources"));
    assert!(yaml_output.contains("deployment.yaml"));
    assert!(yaml_output.contains("service.yaml"));
}

#[test]
fn test_kustomization_with_multiple_resources() {
    let kustomization = Kustomization {
        api_version: "kustomize.config.k8s.io/v1beta1".to_string(),
        kind: "Kustomization".to_string(),
        resources: vec![
            "../base".to_string(),
            "namespace.yaml".to_string(),
            "ingress.yaml".to_string(),
        ],
        images: None,
    };

    let yaml_output = Yaml::serialize_to_string(kustomization).expect("Failed to serialize");

    assert!(yaml_output.contains("../base"));
    assert!(yaml_output.contains("namespace.yaml"));
    assert!(yaml_output.contains("ingress.yaml"));
}

#[test]
fn test_kustomization_round_trip() {
    let original = Kustomization {
        api_version: "kustomize.config.k8s.io/v1beta1".to_string(),
        kind: "Kustomization".to_string(),
        resources: vec!["app.yaml".to_string(), "service.yaml".to_string()],
        images: None,
    };

    let yaml_string = Yaml::serialize_to_string(original.clone()).expect("Failed to serialize");
    let deserialized: Kustomization =
        Yaml::from_string(&yaml_string).expect("Failed to deserialize");

    assert_eq!(deserialized.api_version, "kustomize.config.k8s.io/v1beta1");
    assert_eq!(deserialized.kind, "Kustomization");
    assert_eq!(deserialized.resources.len(), 2);
}

#[test]
fn test_kustomization_empty_resources() {
    let kustomization = Kustomization::default();

    let yaml_output = Yaml::serialize_to_string(kustomization).expect("Failed to serialize");

    // Even with empty resources, should serialize successfully
    assert!(yaml_output.contains("api_version"));
    assert!(yaml_output.contains("kind"));
}

#[test]
fn test_kustomization_deserialization_from_yaml() {
    let yaml_str = r#"
api_version: kustomize.config.k8s.io/v1beta1
kind: Kustomization
resources:
  - deployment.yaml
  - service.yaml
"#;

    let kustomization: Kustomization = Yaml::from_string(yaml_str).expect("Failed to deserialize");

    assert_eq!(kustomization.api_version, "kustomize.config.k8s.io/v1beta1");
    assert_eq!(kustomization.kind, "Kustomization");
    assert_eq!(kustomization.resources.len(), 2);
    assert_eq!(kustomization.resources[0], "deployment.yaml");
    assert_eq!(kustomization.resources[1], "service.yaml");
}
