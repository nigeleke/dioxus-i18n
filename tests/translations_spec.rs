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

const EN: LanguageIdentifier = langid!("en");

fn i18n_from_static() -> I18n {
    let config = I18nConfig::new(EN).with_locale((EN, include_str!("./data/i18n/en.ftl")));
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
