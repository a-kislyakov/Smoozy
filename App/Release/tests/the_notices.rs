//! Free-software notices: check the generator without a registry or network.
#[test]
fn wave4_the_notices_generator_checks_a_fixed_metadata_sample() {
    let result = std::process::Command::new(if cfg!(windows) { "python" } else { "python3" })
        .arg(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("notices.py"))
        .arg("--self-test")
        .status()
        .expect("Python 3 is needed by the notices generator");
    assert!(result.success(), "the notices sample checks failed");
}
