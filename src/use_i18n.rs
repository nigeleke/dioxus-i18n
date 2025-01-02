use dioxus_lib::prelude::*;
use fluent::{FluentArgs, FluentBundle, FluentResource};
use unic_langid::LanguageIdentifier;

#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;

/// `Locale` is a "place-holder" around what will eventually be a `fluent::FluentBundle`
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Locale {
    id: LanguageIdentifier,
    resource: LocaleResource,
}

impl Locale {
    #[deprecated(
        since = "0.3.0",
        note = "remove `Locale::new_static` and use `(lang_id, a_str)` directly"
    )]
    pub fn new_static(id: LanguageIdentifier, str: &'static str) -> Self {
        Self {
            id,
            resource: LocaleResource::Static(str),
        }
    }

    #[deprecated(
        since = "0.3.0",
        note = "remove `Locale::new_dynamic` and use `(lang_id, a_pathbuf)` directly"
    )]
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new_dynamic(id: LanguageIdentifier, path: impl Into<PathBuf>) -> Self {
        Self {
            id,
            resource: LocaleResource::Path(path.into()),
        }
    }
}

impl<T> From<(LanguageIdentifier, T)> for Locale
where
    T: Into<LocaleResource>,
{
    fn from((id, resource): (LanguageIdentifier, T)) -> Self {
        let resource = resource.into();
        Self { id, resource }
    }
}

/// A `LocaleResource` can be static text, or dervied from a file. The file derivation is not supported for `wasm`.
#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum LocaleResource {
    Static(&'static str),
    #[cfg(not(target_arch = "wasm32"))]
    Path(PathBuf),
}

impl LocaleResource {
    pub fn to_resource_string(&self) -> String {
        match self {
            Self::Static(str) => str.to_string(),
            #[cfg(not(target_arch = "wasm32"))]
            Self::Path(path) => {
                std::fs::read_to_string(path).expect("Failed to read locale resource")
            }
        }
    }
}

impl From<&'static str> for LocaleResource {
    fn from(value: &'static str) -> Self {
        Self::Static(value)
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<PathBuf> for LocaleResource {
    fn from(value: PathBuf) -> Self {
        Self::Path(value)
    }
}

/// The configuration for `I18n`.
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct I18nConfig {
    /// The initial value for [`I18n`][`set_language`]
    id: LanguageIdentifier,

    /// The final fallback language if no other locales are found for `id`.
    /// A `Locale` must exist in `locales' if `fallback` is defined.
    fallback: Option<LanguageIdentifier>,

    /// The locales added to the configuration.
    locales: Vec<Locale>,
}

impl I18nConfig {
    /// Create an i18n config with the selected [LanguageIdentifier].
    pub fn new(id: LanguageIdentifier) -> Self {
        Self {
            id,
            fallback: None,
            locales: Vec::new(),
        }
    }

    /// Set a fallback [LanguageIdentifier].
    pub fn with_fallback(mut self, fallback: LanguageIdentifier) -> Self {
        self.fallback = Some(fallback);
        self
    }

    /// Add [Locale].
    pub fn with_locale<T>(mut self, locale: T) -> Self
    where
        T: Into<Locale>,
    {
        let locale = locale.into();
        self.locales.push(locale);
        self
    }
}

/// Initialize an i18n provider.
pub fn use_init_i18n(init: impl FnOnce() -> I18nConfig) -> I18n {
    use_context_provider(move || {
        // Coverage false -ve: See https://github.com/xd009642/tarpaulin/issues/1675
        let I18nConfig {
            id,
            fallback,
            locales,
        } = init();

        I18n::new(id, fallback, locales)
    })
}

#[derive(Clone, Copy)]
pub struct I18n {
    selected_language: Signal<LanguageIdentifier>,
    fallback_language: Signal<Option<LanguageIdentifier>>,
    locales: Signal<Vec<Locale>>,
    active_bundle: Signal<FluentBundle<FluentResource>>,
}

impl I18n {
    pub fn new(
        selected_language: LanguageIdentifier,
        fallback_language: Option<LanguageIdentifier>,
        locales: Vec<Locale>,
    ) -> Self {
        let bundle = create_bundle(&selected_language, &fallback_language, &locales);
        Self {
            selected_language: Signal::new(selected_language),
            fallback_language: Signal::new(fallback_language),
            locales: Signal::new(locales),
            active_bundle: Signal::new(bundle),
        }
    }

    pub fn translate_with_args(&self, msg: &str, args: Option<&FluentArgs>) -> String {
        let bundle = self.active_bundle.read();
        let message = bundle
            .get_message(msg)
            .unwrap_or_else(|| panic!("Failed to get message: {}.", msg));
        let pattern = message.value().expect("Failed to get the message pattern.");
        let mut errors = vec![];

        bundle
            .format_pattern(pattern, args, &mut errors)
            .to_string()
    }

    pub fn translate(&self, msg: &str) -> String {
        self.translate_with_args(msg, None)
    }

    /// Get the selected language.
    pub fn language(&self) -> LanguageIdentifier {
        self.selected_language.read().clone()
    }

    /// Get the fallback language.
    pub fn fallback_language(&self) -> Option<LanguageIdentifier> {
        self.fallback_language.read().clone()
    }

    /// Update the selected language.
    pub fn set_language(&mut self, id: LanguageIdentifier) {
        *self.selected_language.write() = id;
        self.update_active_bundle();
    }

    /// Update the fallback language.
    pub fn set_fallback_language(&mut self, id: LanguageIdentifier) {
        *self.fallback_language.write() = Some(id);
        self.update_active_bundle();
    }

    fn update_active_bundle(&mut self) {
        let bundle = create_bundle(
            &self.selected_language.read(),
            &self.fallback_language.read(),
            &self.locales.read(),
        );
        self.active_bundle.set(bundle);
    }
}

fn create_bundle(
    selected_language: &LanguageIdentifier,
    fallback_language: &Option<LanguageIdentifier>,
    locales: &[Locale],
) -> FluentBundle<FluentResource> {
    let add_resource = |bundle: &mut FluentBundle<FluentResource>, locale: Option<&Locale>| {
        if let Some(locale) = locale {
            let resource = FluentResource::try_new(locale.resource.to_resource_string())
                .expect("Failed to ceate Resource.");
            bundle.add_resource_overriding(resource);
        }
    };

    let mut bundle = FluentBundle::new(vec![selected_language.clone()]);
    if let Some(fallback_language) = fallback_language {
        let resource = locales.iter().find(|l| l.id == *fallback_language);
        add_resource(&mut bundle, resource);
    }

    let (language, script, region, variants) = selected_language.clone().into_parts();
    let variants_lang = LanguageIdentifier::from_parts(language, script, region, &variants);
    let region_lang = LanguageIdentifier::from_parts(language, script, region, &[]);
    let script_lang = LanguageIdentifier::from_parts(language, script, None, &[]);
    let language_lang = LanguageIdentifier::from_parts(language, None, None, &[]);

    let resource = locales.iter().find(|l| l.id == language_lang);
    add_resource(&mut bundle, resource);

    let resource = locales.iter().find(|l| l.id == script_lang);
    add_resource(&mut bundle, resource);

    let resource = locales.iter().find(|l| l.id == region_lang);
    add_resource(&mut bundle, resource);

    let resource = locales.iter().find(|l| l.id == variants_lang);
    add_resource(&mut bundle, resource);

    bundle
}

pub fn i18n() -> I18n {
    consume_context()
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use unic_langid::langid;

    #[allow(deprecated)]
    #[test]
    fn can_add_locale_to_config_deprecating() {
        let lang_a = langid!("la-LA");
        let lang_b = langid!("la-LB");
        let lang_c = langid!("la-LC");
        let config = I18nConfig::new(lang_a.clone())
            .with_locale(Locale::new_static(lang_b.clone(), "lang = lang_b"))
            .with_locale(Locale::new_dynamic(lang_c.clone(), PathBuf::new()));
        assert_eq!(
            config,
            I18nConfig {
                id: lang_a,
                fallback: None,
                locales: vec![
                    Locale {
                        id: lang_b,
                        resource: LocaleResource::Static("lang = lang_b")
                    },
                    Locale {
                        id: lang_c,
                        resource: LocaleResource::Path(PathBuf::new())
                    }
                ]
            }
        );
    }

    #[test]
    fn can_add_locale_to_config_v0_4_0() {
        let lang_a = langid!("la-LA");
        let lang_b = langid!("la-LB");
        let lang_c = langid!("la-LC");
        let config = I18nConfig::new(lang_a.clone())
            .with_locale((lang_b.clone(), "lang = lang_b"))
            .with_locale((lang_c.clone(), PathBuf::new()));
        assert_eq!(
            config,
            I18nConfig {
                id: lang_a,
                fallback: None,
                locales: vec![
                    Locale {
                        id: lang_b,
                        resource: LocaleResource::Static("lang = lang_b")
                    },
                    Locale {
                        id: lang_c,
                        resource: LocaleResource::Path(PathBuf::new())
                    }
                ]
            }
        );
    }

    #[test]
    fn can_add_locale_string_to_config() {
        let lang_a = langid!("la-LA");
        let lang_b = langid!("la-LB");
        let config = I18nConfig::new(lang_a.clone()).with_locale((lang_b.clone(), "lang = lang_b"));
        assert_eq!(
            config,
            I18nConfig {
                id: lang_a,
                fallback: None,
                locales: vec![Locale {
                    id: lang_b,
                    resource: LocaleResource::Static("lang = lang_b")
                },]
            }
        );
    }

    #[test]
    fn can_add_locale_pathbuf_to_config() {
        let lang_a = langid!("la-LA");
        let lang_c = langid!("la-LC");
        let config = I18nConfig::new(lang_a.clone())
            .with_locale((lang_c.clone(), PathBuf::from("./test/data/fallback/la.ftl")));
        assert_eq!(
            config,
            I18nConfig {
                id: lang_a,
                fallback: None,
                locales: vec![Locale {
                    id: lang_c,
                    resource: LocaleResource::Path(PathBuf::from("./test/data/fallback/la.ftl"))
                }]
            }
        );
    }
}
