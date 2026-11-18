use pulse_protocol::{
    CampusId, ClientFrame, ClientSnapshot, ConnectionPhase, Hello, Ping, Pong, ProtocolVersion,
    ServerFrame, SessionId, VersionMismatch, Welcome,
};
use std::path::PathBuf;
use ts_rs::{Config, TS};

fn render_bindings() -> String {
    let config = Config::default();
    let declarations = [
        CampusId::decl(&config),
        SessionId::decl(&config),
        ProtocolVersion::decl(&config),
        Hello::decl(&config),
        Welcome::decl(&config),
        Ping::decl(&config),
        Pong::decl(&config),
        VersionMismatch::decl(&config),
        ClientFrame::decl(&config),
        ServerFrame::decl(&config),
        ConnectionPhase::decl(&config),
        ClientSnapshot::decl(&config),
    ];
    let mut rendered = declarations
        .iter()
        .map(|declaration| format!("export {declaration}"))
        .collect::<Vec<_>>()
        .join("\n");
    rendered.push('\n');
    rendered
}

fn bindings_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("bindings/protocol.ts")
}

#[test]
fn bindings_match_committed_file() {
    let rendered = render_bindings();
    if std::env::var("PULSE_BLESS_BINDINGS").as_deref() == Ok("1") {
        std::fs::write(bindings_path(), &rendered).expect("write bindings");
        return;
    }
    let committed = std::fs::read_to_string(bindings_path()).expect("read bindings");
    assert_eq!(committed, rendered);
}

#[test]
fn bindings_contain_no_comments() {
    let rendered = render_bindings();
    assert!(!rendered.contains("//"));
    assert!(!rendered.contains("/*"));
}
