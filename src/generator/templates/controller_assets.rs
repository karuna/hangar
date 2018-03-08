pub static TEXT: &'static str = "use std::path::{Path, PathBuf};
use rocket::response::NamedFile;
use rocket::State;
use libs::settings::Settings;

#[get(\"/<file..>\")]
pub fn files(file: PathBuf, settings: State<Settings>) -> Option<NamedFile> {
    NamedFile::open(Path::new(&*settings.assets.assets_dir).join(file)).ok()
}
";