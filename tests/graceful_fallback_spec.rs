use dioxus::{dioxus_core::NoOpMutations, prelude::*};
use dioxus_i18n::prelude::{use_init_i18n, I18n, I18nConfig};
use futures::FutureExt;
use unic_langid::{langid, LanguageIdentifier};

use std::{
    cell::{LazyCell, RefCell},
    rc::Rc,
};

//*****************************************************************************
//
// This set of tests takes a heavy handed approach to errors, whereby the
// process is exited. This is done because panic! and assert_eq! failures
// are trapped within `dioxus::runtime::RuntimeGuard`.
//
//*****************************************************************************

#[test]
fn exact_locale_match_will_use_translation() {
    test_hook(
        i18n,
        |value, _proxy| {
            assert_and_force_exit_if_false(
                "exact_locale_match_will_use_translation",
                &value.translate("variants"),
                "variants only",
            );
        },
        |_| {},
    );
}

#[test]
fn non_exact_locale_match_will_use_region() {
    test_hook(
        i18n,
        |value, _proxy| {
            assert_and_force_exit_if_false(
                "non_exact_locale_match_will_use_region",
                &value.translate("region"),
                "region only",
            );
        },
        |_| {},
    );
}

#[test]
fn non_exact_locale_match_will_use_script() {
    test_hook(
        i18n,
        |value, _proxy| {
            assert_and_force_exit_if_false(
                "non_exact_locale_match_will_use_script",
                &value.translate("script"),
                "script only",
            );
        },
        |_| {},
    );
}

#[test]
fn non_exact_locale_match_will_use_language() {
    test_hook(
        i18n,
        |value, _proxy| {
            assert_and_force_exit_if_false(
                "non_exact_locale_match_will_use_language",
                &value.translate("language"),
                "language only",
            );
        },
        |_| {},
    );
}

#[test]
fn no_locale_match_will_use_fallback() {
    test_hook(
        i18n,
        |value, _proxy| {
            assert_and_force_exit_if_false(
                "no_locale_match_will_use_fallback",
                &value.translate("fallback"),
                "fallback only",
            );
        },
        |_| {},
    );
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

fn assert_and_force_exit_if_false(fn_name: &str, actual: &str, expected: &str) {
    if actual != expected {
        eprintln!(
            "\n***** FAIL {} *****: {actual:?} != {expected:?}\n",
            fn_name.to_uppercase()
        );
        std::process::exit(-1);
    }
}

// Lifted from: https://dioxuslabs.com/learn/0.6/cookbook/testing
// Curtailed MockProxy implmentation.
//
fn test_hook<V: 'static>(
    initialize: impl FnMut() -> V + 'static,
    check: impl FnMut(V, MockProxy) + 'static,
    mut final_check: impl FnMut(MockProxy) + 'static,
) {
    #[derive(Props)]
    struct MockAppComponent<I: 'static, C: 'static> {
        hook: Rc<RefCell<I>>,
        check: Rc<RefCell<C>>,
    }

    impl<I, C> PartialEq for MockAppComponent<I, C> {
        fn eq(&self, _: &Self) -> bool {
            true
        }
    }

    impl<I, C> Clone for MockAppComponent<I, C> {
        fn clone(&self) -> Self {
            Self {
                hook: self.hook.clone(),
                check: self.check.clone(),
            }
        }
    }

    fn mock_app<I: FnMut() -> V, C: FnMut(V, MockProxy), V>(
        props: MockAppComponent<I, C>,
    ) -> Element {
        let value = props.hook.borrow_mut()();

        props.check.borrow_mut()(value, MockProxy::new());

        rsx! { div {} }
    }

    let mut vdom = VirtualDom::new_with_props(
        mock_app,
        MockAppComponent {
            hook: Rc::new(RefCell::new(initialize)),
            check: Rc::new(RefCell::new(check)),
        },
    );

    vdom.rebuild_in_place();

    while vdom.wait_for_work().now_or_never().is_some() {
        vdom.render_immediate(&mut NoOpMutations);
    }

    vdom.in_runtime(|| {
        ScopeId::ROOT.in_runtime(|| {
            final_check(MockProxy::new());
        })
    })
}

struct MockProxy {}

impl MockProxy {
    fn new() -> Self {
        Self {}
    }
}
