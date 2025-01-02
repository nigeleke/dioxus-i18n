mod common;
use common::*;

use dioxus_i18n::{
    prelude::{use_init_i18n, I18n, I18nConfig},
    t,
};
use unic_langid::{langid, LanguageIdentifier};

use std::path::PathBuf;

#[test]
fn translate_from_static_source() {
    test_hook(i18n_from_static, |_, proxy| {
        let name = "World";
        proxy.assert(
            &t!("hello", name: name),
            "Hello, \u{2068}World\u{2069}!",
            "translate_from_static_source",
        );
    });
}

#[test]
fn failed_to_translate_with_invalid_key() {
    test_hook(i18n_from_static, |_, proxy| {
        let panic = std::panic::catch_unwind(|| {
            let _ = &t!("invalid");
        });
        proxy.assert(
            &panic.is_err().to_string(),
            "true",
            "failed_to_translate_with_invalid_key",
        );
    });
}

#[test]
fn translate_from_dynamic_source() {
    test_hook(i18n_from_dynamic, |_, proxy| {
        let name = "World";
        proxy.assert(
            &t!("hello", name: name),
            "Hello, \u{2068}World\u{2069}!",
            "translate_from_dynamic_source",
        );
    });
}

#[test]
fn initial_language_is_set() {
    test_hook(i18n_from_static, |value, proxy| {
        proxy.assert(
            &value.language().to_string(),
            &EN.to_string(),
            "initial_language_is_set",
        );
    });
}

#[test]
fn language_can_be_set() {
    test_hook(i18n_from_static, |mut value, proxy| {
        value.set_language(JP);
        proxy.assert(
            &value.language().to_string(),
            &JP.to_string(),
            "language_can_be_set",
        );
    });
}

#[test]
fn no_default_fallback_language() {
    test_hook(i18n_from_static, |value, proxy| {
        proxy.assert(
            &format!("{:?}", value.fallback_language()),
            "None",
            "no_default_fallback_language",
        );
    });
}

#[test]
fn some_default_fallback_language() {
    test_hook(i18n_from_static_with_fallback, |value, proxy| {
        proxy.assert(
            &format!("{:?}", value.fallback_language().map(|l| l.to_string())),
            "Some(\"jp\")",
            "some_default_fallback_language",
        );
    });
}

#[test]
fn fallback_language_can_be_set() {
    test_hook(i18n_from_static_with_fallback, |mut value, proxy| {
        value.set_fallback_language(DE);
        proxy.assert(
            &format!("{:?}", value.fallback_language().map(|l| l.to_string())),
            "Some(\"de\")",
            "fallback_language_can_be_set",
        );
    });
}

const DE: LanguageIdentifier = langid!("de");
const EN: LanguageIdentifier = langid!("en");
const JP: LanguageIdentifier = langid!("jp");

fn i18n_from_static() -> I18n {
    let config = I18nConfig::new(EN).with_locale((EN, include_str!("./data/i18n/en.ftl")));
    use_init_i18n(|| config)
}

fn i18n_from_static_with_fallback() -> I18n {
    let config = I18nConfig::new(EN)
        .with_locale((EN, include_str!("./data/i18n/en.ftl")))
        .with_fallback(JP);
    use_init_i18n(|| config)
}

fn i18n_from_dynamic() -> I18n {
    let config = I18nConfig::new(EN).with_locale((
        EN,
        PathBuf::from(format!(
            "{}/tests/data/i18n/en.ftl",
            env!("CARGO_MANIFEST_DIR")
        )),
    ));
    use_init_i18n(|| config)
}
