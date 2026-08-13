use hoshidicts::{Deinflector, Lookup, Query};

#[test]
fn links_and_runs_with_no_dictionaries() {
    let query = Query::new();
    let deinflector = Deinflector::new();
    let lookup = Lookup::new(&query, &deinflector);
    let results = lookup
        .run("蜂が好きです", 32, 16)
        .expect("lookup should succeed");
    assert_eq!(results.results().len(), 0);
    assert_eq!(query.styles().expect("styles").styles().len(), 0);
    assert!(query.media_file("nope", "nope.png").is_none());
    assert_eq!(query.run("蜂").expect("term query").terms().len(), 0);
    assert_eq!(
        query.run_kanji("蜂").expect("kanji query").entries().len(),
        0
    );
}

#[test]
fn import_rejects_a_missing_archive() {
    let err = hoshidicts::import("/tmp/definitely-missing.zip", "/tmp/hd-out", false);
    assert!(
        err.is_err(),
        "expected an error, got {:?}",
        err.map(|i| i.title)
    );
}
