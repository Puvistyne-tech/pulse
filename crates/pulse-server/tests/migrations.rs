use pulse_server::MIGRATOR;

#[test]
fn migrations_apply_in_order() {
    let descriptions: Vec<_> = MIGRATOR
        .iter()
        .map(|migration| migration.description.to_string())
        .collect();
    assert_eq!(
        descriptions,
        vec![
            "create campuses".to_owned(),
            "create accounts memberships rooms".to_owned()
        ]
    );
}
