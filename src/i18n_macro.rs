/// Translate from key to formatted string, using `I18n::translate` or `I18n::translate_with_args`.
///
/// ```ftl
/// # en-US.ftl
/// #
/// hello = Hello, {$name}!
/// ```
///
/// ```rust
/// # use dioxus::prelude::*;
/// # use dioxus_i18n::{t, prelude::*};
/// # use unic_langid::langid;
/// # #[component]
/// # fn Example() -> Element {
/// #   let lang = langid!("en-US");
/// #   let config = I18nConfig::new(lang.clone()).with_locale((lang.clone(), "hello = Hello, {$name}")).with_fallback(lang.clone());
/// #   let mut i18n = use_init_i18n(|| config);
/// let name = "Avery Gigglesworth";
/// let hi = t!("hello", name: {name});
/// assert_eq!(hi, "Hello, Avery Gigglesworth");
/// #   rsx! { "" }
/// # }
/// ```
///
#[macro_export]
macro_rules! t {
    ($id:expr, $( $name:ident : $value:expr ),* ) => {
        {
            let mut params_map = dioxus_i18n::fluent::FluentArgs::new();
            $(
                params_map.set(stringify!($name), $value);
            )*
            dioxus_i18n::prelude::i18n().translate_with_args($id, Some(&params_map))
        }
    };

    ($id:expr ) => {
        {
            dioxus_i18n::prelude::i18n().translate($id)
        }
    };
}
