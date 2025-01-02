# Changelog

## [Unreleased]

### Added
- Doc comments

- Module tests for `cargo test`

- Amended `I18nConfig::with_locale` so that the `Locale` dynamic or static
  constructors no longer have to be _explicitly_ given.
  They can be determined implicitly from `(LanguageIdentifier, &str)`  or
  `(LanguageIdentifer, PathBuf)`.
  The explicit constructors have been flagged as deprecated.

### Changed

- The translations used are determined when `I18n::set_language` or
  `I18n::set_fallback_language` is called, and not each time a message is translated.

- __Fallback handling has changed__. It no longer just uses _fallback_language_ when the message
  id is missing from the current _locale_. It performs a graceful fallback from
  _<language>-<region>_ to _<language>_ before using the actual _fallback_ (in fact it
  falls back along the _<language>-<optionalScript>-<optionalRegion>-<optionalVariants>_
  hiearchy).

  __Note:__ this is a breaking change which may impact the selected translation.

## [0.3.0] 2025-12-10

### Updated
- Support for [Dioxus v0.6](https://dioxuslabs.com/)
