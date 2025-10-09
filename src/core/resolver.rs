//! Schema resolution via canonical-manager
//!
//! Provides type lookups and dependency resolution using the canonical manager

use crate::core::{Error, Result};
use octofhir_canonical_manager::CanonicalManager;
use octofhir_canonical_manager::resolver::ResolvedResource;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, warn};

/// Schema resolver using canonical-manager
pub struct SchemaResolver {
    /// Canonical manager instance
    manager: Arc<CanonicalManager>,

    /// Cache of resolved StructureDefinitions
    cache: Arc<RwLock<HashMap<String, Value>>>,

    /// Cache of primitive type checks
    primitives: Arc<RwLock<HashMap<String, bool>>>,
}

impl SchemaResolver {
    /// Create a new schema resolver
    ///
    /// # Arguments
    ///
    /// * `manager` - Canonical manager instance
    pub fn new(manager: Arc<CanonicalManager>) -> Self {
        Self {
            manager,
            cache: Arc::new(RwLock::new(HashMap::new())),
            primitives: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Resolve a type by canonical URL
    ///
    /// # Arguments
    ///
    /// * `canonical_url` - Canonical URL of the type to resolve
    ///
    /// # Returns
    ///
    /// StructureDefinition JSON if found
    pub async fn resolve_type(&self, canonical_url: &str) -> Result<Option<Value>> {
        // Check cache first
        {
            let cache = self.cache.read().await;
            if let Some(cached) = cache.get(canonical_url) {
                debug!("Cache hit for {}", canonical_url);
                return Ok(Some(cached.clone()));
            }
        }

        // Resolve via canonical manager
        debug!("Resolving {} via canonical-manager", canonical_url);

        match self.manager.resolve(canonical_url).await {
            Ok(resolved) => {
                let content = resolved.resource.content.clone();

                // Cache the result
                {
                    let mut cache = self.cache.write().await;
                    cache.insert(canonical_url.to_string(), content.clone());
                }

                Ok(Some(content))
            }
            Err(e) => {
                warn!("Failed to resolve {}: {}", canonical_url, e);
                Ok(None)
            }
        }
    }

    /// Resolve base type for a StructureDefinition
    ///
    /// # Arguments
    ///
    /// * `base_url` - Canonical URL of the base type
    ///
    /// # Returns
    ///
    /// Base type StructureDefinition if found
    pub async fn resolve_base_type(&self, base_url: &str) -> Result<Option<Value>> {
        self.resolve_type(base_url).await
    }

    /// Check if a type is a FHIR primitive
    ///
    /// Uses canonical manager to look up the type and check its kind
    pub async fn is_primitive_type(&self, type_code: &str) -> Result<bool> {
        // Check cache first
        {
            let cache = self.primitives.read().await;
            if let Some(&is_prim) = cache.get(type_code) {
                return Ok(is_prim);
            }
        }

        // Build canonical URL for core FHIR types
        let canonical_url = format!("http://hl7.org/fhir/StructureDefinition/{}", type_code);

        if let Some(sd) = self.resolve_type(&canonical_url).await? {
            let kind = sd.get("kind").and_then(|v| v.as_str()).unwrap_or("complex-type");

            let is_prim = kind == "primitive-type";

            // Cache the result
            {
                let mut cache = self.primitives.write().await;
                cache.insert(type_code.to_string(), is_prim);
            }

            Ok(is_prim)
        } else {
            // If not found, assume it's a complex type
            Ok(false)
        }
    }

    /// Resolve a value set by canonical URL
    ///
    /// # Arguments
    ///
    /// * `value_set_url` - Canonical URL of the value set
    ///
    /// # Returns
    ///
    /// ValueSet resource if found
    pub async fn resolve_value_set(&self, value_set_url: &str) -> Result<Option<Value>> {
        debug!("Resolving value set: {}", value_set_url);

        match self.manager.resolve(value_set_url).await {
            Ok(resolved) => Ok(Some(resolved.resource.content)),
            Err(e) => {
                warn!("Failed to resolve value set {}: {}", value_set_url, e);
                Ok(None)
            }
        }
    }

    /// Get all resources of a specific type
    ///
    /// # Arguments
    ///
    /// * `resource_type` - FHIR resource type (e.g., "StructureDefinition")
    /// * `limit` - Maximum number of resources to return (defaults to 1000)
    ///
    /// # Returns
    ///
    /// List of matching resources
    pub async fn get_resources_by_type(
        &self,
        resource_type: &str,
        limit: Option<usize>,
    ) -> Result<Vec<ResolvedResource>> {
        let limit = limit.unwrap_or(1000);
        debug!("Searching for resources of type: {} (limit: {})", resource_type, limit);

        let query = self
            .manager
            .search()
            .await
            .resource_type(resource_type)
            .limit(limit)
            .execute()
            .await
            .map_err(|e| Error::CanonicalManager(e.to_string()))?;

        // Convert search results to resolved resources
        let mut resources = Vec::new();
        for result in query.resources {
            // Resolve each resource to get full content
            if let Some(url) = &result.resource.url
                && let Ok(resolved) = self.manager.resolve(url).await
            {
                resources.push(resolved);
            }
        }

        Ok(resources)
    }

    /// Get all StructureDefinitions for a FHIR version
    ///
    /// # Arguments
    ///
    /// * `fhir_version` - FHIR version (e.g., "R4", "R5")
    ///
    /// # Returns
    ///
    /// List of StructureDefinition resources
    pub async fn get_all_structure_definitions(&self, _fhir_version: &str) -> Result<Vec<Value>> {
        debug!("Loading all StructureDefinitions");

        // Use a high limit to get all StructureDefinitions (typically 145+ for R4)
        let resources = self.get_resources_by_type("StructureDefinition", Some(1000)).await?;

        Ok(resources.into_iter().map(|r| r.resource.content).collect())
    }

    /// Clear all caches
    pub async fn clear_cache(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();

        let mut prim_cache = self.primitives.write().await;
        prim_cache.clear();

        debug!("Schema resolver cache cleared");
    }

    /// Get cache statistics
    pub async fn cache_stats(&self) -> CacheStats {
        let cache = self.cache.read().await;
        let primitives = self.primitives.read().await;

        CacheStats { type_cache_size: cache.len(), primitive_cache_size: primitives.len() }
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    /// Number of cached types
    pub type_cache_size: usize,
    /// Number of cached primitive type checks
    pub primitive_cache_size: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use octofhir_canonical_manager::FcmConfig;
    use tempfile::TempDir;

    async fn create_test_manager() -> Arc<CanonicalManager> {
        let temp_dir = TempDir::new().unwrap();
        let mut config = FcmConfig::default();
        config.storage.packages_dir = temp_dir.path().join("packages");
        config.storage.cache_dir = temp_dir.path().join("cache");
        config.storage.index_dir = temp_dir.path().join("index");
        Arc::new(CanonicalManager::new(config).await.unwrap())
    }

    #[tokio::test]
    async fn test_resolver_creation() {
        let manager = create_test_manager().await;
        let resolver = SchemaResolver::new(manager);

        let stats = resolver.cache_stats().await;
        assert_eq!(stats.type_cache_size, 0);
        assert_eq!(stats.primitive_cache_size, 0);
    }

    #[tokio::test]
    async fn test_cache_clear() {
        let manager = create_test_manager().await;
        let resolver = SchemaResolver::new(manager);

        resolver.clear_cache().await;

        let stats = resolver.cache_stats().await;
        assert_eq!(stats.type_cache_size, 0);
    }

    #[tokio::test]
    async fn test_resolve_nonexistent_type() {
        let manager = create_test_manager().await;
        let resolver = SchemaResolver::new(manager);

        let result = resolver.resolve_type("http://example.com/nonexistent").await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_cache_stats() {
        let manager = create_test_manager().await;
        let resolver = SchemaResolver::new(manager);

        let stats = resolver.cache_stats().await;
        assert_eq!(stats.type_cache_size, 0);
        assert_eq!(stats.primitive_cache_size, 0);
    }
}
