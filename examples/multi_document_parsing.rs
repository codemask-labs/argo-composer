use yaml::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
struct Application {
    pub kind: String,
    pub name: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Service {
    pub kind: String,
    pub name: String,
    pub port: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct ConfigMap {
    pub kind: String,
    pub name: String,
    pub data: String,
}

// Enum to hold any of the document types
#[derive(Debug)]
enum Resource {
    Application(Application),
    Service(Service),
    ConfigMap(ConfigMap),
    Unknown(String),
}

fn main() {
    // Example multi-document YAML with different types
    let yaml_content = r#"
kind: Application
name: my-app
version: 1.0.0
---
kind: Service
name: api-service
port: 8080
---
kind: ConfigMap
name: app-config
data: some configuration
---
kind: Application
name: another-app
version: 2.0.0
"#;

    println!("=== Parsing Multiple Documents with Different Types ===\n");

    // Parse all documents
    let documents = parse_documents_from_string(yaml_content).expect("Failed to parse documents");

    println!("Found {} documents\n", documents.len());

    // Process each document based on its 'kind' field
    let mut resources: Vec<Resource> = Vec::new();

    for (index, document) in documents.iter().enumerate() {
        // Get the discriminator field
        let kind = get_discriminator_field(document, "kind");

        println!("Document {}: kind = {:?}", index + 1, kind);

        // Deserialize based on the kind
        let resource = match kind.as_deref() {
            Some("Application") => match deserialize_document::<Application>(document) {
                Ok(app) => {
                    println!("  ✓ Deserialized as Application: {:?}", app);
                    Resource::Application(app)
                }
                Err(e) => {
                    println!("  ✗ Failed to deserialize: {:?}", e);
                    Resource::Unknown(kind.unwrap_or_default())
                }
            },
            Some("Service") => match deserialize_document::<Service>(document) {
                Ok(svc) => {
                    println!("  ✓ Deserialized as Service: {:?}", svc);
                    Resource::Service(svc)
                }
                Err(e) => {
                    println!("  ✗ Failed to deserialize: {:?}", e);
                    Resource::Unknown(kind.unwrap_or_default())
                }
            },
            Some("ConfigMap") => match deserialize_document::<ConfigMap>(document) {
                Ok(cm) => {
                    println!("  ✓ Deserialized as ConfigMap: {:?}", cm);
                    Resource::ConfigMap(cm)
                }
                Err(e) => {
                    println!("  ✗ Failed to deserialize: {:?}", e);
                    Resource::Unknown(kind.unwrap_or_default())
                }
            },
            Some(unknown) => {
                println!("  ⚠ Unknown resource kind: {}", unknown);
                Resource::Unknown(unknown.to_string())
            }
            None => {
                println!("  ⚠ No 'kind' field found");
                Resource::Unknown("unknown".to_string())
            }
        };

        resources.push(resource);
        println!();
    }

    // Summary
    println!("=== Summary ===");
    let applications = resources
        .iter()
        .filter(|r| matches!(r, Resource::Application(_)))
        .count();
    let services = resources
        .iter()
        .filter(|r| matches!(r, Resource::Service(_)))
        .count();
    let config_maps = resources
        .iter()
        .filter(|r| matches!(r, Resource::ConfigMap(_)))
        .count();
    let unknown = resources
        .iter()
        .filter(|r| matches!(r, Resource::Unknown(_)))
        .count();

    println!("Applications: {}", applications);
    println!("Services: {}", services);
    println!("ConfigMaps: {}", config_maps);
    println!("Unknown: {}", unknown);
    println!("Total: {}", resources.len());
}
