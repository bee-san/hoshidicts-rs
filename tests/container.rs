use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use hoshidicts::{Query, Term, import, pack, verify};

fn workdir(name: &str) -> PathBuf {
    let dir = env::temp_dir().join(format!("hoshidicts-rs-container-{name}"));
    fs::remove_dir_all(&dir).ok();
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn describe(path: &Path) -> String {
    let mut query = Query::new();
    query.add_term_dict(path).unwrap();
    query.add_freq_dict(path).unwrap();
    query.add_pitch_dict(path).unwrap();
    query.add_kanji_dict(path).unwrap();

    let mut out = String::new();
    for expression in ["食べる", "走る", "猫", "ない"] {
        let results = query.run(expression).unwrap();
        out += &format!("query {expression} {}\n", results.terms().len());
        for term in results.terms() {
            out += &describe_term(term);
        }
    }

    for character in ["食", "猫"] {
        let results = query.run_kanji(character).unwrap();
        out += &format!("kanji {character} {}\n", results.entries().len());
        for entry in results.entries() {
            out += &format!(
                "  {} {} {} {}\n",
                entry.dict_name(),
                entry.onyomi(),
                entry.kunyomi(),
                entry.tags()
            );
            for definition in entry.definitions() {
                out += &format!("    definition {definition}\n");
            }
            let mut stats: Vec<String> = entry
                .stats()
                .iter()
                .map(|stat| format!("    stat {} {}\n", stat.key(), stat.value()))
                .collect();
            stats.sort();
            out += &stats.concat();
        }
    }

    for style in query.styles().unwrap().styles() {
        out += &format!("styles {} {}\n", style.dict_name(), style.styles());
        for file in ["media/sample.txt", "missing.txt"] {
            let media = query
                .media_file(style.dict_name(), file)
                .unwrap_or_default();
            out += &format!("media {file} {}\n", String::from_utf8_lossy(media));
        }
    }

    out
}

fn describe_term(term: &Term) -> String {
    let mut out = format!(
        "  term {} {} {} {}\n",
        term.expression(),
        term.reading(),
        term.rules(),
        term.score()
    );
    for glossary in term.glossaries() {
        out += &format!(
            "    glossary {} {} {} {}\n",
            glossary.dict_name(),
            glossary.glossary(),
            glossary.definition_tags(),
            glossary.term_tags()
        );
    }
    for entry in term.frequencies() {
        out += &format!("    frequency {}\n", entry.dict_name());
        for frequency in entry.frequencies() {
            out += &format!(
                "      {} {}\n",
                frequency.value(),
                frequency.display_value()
            );
        }
    }
    for entry in term.pitches() {
        out += &format!("    pitch {}\n", entry.dict_name());
        for pitch in entry.pitches() {
            out += &format!(
                "      {} {} {:?} {:?}\n",
                pitch.position(),
                pitch.pattern(),
                pitch.nasal(),
                pitch.devoice()
            );
        }
        for transcription in entry.transcriptions() {
            out += &format!("      ipa {transcription}\n");
        }
    }
    out
}

#[test]
fn container_matches_the_directory_it_was_packed_from() {
    let workdir = workdir("parity");
    let fixture =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("hoshidicts/tests/fixtures/dictionary.zip");

    let imported = import(&fixture, &workdir, false).unwrap();
    let directory = workdir.join(&imported.title);
    let container = workdir.join("dictionary.hoshi");

    let bytes = pack(&directory, &container).unwrap();
    assert_eq!(bytes, fs::metadata(&container).unwrap().len());
    assert_eq!(verify(&container).unwrap(), 3);

    let from_directory = describe(&directory);
    let from_container = describe(&container);
    assert!(from_directory.contains("    glossary "));
    assert!(from_directory.contains("media media/sample.txt generated"));
    assert_eq!(from_directory, from_container);
}

#[test]
fn verify_rejects_a_corrupt_container() {
    let workdir = workdir("corrupt");
    let fixture =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("hoshidicts/tests/fixtures/dictionary.zip");

    let imported = import(&fixture, &workdir, false).unwrap();
    let container = workdir.join("dictionary.hoshi");
    pack(workdir.join(&imported.title), &container).unwrap();

    let mut bytes = fs::read(&container).unwrap();
    let last = bytes.len() - 1;
    bytes[last] ^= 0xff;
    fs::write(&container, &bytes).unwrap();

    assert!(verify(&container).is_err());
}

#[test]
fn pack_rejects_a_directory_that_is_not_a_dictionary() {
    let workdir = workdir("empty");

    assert!(pack(&workdir, workdir.join("out.hoshi")).is_err());
}
