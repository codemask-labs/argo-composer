#[cfg(test)]
mod tests {
    use crate::{YamlAbstractSyntaxTree, YamlValue};

    #[test]
    fn test_simple_scalar() {
        let documents = YamlAbstractSyntaxTree::parse_string("hello".to_string()).unwrap();
        assert_eq!(documents.len(), 1);
        assert_eq!(documents[0].nodes.len(), 1);
        assert_eq!(
            documents[0].nodes[0].value,
            YamlValue::String("hello".to_string())
        );
    }

    #[test]
    fn test_simple_object() {
        let yaml = "name: John\nage: 30";
        let documents = YamlAbstractSyntaxTree::parse_string(yaml.to_string()).unwrap();

        assert_eq!(documents.len(), 1);
        // No empty lines, so both keys are in same object
        assert_eq!(documents[0].nodes.len(), 1);
        match &documents[0].nodes[0].value {
            YamlValue::Object(pairs) => {
                assert_eq!(pairs.len(), 2);
                assert_eq!(pairs[0].0, "name");
                assert_eq!(pairs[1].0, "age");
            }
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_with_comments() {
        let yaml = "# Leading comment\nkey: value # inline comment\n# Another comment\nother: data";
        let documents = YamlAbstractSyntaxTree::parse_string(yaml.to_string()).unwrap();

        assert_eq!(documents.len(), 1);
        // Comment lines break objects, so: Comment, Object(key), Comment, Object(other)
        assert_eq!(documents[0].nodes.len(), 4);

        // First node is a comment
        match &documents[0].nodes[0].value {
            YamlValue::Comment(text) => assert_eq!(text, "Leading comment"),
            _ => panic!("Expected comment"),
        }

        // Second node is object with inline comment
        match &documents[0].nodes[1].value {
            YamlValue::Object(pairs) => {
                assert_eq!(pairs.len(), 1);
                assert_eq!(pairs[0].0, "key");
                assert_eq!(pairs[0].1.inline_comment.as_deref(), Some("inline comment"));
            }
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_indentation_detection() {
        let yaml = "parent:\n  child: value\n  another: data";
        let documents = YamlAbstractSyntaxTree::parse_string(yaml.to_string()).unwrap();

        assert_eq!(documents.len(), 1);
        assert_eq!(documents[0].nodes.len(), 1);
        match &documents[0].nodes[0].value {
            YamlValue::Object(pairs) => {
                assert_eq!(pairs.len(), 1);
                assert_eq!(pairs[0].0, "parent");
            }
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_sequence() {
        let yaml = "- apple\n- banana\n- cherry";
        let documents = YamlAbstractSyntaxTree::parse_string(yaml.to_string()).unwrap();

        assert_eq!(documents.len(), 1);
        assert_eq!(documents[0].nodes.len(), 1);
        match &documents[0].nodes[0].value {
            YamlValue::Collection(items) => {
                assert_eq!(items.len(), 3);
            }
            _ => panic!("Expected collection"),
        }
    }

    #[test]
    fn test_document_markers() {
        let yaml = "---\nkey: value\n...";
        let documents = YamlAbstractSyntaxTree::parse_string(yaml.to_string()).unwrap();

        assert_eq!(documents.len(), 1);
    }

    #[test]
    fn test_nested_structure() {
        let yaml = "parent:\n  child:\n    nested: value";
        let documents = YamlAbstractSyntaxTree::parse_string(yaml.to_string()).unwrap();

        assert_eq!(documents.len(), 1);
        assert_eq!(documents[0].nodes.len(), 1);
        // Verify nesting preserved
        match &documents[0].nodes[0].value {
            YamlValue::Object(pairs) => {
                assert_eq!(pairs.len(), 1);
                assert_eq!(pairs[0].0, "parent");
            }
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_real_yaml_document() {
        let yaml = r#"# ArgoCD Application manifests
apiVersion: argoproj.io/v1alpha1
kind: Application # Example application
metadata:
  name: example
  namespace: argocd # ArgoCD namespace
  annotations:
    argocd.argoproj.io/manifest-generate-paths: .
  finalizers:
    - resources-finalizer.argocd.argoproj.io # Ensure proper cleanup
spec:
  project: default # Default project
  source:
    repoURL: https://github.com/codemask-labs/argocd-resources
    targetRevision: main # Main branch
    path: ./example
  destination:
    server: https://kubernetes.default.svc
    namespace: argocd
---
# Second application for projects
apiVersion: argoproj.io/v1alpha1
kind: Application
metadata:
  name: projects
  namespace: argocd
  annotations:
    argocd.argoproj.io/manifest-generate-paths: .
  finalizers:
    - resources-finalizer.argocd.argoproj.io
spec:
  project: default
  source:
    repoURL: https://github.com/codemask-labs/argocd-resources
    targetRevision: main
    path: ./projects
  destination:
    server: https://kubernetes.default.svc
    namespace: argocd"#;

        let documents = YamlAbstractSyntaxTree::parse_string(yaml.to_string()).unwrap();

        // Should parse 2 documents (separated by ---)
        assert_eq!(documents.len(), 2);

        // First document should have a leading comment node and an object node
        assert!(documents[0].nodes.len() >= 2);

        // Verify leading comment is first node
        match &documents[0].nodes[0].value {
            YamlValue::Comment(text) => assert_eq!(text, "ArgoCD Application manifests"),
            _ => panic!("Expected comment as first node"),
        }

        // Find the main object node (skip comments)
        let root_obj = documents[0]
            .nodes
            .iter()
            .find(|n| matches!(n.value, YamlValue::Object(_)))
            .unwrap();
        match &root_obj.value {
            YamlValue::Object(pairs) => {
                // Verify top-level fields
                assert_eq!(pairs.len(), 4);
                assert_eq!(pairs[0].0, "apiVersion");
                assert_eq!(pairs[1].0, "kind");
                assert_eq!(pairs[2].0, "metadata");
                assert_eq!(pairs[3].0, "spec");

                // Verify apiVersion value
                match &pairs[0].1.value {
                    YamlValue::String(val) => assert_eq!(val, "argoproj.io/v1alpha1"),
                    _ => panic!("Expected string for apiVersion"),
                }

                // Verify kind value
                match &pairs[1].1.value {
                    YamlValue::String(val) => {
                        // Should be "Application" with inline comment
                        assert_eq!(val, "Application");
                        assert_eq!(
                            pairs[1].1.inline_comment.as_deref(),
                            Some("Example application")
                        );
                    }
                    _ => panic!("Expected string for kind"),
                }

                // Verify metadata structure
                match &pairs[2].1.value {
                    YamlValue::Object(metadata_pairs) => {
                        assert_eq!(metadata_pairs.len(), 4);
                        assert_eq!(metadata_pairs[0].0, "name");
                        assert_eq!(metadata_pairs[1].0, "namespace");
                        assert_eq!(metadata_pairs[2].0, "annotations");
                        assert_eq!(metadata_pairs[3].0, "finalizers");

                        // Check name value
                        match &metadata_pairs[0].1.value {
                            YamlValue::String(val) => assert_eq!(val, "example"),
                            _ => panic!("Expected string for name"),
                        }

                        // Check namespace value
                        match &metadata_pairs[1].1.value {
                            YamlValue::String(val) => {
                                // Should be "argocd" with inline comment
                                assert_eq!(val, "argocd");
                                assert_eq!(
                                    metadata_pairs[1].1.inline_comment.as_deref(),
                                    Some("ArgoCD namespace")
                                );
                            }
                            _ => panic!("Expected string for namespace"),
                        }

                        // Check annotations is an object
                        match &metadata_pairs[2].1.value {
                            YamlValue::Object(annotations) => {
                                assert_eq!(annotations.len(), 1);
                                assert_eq!(
                                    annotations[0].0,
                                    "argocd.argoproj.io/manifest-generate-paths"
                                );
                                match &annotations[0].1.value {
                                    YamlValue::String(val) => assert_eq!(val, "."),
                                    _ => panic!("Expected string for annotation value"),
                                }
                            }
                            _ => panic!("Expected object for annotations"),
                        }

                        // Check finalizers is a sequence
                        match &metadata_pairs[3].1.value {
                            YamlValue::Collection(items) => {
                                assert_eq!(items.len(), 1);
                                match &items[0].value {
                                    YamlValue::String(val) => {
                                        // Should have inline comment
                                        assert_eq!(val, "resources-finalizer.argocd.argoproj.io");
                                        assert_eq!(
                                            items[0].inline_comment.as_deref(),
                                            Some("Ensure proper cleanup")
                                        );
                                    }
                                    _ => panic!("Expected string in finalizers"),
                                }
                            }
                            _ => panic!("Expected collection for finalizers"),
                        }
                    }
                    _ => panic!("Expected object for metadata"),
                }

                // Verify spec structure
                match &pairs[3].1.value {
                    YamlValue::Object(spec_pairs) => {
                        assert_eq!(spec_pairs.len(), 3);
                        assert_eq!(spec_pairs[0].0, "project");
                        assert_eq!(spec_pairs[1].0, "source");
                        assert_eq!(spec_pairs[2].0, "destination");

                        // Check project value
                        match &spec_pairs[0].1.value {
                            YamlValue::String(val) => {
                                // Should be "default" with inline comment
                                assert_eq!(val, "default");
                                assert_eq!(
                                    spec_pairs[0].1.inline_comment.as_deref(),
                                    Some("Default project")
                                );
                            }
                            _ => panic!("Expected string for project"),
                        }

                        // Check source object
                        match &spec_pairs[1].1.value {
                            YamlValue::Object(source_pairs) => {
                                assert_eq!(source_pairs.len(), 3);
                                assert_eq!(source_pairs[0].0, "repoURL");
                                assert_eq!(source_pairs[1].0, "targetRevision");
                                assert_eq!(source_pairs[2].0, "path");

                                match &source_pairs[0].1.value {
                                    YamlValue::String(val) => assert_eq!(
                                        val,
                                        "https://github.com/codemask-labs/argocd-resources"
                                    ),
                                    _ => panic!("Expected string for repoURL"),
                                }
                                match &source_pairs[1].1.value {
                                    YamlValue::String(val) => {
                                        // Should be "main" with inline comment
                                        assert_eq!(val, "main");
                                        assert_eq!(
                                            source_pairs[1].1.inline_comment.as_deref(),
                                            Some("Main branch")
                                        );
                                    }
                                    _ => panic!("Expected string for targetRevision"),
                                }
                                match &source_pairs[2].1.value {
                                    YamlValue::String(val) => assert_eq!(val, "./example"),
                                    _ => panic!("Expected string for path"),
                                }
                            }
                            _ => panic!("Expected object for source"),
                        }

                        // Check destination object
                        match &spec_pairs[2].1.value {
                            YamlValue::Object(dest_pairs) => {
                                assert_eq!(dest_pairs.len(), 2);
                                assert_eq!(dest_pairs[0].0, "server");
                                assert_eq!(dest_pairs[1].0, "namespace");

                                match &dest_pairs[0].1.value {
                                    YamlValue::String(val) => {
                                        assert_eq!(val, "https://kubernetes.default.svc")
                                    }
                                    _ => panic!("Expected string for server"),
                                }
                                match &dest_pairs[1].1.value {
                                    YamlValue::String(val) => assert_eq!(val, "argocd"),
                                    _ => panic!("Expected string for namespace"),
                                }
                            }
                            _ => panic!("Expected object for destination"),
                        }
                    }
                    _ => panic!("Expected object for spec"),
                }
            }
            _ => panic!("Expected object for root"),
        }

        // Test second document has leading comment and content
        assert!(documents[1].nodes.len() >= 2);

        // Verify leading comment
        match &documents[1].nodes[0].value {
            YamlValue::Comment(text) => assert_eq!(text, "Second application for projects"),
            _ => panic!("Expected comment"),
        }

        // Find the main object
        let root_obj_2 = documents[1]
            .nodes
            .iter()
            .find(|n| matches!(n.value, YamlValue::Object(_)))
            .unwrap();
        match &root_obj_2.value {
            YamlValue::Object(pairs) => match &pairs[2].1.value {
                YamlValue::Object(metadata_pairs) => match &metadata_pairs[0].1.value {
                    YamlValue::String(val) => assert_eq!(val, "projects"),
                    _ => panic!("Expected string for name"),
                },
                _ => panic!("Expected object for metadata"),
            },
            _ => panic!("Expected object for root"),
        }
    }

    #[test]
    fn test_document_with_only_comments() {
        let yaml = r#"# This is just a comment
# Another comment
# No actual content"#;

        let documents = YamlAbstractSyntaxTree::parse_string(yaml.to_string()).unwrap();

        // Should have one document with 3 comment nodes
        assert_eq!(documents.len(), 1);
        assert_eq!(documents[0].nodes.len(), 3);

        // All nodes should be comments
        for node in &documents[0].nodes {
            assert!(matches!(node.value, YamlValue::Comment(_)));
        }
    }

    #[test]
    fn test_empty_document() {
        let yaml = "";
        let documents = YamlAbstractSyntaxTree::parse_string(yaml.to_string()).unwrap();
        assert_eq!(documents.len(), 0);
    }

    #[test]
    fn test_multiline_literal_string() {
        let yaml = r#"description: |
  This is a multiline
  literal string that
  preserves line breaks"#;

        let documents = YamlAbstractSyntaxTree::parse_string(yaml.to_string()).unwrap();
        assert_eq!(documents.len(), 1);
        assert_eq!(documents[0].nodes.len(), 1);

        match &documents[0].nodes[0].value {
            YamlValue::Object(pairs) => {
                assert_eq!(pairs[0].0, "description");
                match &pairs[0].1.value {
                    YamlValue::String(val) => {
                        assert!(val.contains('\n'));
                        assert!(val.contains("This is a multiline"));
                        assert!(val.contains("preserves line breaks"));
                    }
                    _ => panic!("Expected string"),
                }
            }
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_multiline_folded_string() {
        let yaml = r#"description: >
  This is a multiline
  folded string that
  joins into one line"#;

        let documents = YamlAbstractSyntaxTree::parse_string(yaml.to_string()).unwrap();
        assert_eq!(documents.len(), 1);
        assert_eq!(documents[0].nodes.len(), 1);

        match &documents[0].nodes[0].value {
            YamlValue::Object(pairs) => {
                assert_eq!(pairs[0].0, "description");
                match &pairs[0].1.value {
                    YamlValue::String(val) => {
                        // Folded should join lines with spaces
                        assert!(!val.contains('\n'));
                        assert!(val.contains("This is a multiline folded string"));
                    }
                    _ => panic!("Expected string"),
                }
            }
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_multiline_literal_strip() {
        let yaml = r#"description: |-
  Line 1
  Line 2
  Line 3"#;

        let documents = YamlAbstractSyntaxTree::parse_string(yaml.to_string()).unwrap();

        match &documents[0].nodes[0].value {
            YamlValue::Object(pairs) => {
                match &pairs[0].1.value {
                    YamlValue::String(val) => {
                        // Should not end with newline when using |-
                        assert!(!val.ends_with('\n'));
                        assert!(val.contains("Line 1"));
                    }
                    _ => panic!("Expected string"),
                }
            }
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_multiline_explicit_indent() {
        let yaml = r#"description: |2
    This has 4 spaces
    of indentation"#;

        let documents = YamlAbstractSyntaxTree::parse_string(yaml.to_string()).unwrap();
        assert_eq!(documents.len(), 1);
        assert_eq!(documents[0].nodes.len(), 1);

        match &documents[0].nodes[0].value {
            YamlValue::Object(pairs) => {
                assert_eq!(pairs[0].0, "description");
                match &pairs[0].1.value {
                    YamlValue::String(val) => {
                        assert!(val.contains("This has 4 spaces"));
                        assert!(val.contains("of indentation"));
                    }
                    _ => panic!("Expected string"),
                }
            }
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_multiline_folded_strip_explicit() {
        let yaml = r#"text: >-1
 Line 1
 Line 2"#;

        let documents = YamlAbstractSyntaxTree::parse_string(yaml.to_string()).unwrap();

        match &documents[0].nodes[0].value {
            YamlValue::Object(pairs) => {
                match &pairs[0].1.value {
                    YamlValue::String(val) => {
                        // Folded should join lines
                        assert!(!val.contains('\n'));
                        // Strip should remove trailing newline
                        assert!(!val.ends_with('\n'));
                        assert!(val.contains("Line 1"));
                    }
                    _ => panic!("Expected string"),
                }
            }
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_empty_lines_preserved() {
        let yaml = r#"# Header comment
first: value1

second: value2

# Middle comment

third: value3"#;

        let documents = YamlAbstractSyntaxTree::parse_string(yaml.to_string()).unwrap();
        assert_eq!(documents.len(), 1);

        // Should have: comment, first, empty, second, empty, comment, empty, third
        assert_eq!(documents[0].nodes.len(), 8);

        // Verify structure
        assert!(matches!(documents[0].nodes[0].value, YamlValue::Comment(_)));
        assert!(matches!(documents[0].nodes[1].value, YamlValue::Object(_)));
        assert!(matches!(documents[0].nodes[2].value, YamlValue::EmptyLine));
        assert!(matches!(documents[0].nodes[3].value, YamlValue::Object(_)));
        assert!(matches!(documents[0].nodes[4].value, YamlValue::EmptyLine));
        assert!(matches!(documents[0].nodes[5].value, YamlValue::Comment(_)));
        assert!(matches!(documents[0].nodes[6].value, YamlValue::EmptyLine));
        assert!(matches!(documents[0].nodes[7].value, YamlValue::Object(_)));
    }

    #[test]
    fn test_empty_lines_in_collections() {
        let yaml = r#"items:
  - one

  - two

  - three"#;

        let documents = YamlAbstractSyntaxTree::parse_string(yaml.to_string()).unwrap();
        assert_eq!(documents.len(), 1);
        assert_eq!(documents[0].nodes.len(), 1);

        match &documents[0].nodes[0].value {
            YamlValue::Object(pairs) => {
                assert_eq!(pairs[0].0, "items");
                match &pairs[0].1.value {
                    YamlValue::Collection(items) => {
                        // Should have: one, empty, two, empty, three
                        assert_eq!(items.len(), 5);

                        match &items[0].value {
                            YamlValue::String(s) => assert_eq!(s, "one"),
                            _ => panic!("Expected string"),
                        }
                        assert!(matches!(items[1].value, YamlValue::EmptyLine));
                        match &items[2].value {
                            YamlValue::String(s) => assert_eq!(s, "two"),
                            _ => panic!("Expected string"),
                        }
                        assert!(matches!(items[3].value, YamlValue::EmptyLine));
                        match &items[4].value {
                            YamlValue::String(s) => assert_eq!(s, "three"),
                            _ => panic!("Expected string"),
                        }
                    }
                    _ => panic!("Expected collection"),
                }
            }
            _ => panic!("Expected object"),
        }
    }
}
