mod common;
use common::*;

use dioxus_i18n::prelude::{use_init_i18n, I18n, I18nConfig};
use unic_langid::{langid, LanguageIdentifier};

use std::cell::LazyCell;

#[test]
fn exact_locale_match_will_use_translation() {
    test_hook(i18n, |value, proxy| {
        proxy.assert(
            &value.translate("variants"),
            "variants only",
            "exact_locale_match_will_use_translation",
        );
    });
}

#[test]
fn non_exact_locale_match_will_use_region() {
    test_hook(i18n, |value, proxy| {
        proxy.assert(
            &value.translate("region"),
            "region only",
            "non_exact_locale_match_will_use_region",
        );
    });
}

#[test]
fn non_exact_locale_match_will_use_script() {
    test_hook(i18n, |value, proxy| {
        proxy.assert(
            &value.translate("script"),
            "script only",
            "non_exact_locale_match_will_use_script",
        );
    });
}

#[test]
fn non_exact_locale_match_will_use_language() {
    test_hook(i18n, |value, proxy| {
        proxy.assert(
            &value.translate("language"),
            "language only",
            "non_exact_locale_match_will_use_language",
        );
    });
}

#[test]
fn no_locale_match_will_use_fallback() {
    test_hook(i18n, |value, proxy| {
        proxy.assert(
            &value.translate("fallback"),
            "fallback only",
            "no_locale_match_will_use_fallback",
        );
    });
}

const FALLBACK_LANG: LanguageIdentifier = langid!("fb-FB");
const LANGUAGE_LANG: LanguageIdentifier = langid!("la");
const SCRIPT_LANG: LanguageIdentifier = langid!("la-Scpt");
const REGION_LANG: LanguageIdentifier = langid!("la-Scpt-LA");
const VARIANTS_LANG: LazyCell<LanguageIdentifier> =
    LazyCell::new(|| langid!("la-Scpt-LA-variants"));

fn i18n() -> I18n {
    let config = I18nConfig::new(VARIANTS_LANG.clone())
        .with_locale((LANGUAGE_LANG, include_str!("../tests/data/fallback/la.ftl")))
        .with_locale((
            SCRIPT_LANG,
            include_str!("../tests/data/fallback/la-Scpt.ftl"),
        ))
        .with_locale((
            REGION_LANG,
            include_str!("../tests/data/fallback/la-Scpt-LA.ftl"),
        ))
        .with_locale((
            VARIANTS_LANG.clone(),
            include_str!("../tests/data/fallback/la-Scpt-LA-variants.ftl"),
        ))
        .with_locale((
            FALLBACK_LANG,
            include_str!("../tests/data/fallback/fb-FB.ftl"),
        ))
        .with_fallback(FALLBACK_LANG);
    use_init_i18n(|| config)
}
