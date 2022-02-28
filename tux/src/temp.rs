use std::path::{Path, PathBuf};

use path_clean::PathClean;

/// Provides a unique temporary directory that can be used to setup data files
/// for tests. The directory and its contents will be deleted when the value
/// is dropped.
pub struct TestTempDir {
	dir: tempfile::TempDir,
}

impl TestTempDir {
	pub fn create_new() -> TestTempDir {
		TestTempDir {
			dir: tempfile::tempdir().expect("creating temp dir for test"),
		}
	}

	pub fn path(&self) -> &Path {
		self.dir.path()
	}

	pub fn create_file<S: AsRef<[u8]>>(&self, name: &str, text: S) -> PathBuf {
		let mut path = self.path().to_owned();
		path.push(name);

		let path = path.clean();
		if !path.starts_with(self.path()) {
			panic!("cannot create test file outside root dir");
		}

		let parent = path.parent().expect("parent dir for new test file");
		std::fs::create_dir_all(parent).expect("creating parent dir for new test file");

		std::fs::write(&path, text).expect("failed to write test file");
		path
	}

	pub fn run_and_get_output(&self, cmd: &str, args: &[&str]) -> String {
		let mut cmd = super::get_exe_command(cmd);
		cmd.args(args);
		cmd.current_dir(self.path());
		super::run_command_and_get_output(&mut cmd)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_temp_dir_should_create_new_directory() {
		let dir = TestTempDir::create_new();
		let path = dir.path();
		assert!(path.is_dir());
	}

	#[test]
	fn test_temp_dir_should_delete_diretory_on_drop() {
		let dir = TestTempDir::create_new();
		let path = dir.path().to_owned();
		drop(dir);
		assert!(!path.exists());
	}

	#[test]
	fn test_temp_dir_path_should_be_absolute() {
		let dir = TestTempDir::create_new();
		let path = dir.path();
		assert!(path.is_absolute());
	}

	#[test]
	fn test_temp_should_create_file_at_root() {
		let dir = TestTempDir::create_new();
		let file_path = dir.create_file("some_file.txt", "some file contents");
		assert!(file_path.is_file());

		let contents = std::fs::read_to_string(file_path).unwrap();
		assert_eq!(contents, "some file contents");
	}

	#[test]
	fn test_temp_should_create_file_with_directories() {
		let dir = TestTempDir::create_new();
		let file_path = dir.create_file("sub/a/b/simple_file.txt", "abc");
		assert!(file_path.is_file());

		let mut sub_dir = dir.path().to_owned();
		sub_dir.push("sub");
		assert!(sub_dir.is_dir());

		let contents = std::fs::read_to_string(file_path).unwrap();
		assert_eq!(contents, "abc");
	}

	#[test]
	fn test_temp_dir_should_delete_diretory_on_drop_even_if_non_empty() {
		let dir = TestTempDir::create_new();
		let path = dir.path().to_owned();
		dir.create_file("root.txt", "text");
		dir.create_file("a/file.txt", "text");
		dir.create_file("b/file.txt", "text");
		dir.create_file("c/sub/file.txt", "text");
		drop(dir);
		assert!(!path.exists());
	}

	#[test]
	fn test_temp_should_not_create_file_outside_root_directory() {
		let dir = TestTempDir::create_new();
		let result = std::panic::catch_unwind(|| {
			dir.create_file(
				"sub/../../test_file.txt",
				"this test file should not be created",
			);
		});
		assert!(result.is_err());
	}
}
