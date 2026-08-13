use hoshidicts::{Deinflector, Lookup, OwnedLookup, Query};

fn assert_send<T: Send>() {}

#[test]
fn handles_are_send() {
    assert_send::<Query>();
    assert_send::<Deinflector>();
    assert_send::<Lookup<'static>>();
    assert_send::<OwnedLookup>();
}

#[test]
fn an_owned_lookup_can_be_moved_to_another_thread() {
    let lookup = OwnedLookup::new(Query::new(), Deinflector::new());

    let handle = std::thread::spawn(move || {
        let results = lookup
            .run("蜂が好きです", 32, 16)
            .expect("lookup should succeed");
        let styles = lookup.query().styles().expect("styles").styles().len();
        (results.results().len(), styles)
    });

    assert_eq!(handle.join().unwrap(), (0, 0));
}
