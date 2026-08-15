use hoshidicts::{Deinflector, Lookup, LookupFrequencyOrder, LookupOptions, OwnedLookup, Query};

fn lookup_owner() -> (Query, Deinflector) {
    (Query::new(), Deinflector::new())
}

#[test]
fn frequency_order_defaults_to_auto() {
    assert_eq!(LookupFrequencyOrder::default(), LookupFrequencyOrder::Auto);
}

#[test]
fn options_default_is_empty_and_auto() {
    let options = LookupOptions::default();
    assert_eq!(options.frequency_dictionary, None);
    assert_eq!(options.frequency_order, LookupFrequencyOrder::Auto);
    assert_eq!(options.primary_reading, None);
}

#[test]
fn run_with_default_options_matches_run() {
    let (query, deinflector) = lookup_owner();
    let lookup = Lookup::new(&query, &deinflector);

    let plain = lookup.run("蜂が好きです", 32, 16).expect("run");
    let with_options = lookup
        .run_with_options("蜂が好きです", 32, 16, &LookupOptions::default())
        .expect("run_with_options");

    assert_eq!(plain.results().len(), with_options.results().len());
}

#[test]
fn every_frequency_order_is_accepted() {
    let (query, deinflector) = lookup_owner();
    let lookup = Lookup::new(&query, &deinflector);

    for order in [
        LookupFrequencyOrder::Auto,
        LookupFrequencyOrder::Ascending,
        LookupFrequencyOrder::Descending,
        LookupFrequencyOrder::Disabled,
    ] {
        let options = LookupOptions {
            frequency_order: order,
            ..Default::default()
        };
        let results = lookup
            .run_with_options("蜂が好きです", 32, 16, &options)
            .expect("run_with_options should accept every frequency order");
        assert_eq!(results.results().len(), 0);
    }
}

#[test]
fn frequency_dictionary_and_primary_reading_are_borrowed() {
    let (query, deinflector) = lookup_owner();
    let lookup = Lookup::new(&query, &deinflector);

    let freq = String::from("Some Frequency Dict");
    let reading = String::from("はち");
    let options = LookupOptions {
        frequency_dictionary: Some(freq.as_str()),
        frequency_order: LookupFrequencyOrder::Descending,
        primary_reading: Some(reading.as_str()),
    };

    let results = lookup
        .run_with_options("蜂", 8, 16, &options)
        .expect("run_with_options with borrowed strings");
    assert_eq!(results.results().len(), 0);
}

#[test]
fn owned_lookup_supports_options() {
    let owned = OwnedLookup::new(Query::new(), Deinflector::new());
    let options = LookupOptions {
        frequency_order: LookupFrequencyOrder::Disabled,
        ..Default::default()
    };
    let results = owned
        .run_with_options("蜂が好きです", 32, 16, &options)
        .expect("owned run_with_options");
    assert_eq!(results.results().len(), 0);
}

#[test]
fn options_reject_interior_nul_lookup_string() {
    let (query, deinflector) = lookup_owner();
    let lookup = Lookup::new(&query, &deinflector);
    let err = lookup.run_with_options("bad\0string", 8, 16, &LookupOptions::default());
    assert!(
        err.is_err(),
        "interior NUL in lookup string must be rejected"
    );
}
