//! Example showing how to load a Locale file as an Asset using the Loader.

use amethyst::{
    assets::{AssetStorage, Handle, Loader, Processor, ProgressCounter},
    ecs::{Read, ReadExpect},
    locale::*,
    prelude::*,
    utils::application_root_dir,
    Error,
};

struct Example {
    progress_counter: Option<ProgressCounter>,
    resource_en: Option<Handle<Translation>>,
    resource_fr: Option<Handle<Translation>>,
    bundle_en: Option<Handle<Locale>>,
    bundle_fr: Option<Handle<Locale>>,
}

impl Example {
    pub fn new() -> Self {
        Example {
            progress_counter: None,
            resource_en: None,
            resource_fr: None,
            bundle_en: None,
            bundle_fr: None,
        }
    }
}

impl SimpleState for Example {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world.add_resource(AssetStorage::<Translation>::new());
        let mut progress_counter = ProgressCounter::default();
        self.resource_en = Some(data.world.exec(
            |(loader, storage): (ReadExpect<'_, Loader>, Read<'_, AssetStorage<Translation>>)| {
                loader.load(
                    "locale/locale_en.ftl",
                    TranslationFormat,
                    (),
                    &mut progress_counter,
                    &storage,
                )
            },
        ));
        self.resource_fr = Some(data.world.exec(
            |(loader, storage): (ReadExpect<'_, Loader>, Read<'_, AssetStorage<Translation>>)| {
                loader.load(
                    "locale/locale_fr.ftl",
                    TranslationFormat,
                    (),
                    &mut progress_counter,
                    &storage,
                )
            },
        ));

        data.world.add_resource(AssetStorage::<Locale>::new());
        self.bundle_en = Some(data.world.exec(
            |(loader, storage): (ReadExpect<'_, Loader>, Read<'_, AssetStorage<Locale>>)| {
                loader.load_from_data(LocaleData(vec!["en"]), &mut progress_counter, &storage)
            },
        ));
        self.bundle_fr = Some(data.world.exec(
            |(loader, storage): (ReadExpect<'_, Loader>, Read<'_, AssetStorage<Locale>>)| {
                loader.load_from_data(LocaleData(vec!["fr"]), &mut progress_counter, &storage)
            },
        ));

        self.progress_counter = Some(progress_counter);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        // Check if the locale has been loaded.
        if self.progress_counter.as_ref().unwrap().is_complete() {
            let resources = data.world.read_resource::<AssetStorage<Translation>>();
            let bundles = data.world.read_resource::<AssetStorage<Locale>>();
            for (resource, bundle) in [
                (&self.resource_en, &self.bundle_en),
                (&self.resource_fr, &self.bundle_fr),
            ]
            .iter()
            {
                if let Some(resource) = resource
                    .as_ref()
                    .and_then(|resource| resources.get(resource))
                {
                    if let Some(bundle) = bundle.as_ref().and_then(|bundle| bundles.get_mut(bundle))
                    {
                        bundle.bundle.add_resource(&resource.resource).unwrap();

                        println!("{}", bundle.bundle.format("hello", None).unwrap().0);
                        println!("{}", bundle.bundle.format("bye", None).unwrap().0);
                    }
                }
            }
            Trans::Quit
        } else {
            Trans::None
        }
    }
}

fn main() -> Result<(), Error> {
    amethyst::start_logger(Default::default());

    let resources_directory = application_root_dir()?.join("examples/assets");

    let game_data = GameDataBuilder::default().with(Processor::<Locale>::new(), "proc", &[]);

    let mut game = Application::new(resources_directory, Example::new(), game_data)?;
    game.run();
    Ok(())
}
