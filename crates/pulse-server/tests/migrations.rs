use pulse_server::MIGRATOR;

#[test]
fn campuses_is_the_only_migration() {
    let descriptions: Vec<_> = MIGRATOR
        .iter()
        .map(|migration| migration.description.to_string())
        .collect();
    assert_eq!(descriptions, vec!["create campuses".to_owned()]);
}
