use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn prime_sieve() -> Result<(), Box<dyn std::error::Error>> {
	let mut cmd = Command::cargo_bin("rocklang")?;

	cmd.arg("examples/sieve.rc");
	cmd.assert()
		.success()
		.stdout(predicate::eq("2.000000\n3.000000\n5.000000\n7.000000\n").normalize());

	Ok(())
}
