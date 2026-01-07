use std::collections::BTreeMap;
use yaml::prelude::*;

#[derive(Serialize, Deserialize, Default)]
struct TestStruct {
    /// Name field
    pub name: String,

    /// Empty vec field
    pub empty_vec: Vec<String>,

    /// Non-empty vec field  
    pub non_empty_vec: Vec<String>,

    /// Empty map field (without Option - should be skipped)
    pub empty_map: BTreeMap<String, String>,

    /// Non-empty map field (without Option)
    pub non_empty_map: BTreeMap<String, String>,

    /// Empty map with Option (None - should be skipped)
    pub optional_empty_map: Option<BTreeMap<String, String>>,

    /// Non-empty map with Option
    pub optional_non_empty_map: Option<BTreeMap<String, String>>,
}

fn main() {
    let mut map1 = BTreeMap::new();
    map1.insert("key1".to_string(), "value1".to_string());

    let mut map2 = BTreeMap::new();
    map2.insert("key2".to_string(), "value2".to_string());

    let test = TestStruct {
        name: "test".to_string(),
        empty_vec: vec![],
        non_empty_vec: vec!["item1".to_string()],
        empty_map: BTreeMap::new(),
        non_empty_map: map1,
        optional_empty_map: Some(BTreeMap::new()),
        optional_non_empty_map: Some(map2),
    };

    let yaml = Yaml::serialize_to_string(test).expect("Failed");
    println!("=== Testing Empty Collection Skipping ===\n");
    println!("{}", yaml);
    println!("\n=== Expected Behavior ===");
    println!("✓ empty_vec should be SKIPPED");
    println!("✓ empty_map should be SKIPPED");
    println!("✓ optional_empty_map with Some(empty) should appear");
    println!("✓ non_empty_vec should appear");
    println!("✓ non_empty_map should appear");
    println!("✓ optional_non_empty_map should appear");
}
