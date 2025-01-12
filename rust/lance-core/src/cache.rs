// Copyright 2023 Lance Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Cache implementation

use std::any::{Any, TypeId};
use std::sync::Arc;

use moka::sync::Cache;
use object_store::path::Path;

pub const DEFAULT_INDEX_CACHE_SIZE: usize = 128;
pub const DEFAULT_METADATA_CACHE_SIZE: usize = 128;

type ArcAny = Arc<dyn Any + Send + Sync>;

/// Cache for various metadata about files.
///
/// The cache is keyed by the file path and the type of metadata.
#[derive(Clone)]
pub struct FileMetadataCache {
    cache: Arc<Cache<(Path, TypeId), ArcAny>>,
}

impl FileMetadataCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            cache: Arc::new(Cache::new(capacity as u64)),
        }
    }

    pub fn get<T: Send + Sync + 'static>(&self, path: &Path) -> Option<Arc<T>> {
        self.cache
            .get(&(path.to_owned(), TypeId::of::<T>()))
            .map(|metadata| metadata.clone().downcast::<T>().unwrap())
    }

    pub fn insert<T: Send + Sync + 'static>(&self, path: Path, metadata: Arc<T>) {
        self.cache.insert((path, TypeId::of::<T>()), metadata);
    }
}
