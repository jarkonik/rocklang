use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
#[cfg_attr(tarpaulin, ignore)]
fn prime_sieve() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rocklang")?;

    cmd.arg("examples/sieve.rck");
    cmd.assert()
        .success()
        .stdout(predicate::eq("2.000000\n3.000000\n5.000000\n7.000000\n").normalize());

    Ok(())
}

#[test]
#[cfg_attr(tarpaulin, ignore)]
fn mandelbrot() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rocklang")?;

    cmd.arg("examples/mandelbrot.rck");
    cmd.assert().success().stdout(
        predicate::eq(
            "...............*..............
...............*..............
...............*..............
..............***.............
.............*****............
.............*****............
.............*****............
..............***.............
.............*****............
...........*********..........
.........*************........
..........***********.........
.........*************........
.......*****************......
.......*****************......
.....*...*************...*....
.........*************........
..........***********.........
..........*****.*****.........
..............................
..............................
..............................
..............................
..............................
..............................
..............................
..............................
..............................
..............................
..............................
",
        )
        .normalize(),
    );

    Ok(())
}

#[test]
#[cfg_attr(tarpaulin, ignore)]
fn ffi() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rocklang")?;

    cmd.arg("examples/ffi.rck");
    cmd.assert()
        .success()
        .stdout(predicate::eq("2 + 3 is 5.000000\n").normalize());

    Ok(())
}
