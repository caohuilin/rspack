use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::PathBuf;
use std::sync::Arc;

use rspack_fs::ReadableFileSystem;
use rspack_paths::AssertUtf8;

fn hash_single_file(fs: &Arc<dyn ReadableFileSystem>, path: &PathBuf, hasher: &mut DefaultHasher) {
  let path = path.clone().assert_utf8();
  let bytes = fs
    .read(&path)
    .unwrap_or_else(|_| panic!("Failed to read buildDependency({path}) content."));
  // Hash both the path and content to ensure changes in file location also trigger cache invalidation
  path.hash(hasher);
  bytes.hash(hasher);
}

fn hash_directory(
  fs: &Arc<dyn ReadableFileSystem>,
  dir_path: &PathBuf,
  hasher: &mut DefaultHasher,
) {
  let dir_path = dir_path.clone().assert_utf8();
  let entries = fs
    .read_dir(&dir_path)
    .unwrap_or_else(|_| panic!("Failed to read directory: {dir_path}"));

  // Sort entries to ensure consistent hashing
  let mut entries = entries;
  entries.sort(); // Sort the strings directly

  for entry_name in entries {
    let path = dir_path.join(entry_name); // Combine the directory path and file name to form the complete path
    let metadata = fs
      .metadata(&path)
      .unwrap_or_else(|_| panic!("Failed to get metadata for {path}"));

    if metadata.is_file {
      hash_single_file(fs, &PathBuf::from(&path), hasher);
    } else if metadata.is_directory {
      hash_directory(fs, &PathBuf::from(&path), hasher);
    }
  }
}

pub fn get_version(
  fs: Arc<dyn ReadableFileSystem>,
  dependencies: &Vec<PathBuf>,
  add_salt: impl FnOnce(&mut DefaultHasher),
) -> String {
  let mut hasher = DefaultHasher::new();

  for dep in dependencies {
    let path = dep.clone().assert_utf8();
    let meta = fs
      .metadata(&path)
      .unwrap_or_else(|_| panic!("Failed to get buildDependency({path}) metadata info."));

    if meta.is_file {
      hash_single_file(&fs, dep, &mut hasher);
    } else if meta.is_directory {
      hash_directory(&fs, dep, &mut hasher);
    } else {
      panic!("buildDependency({path}) is neither a file nor a directory.");
    }
  }

  add_salt(&mut hasher);
  hex::encode(hasher.finish().to_ne_bytes())
}
