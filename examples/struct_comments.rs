use yaml::prelude::*;

/// Example configuration for a web server
/// This demonstrates struct-level doc comments appearing at the top of YAML output
#[derive(Serialize, Deserialize, Default)]
struct ServerConfig {
    /// The hostname or IP address to bind to
    pub host: String,

    /// The port number to listen on
    pub port: String,

    /// Optional SSL certificate path
    pub ssl_cert: Option<String>,
}

fn main() {
    let config = ServerConfig {
        host: "localhost".to_string(),
        port: "8080".to_string(),
        ssl_cert: None,
    };

    let yaml = Yaml::serialize_to_string(config).expect("Failed to serialize");

    println!("{}", yaml);
}
