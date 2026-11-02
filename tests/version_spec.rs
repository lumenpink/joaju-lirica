#[test]
fn test_rust_version_constant() {
    assert!(
        !joaju_lirica::VERSION.is_empty(),
        "VERSION const cannot be empty"
    );
    assert!(
        joaju_lirica::VERSION.starts_with("0.") || joaju_lirica::VERSION.starts_with("1."),
        "VERSION should start with SemVer major digit"
    );
}

#[test]
fn test_ci_workflow_kombinat_notification_and_no_codeberg() {
    let ci_yaml = std::fs::read_to_string(".forgejo/workflows/ci.yml").expect("ci.yml must exist");
    assert!(
        ci_yaml.contains("http://127.0.0.1:4040/api/cascade/dispatch"),
        "CI workflow must notify Kombinat cascade daemon on release"
    );
    assert!(
        !ci_yaml.contains("codeberg.org/api/v1/repos"),
        "CI workflow must not contain legacy Codeberg dispatch"
    );
}
