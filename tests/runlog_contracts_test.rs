use std::process::Command;

#[test]
fn runlog_read_preserves_glob_string_lines() {
    let dir = tempfile::tempdir().expect("temporary source fixture");
    let source = "export default {\n  // a removable comment\n  include: [\n    'tests/**/*.test.ts',\n    \"tests/**/*.spec.ts\",\n    `workspace/**/*.tsx`,\n  ],\n  exclude: ['**/*.d.ts'],\n};\n";
    let file = dir.path().join("vitest.config.ts");
    std::fs::write(&file, source).expect("write source");
    let actual = Command::new(env!("CARGO_BIN_EXE_rtk"))
        .env("RTK_DB_PATH", dir.path().join("tracking.db"))
        .arg("read")
        .arg(file)
        .output()
        .expect("RTK read");
    assert!(actual.status.success());
    let output = String::from_utf8(actual.stdout).expect("UTF-8 output");
    for line in source.lines().filter(|line| line.contains("/*")) {
        assert!(
            output.lines().any(|actual| actual == line),
            "missing source line: {line}"
        );
    }
    assert!(!output.contains("removable comment"));
}
