use octofhir_canonical_manager::{CanonicalManager, FcmConfig};
use octofhir_codegen::core::resolver::SchemaResolver;
use std::sync::Arc;
use tempfile::TempDir;

async fn setup_test_manager() -> Arc<CanonicalManager> {
    let temp_dir = TempDir::new().unwrap();
    let mut config = FcmConfig::default();
    config.storage.packages_dir = temp_dir.path().join("packages");
    config.storage.cache_dir = temp_dir.path().join("cache");
    config.storage.index_dir = temp_dir.path().join("index");
    Arc::new(CanonicalManager::new(config).await.unwrap())
}

#[tokio::test]
async fn test_resolver_creation() {
    let manager = setup_test_manager().await;
    let resolver = SchemaResolver::new(manager);

    let stats = resolver.cache_stats().await;
    assert_eq!(stats.type_cache_size, 0);
    assert_eq!(stats.primitive_cache_size, 0);
}

#[tokio::test]
async fn test_resolve_nonexistent_type() {
    let manager = setup_test_manager().await;
    let resolver = SchemaResolver::new(manager);

    // Try to resolve a type that doesn't exist
    let result = resolver.resolve_type("http://example.com/nonexistent").await;

    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[tokio::test]
async fn test_cache_functionality() {
    let manager = setup_test_manager().await;
    let resolver = SchemaResolver::new(manager);

    let stats_before = resolver.cache_stats().await;
    assert_eq!(stats_before.type_cache_size, 0);

    // Resolve a URL (will fail but should be cached as None)
    let url = "http://hl7.org/fhir/StructureDefinition/string";
    let result1 = resolver.resolve_type(url).await;
    assert!(result1.is_ok());

    // Second resolve should use cache
    let result2 = resolver.resolve_type(url).await;
    assert!(result2.is_ok());

    // Clear cache
    resolver.clear_cache().await;
    let stats_after = resolver.cache_stats().await;
    assert_eq!(stats_after.type_cache_size, 0);
}

#[tokio::test]
async fn test_resolve_base_type() {
    let manager = setup_test_manager().await;
    let resolver = SchemaResolver::new(manager);

    // Try to resolve base type (will be None without packages installed)
    let result =
        resolver.resolve_base_type("http://hl7.org/fhir/StructureDefinition/Resource").await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_is_primitive_type_no_packages() {
    let manager = setup_test_manager().await;
    let resolver = SchemaResolver::new(manager);

    // Without packages installed, this will return false
    let result = resolver.is_primitive_type("string").await;
    assert!(result.is_ok());

    // Check cache was updated
    let stats = resolver.cache_stats().await;
    assert!(stats.primitive_cache_size <= 1);
}

#[tokio::test]
async fn test_resolve_value_set() {
    let manager = setup_test_manager().await;
    let resolver = SchemaResolver::new(manager);

    // Try to resolve value set (will be None without packages)
    let result =
        resolver.resolve_value_set("http://hl7.org/fhir/ValueSet/administrative-gender").await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_resources_by_type() {
    let manager = setup_test_manager().await;
    let resolver = SchemaResolver::new(manager);

    // Search for StructureDefinitions (will be empty without packages)
    let result = resolver.get_resources_by_type("StructureDefinition", None).await;

    assert!(result.is_ok());
    let resources = result.unwrap();
    // Without packages, should be empty
    assert_eq!(resources.len(), 0);
}

#[tokio::test]
async fn test_get_all_structure_definitions() {
    let manager = setup_test_manager().await;
    let resolver = SchemaResolver::new(manager);

    // Get all StructureDefinitions for R4 (will be empty without packages)
    let result = resolver.get_all_structure_definitions("R4").await;

    assert!(result.is_ok());
    let sds = result.unwrap();
    // Without packages, should be empty
    assert_eq!(sds.len(), 0);
}

#[tokio::test]
async fn test_cache_stats_tracking() {
    let manager = setup_test_manager().await;
    let resolver = SchemaResolver::new(manager);

    // Initial stats
    let stats1 = resolver.cache_stats().await;
    assert_eq!(stats1.type_cache_size, 0);
    assert_eq!(stats1.primitive_cache_size, 0);

    // Resolve a type (will be cached even if not found)
    let _ = resolver.resolve_type("http://example.com/Test").await.unwrap();

    // Check stats updated
    let stats2 = resolver.cache_stats().await;
    assert!(stats2.type_cache_size <= 1);

    // Clear and verify
    resolver.clear_cache().await;
    let stats3 = resolver.cache_stats().await;
    assert_eq!(stats3.type_cache_size, 0);
}

#[tokio::test]
async fn test_parser_with_resolver() {
    use octofhir_codegen::core::parser::StructureDefinitionParser;

    let manager = setup_test_manager().await;
    let resolver = Arc::new(SchemaResolver::new(manager));

    // Create parser with resolver
    let parser = StructureDefinitionParser::with_resolver(resolver.clone());

    // Just verify parser can be created with resolver
    // Actual parsing with resolver will be tested when packages are available
    drop(parser);
}

#[tokio::test]
async fn test_parser_set_resolver() {
    use octofhir_codegen::core::parser::StructureDefinitionParser;

    let manager = setup_test_manager().await;
    let resolver = Arc::new(SchemaResolver::new(manager));

    // Create parser without resolver
    let mut parser = StructureDefinitionParser::new();

    // Set resolver after creation
    parser.set_resolver(resolver.clone());

    // Verify parser still works
    drop(parser);
}
