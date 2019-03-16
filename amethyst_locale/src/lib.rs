//! # amethyst_locale
//!
//! Localisation binding a `Fluent` file to an Asset<Locale> via the use of amethyst_assets.

#![warn(missing_docs, rust_2018_idioms, rust_2018_compatibility)]

use fluent::{FluentBundle, FluentResource};

use amethyst_assets::{Asset, Handle, ProcessingState, SimpleFormat};
use amethyst_core::ecs::prelude::VecStorage;
use amethyst_error::Error;

/// Loads the strings from localisation files.
#[derive(Clone)]
pub struct TranslationFormat;

impl SimpleFormat<Translation> for TranslationFormat {
    const NAME: &'static str = "FTL";

    type Options = ();

    fn import(&self, bytes: Vec<u8>, _: ()) -> Result<Translation, Error> {
        let s = String::from_utf8(bytes)?;

        let resource = FluentResource::try_new(s).expect("Error creating fluent resource!");
        Ok(Translation { resource })
    }
}

impl Into<Result<ProcessingState<Translation>, Error>> for Translation {
    fn into(self) -> Result<ProcessingState<Translation>, Error> {
        Ok(ProcessingState::Loaded(self))
    }
}

pub struct Translation {
    pub resource: FluentResource,
}

impl Asset for Translation {
    const NAME: &'static str = "locale::Translation";
    type Data = Translation;
    type HandleStorage = VecStorage<Handle<Translation>>;
}

pub struct Locale {
    pub bundle: FluentBundle<'static>,
}

impl Into<Result<ProcessingState<Locale>, Error>> for Locale {
    fn into(self) -> Result<ProcessingState<Locale>, Error> {
        Ok(ProcessingState::Loaded(self))
    }
}

impl Asset for Locale {
    const NAME: &'static str = "locale::Locale";
    type Data = LocaleData;
    type HandleStorage = VecStorage<Handle<Locale>>;
}

impl From<LocaleData> for Result<ProcessingState<Locale>, Error> {
    fn from(locale_data: LocaleData) -> Result<ProcessingState<Locale>, Error> {
        Ok(ProcessingState::Loaded(Locale {
            bundle: FluentBundle::new(&locale_data.0),
        }))
    }
}

#[derive(Clone)]
pub struct LocaleData(pub Vec<&'static str>);
