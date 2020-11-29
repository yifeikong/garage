use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command;  // Run programs

#[test]
fn smoke() -> Result<(), CargoError> {
	let mut cmd = Command::cargo_bin("garage")?;
}
