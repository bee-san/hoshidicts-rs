use std::ptr::NonNull;

use crate::ffi;

pub(crate) fn slice<'a, T>(ptr: *const T, len: usize) -> &'a [T] {
    if len == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(ptr, len) }
    }
}

impl ffi::hd_str {
    fn as_str(&self) -> &str {
        let bytes = slice(self.ptr.cast(), self.len);
        match std::str::from_utf8(bytes) {
            Ok(text) => text,
            Err(error) => std::str::from_utf8(&bytes[..error.valid_up_to()]).unwrap_or_default(),
        }
    }
}

#[repr(transparent)]
pub struct Glossary(ffi::hd_glossary_entry);

impl Glossary {
    pub fn dict_name(&self) -> &str {
        self.0.dict_name.as_str()
    }

    pub fn glossary(&self) -> &str {
        self.0.glossary.as_str()
    }

    pub fn definition_tags(&self) -> &str {
        self.0.definition_tags.as_str()
    }

    pub fn term_tags(&self) -> &str {
        self.0.term_tags.as_str()
    }
}

#[repr(transparent)]
pub struct Frequency(ffi::hd_frequency);

impl Frequency {
    pub fn value(&self) -> i32 {
        self.0.value
    }

    pub fn display_value(&self) -> &str {
        self.0.display_value.as_str()
    }
}

#[repr(transparent)]
pub struct FrequencyEntry(ffi::hd_frequency_entry);

impl FrequencyEntry {
    pub fn dict_name(&self) -> &str {
        self.0.dict_name.as_str()
    }

    pub fn frequencies(&self) -> &[Frequency] {
        slice(self.0.frequencies.cast(), self.0.frequencies_count)
    }
}

#[repr(transparent)]
pub struct Pitch(ffi::hd_pitch);

impl Pitch {
    pub fn position(&self) -> i32 {
        self.0.position
    }

    pub fn pattern(&self) -> &str {
        self.0.pattern.as_str()
    }

    pub fn nasal(&self) -> &[i32] {
        slice(self.0.nasal, self.0.nasal_count)
    }

    pub fn devoice(&self) -> &[i32] {
        slice(self.0.devoice, self.0.devoice_count)
    }
}

#[repr(transparent)]
pub struct PitchEntry(ffi::hd_pitch_entry);

impl PitchEntry {
    pub fn dict_name(&self) -> &str {
        self.0.dict_name.as_str()
    }

    pub fn pitches(&self) -> &[Pitch] {
        slice(self.0.pitches.cast(), self.0.pitches_count)
    }

    pub fn transcriptions(&self) -> impl ExactSizeIterator<Item = &str> {
        slice(self.0.transcriptions, self.0.transcriptions_count)
            .iter()
            .map(|s| s.as_str())
    }
}

#[repr(transparent)]
pub struct Term(ffi::hd_term_result);

impl Term {
    pub fn expression(&self) -> &str {
        self.0.expression.as_str()
    }

    pub fn reading(&self) -> &str {
        self.0.reading.as_str()
    }

    pub fn rules(&self) -> &str {
        self.0.rules.as_str()
    }

    pub fn score(&self) -> i32 {
        self.0.score
    }

    pub fn glossaries(&self) -> &[Glossary] {
        slice(self.0.glossaries.cast(), self.0.glossaries_count)
    }

    pub fn frequencies(&self) -> &[FrequencyEntry] {
        slice(self.0.frequencies.cast(), self.0.frequencies_count)
    }

    pub fn pitches(&self) -> &[PitchEntry] {
        slice(self.0.pitches.cast(), self.0.pitches_count)
    }
}

#[repr(transparent)]
pub struct KanjiStat(ffi::hd_kanji_stat);

impl KanjiStat {
    pub fn key(&self) -> &str {
        self.0.key.as_str()
    }

    pub fn value(&self) -> &str {
        self.0.value.as_str()
    }
}

#[repr(transparent)]
pub struct KanjiEntry(ffi::hd_kanji_entry);

impl KanjiEntry {
    pub fn dict_name(&self) -> &str {
        self.0.dict_name.as_str()
    }

    pub fn onyomi(&self) -> &str {
        self.0.onyomi.as_str()
    }

    pub fn kunyomi(&self) -> &str {
        self.0.kunyomi.as_str()
    }

    pub fn tags(&self) -> &str {
        self.0.tags.as_str()
    }

    pub fn definitions(&self) -> impl ExactSizeIterator<Item = &str> {
        slice(self.0.definitions, self.0.definitions_count)
            .iter()
            .map(|s| s.as_str())
    }

    pub fn stats(&self) -> &[KanjiStat] {
        slice(self.0.stats.cast(), self.0.stats_count)
    }
}

#[repr(transparent)]
pub struct DictionaryStyle(ffi::hd_dictionary_style);

impl DictionaryStyle {
    pub fn dict_name(&self) -> &str {
        self.0.dict_name.as_str()
    }

    pub fn styles(&self) -> &str {
        self.0.styles.as_str()
    }
}

#[repr(transparent)]
pub struct TransformGroup(ffi::hd_transform_group);

impl TransformGroup {
    pub fn name(&self) -> &str {
        self.0.name.as_str()
    }

    pub fn description(&self) -> &str {
        self.0.description.as_str()
    }
}

#[repr(transparent)]
pub struct LookupResult(ffi::hd_lookup_result);

impl LookupResult {
    pub fn matched(&self) -> &str {
        self.0.matched.as_str()
    }

    pub fn deinflected(&self) -> &str {
        self.0.deinflected.as_str()
    }

    pub fn trace(&self) -> &[TransformGroup] {
        slice(self.0.trace.cast(), self.0.trace_count)
    }

    pub fn term(&self) -> &Term {
        unsafe { &*(&raw const self.0.term).cast() }
    }

    pub fn preprocessor_steps(&self) -> i32 {
        self.0.preprocessor_steps
    }
}

pub struct Results {
    pub(crate) ptr: NonNull<ffi::hd_results>,
    pub(crate) terms: *const ffi::hd_term_result,
    pub(crate) count: usize,
}

impl Results {
    pub fn terms(&self) -> &[Term] {
        slice(self.terms.cast(), self.count)
    }
}

impl Drop for Results {
    fn drop(&mut self) {
        unsafe { ffi::hd_results_free(self.ptr.as_ptr()) }
    }
}

pub struct KanjiResults {
    pub(crate) ptr: NonNull<ffi::hd_kanji_results>,
    pub(crate) entries: *const ffi::hd_kanji_entry,
    pub(crate) count: usize,
}

impl KanjiResults {
    pub fn entries(&self) -> &[KanjiEntry] {
        slice(self.entries.cast(), self.count)
    }
}

impl Drop for KanjiResults {
    fn drop(&mut self) {
        unsafe { ffi::hd_kanji_results_free(self.ptr.as_ptr()) }
    }
}

pub struct Styles {
    pub(crate) ptr: NonNull<ffi::hd_styles>,
    pub(crate) styles: *const ffi::hd_dictionary_style,
    pub(crate) count: usize,
}

impl Styles {
    pub fn styles(&self) -> &[DictionaryStyle] {
        slice(self.styles.cast(), self.count)
    }
}

impl Drop for Styles {
    fn drop(&mut self) {
        unsafe { ffi::hd_styles_free(self.ptr.as_ptr()) }
    }
}

pub struct LookupResults {
    pub(crate) ptr: NonNull<ffi::hd_lookup_results>,
    pub(crate) results: *const ffi::hd_lookup_result,
    pub(crate) count: usize,
}

impl LookupResults {
    pub fn results(&self) -> &[LookupResult] {
        slice(self.results.cast(), self.count)
    }
}

impl Drop for LookupResults {
    fn drop(&mut self) {
        unsafe { ffi::hd_lookup_results_free(self.ptr.as_ptr()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn borrowed(bytes: &[u8]) -> ffi::hd_str {
        ffi::hd_str {
            ptr: bytes.as_ptr().cast(),
            len: bytes.len(),
        }
    }

    #[test]
    fn valid_utf8_is_returned_whole() {
        assert_eq!(borrowed("蜂が好き".as_bytes()).as_str(), "蜂が好き");
    }

    #[test]
    fn invalid_utf8_truncates_instead_of_panicking() {
        let bytes = [0xe8, 0x9c, 0x82, 0xe3, 0x81];
        assert_eq!(borrowed(&bytes).as_str(), "蜂");
    }

    #[test]
    fn a_leading_invalid_byte_yields_an_empty_string() {
        assert_eq!(borrowed(&[0xff, 0xfe]).as_str(), "");
    }

    #[test]
    fn an_empty_string_does_not_dereference_the_pointer() {
        let empty = ffi::hd_str {
            ptr: std::ptr::null(),
            len: 0,
        };
        assert_eq!(empty.as_str(), "");
    }
}
