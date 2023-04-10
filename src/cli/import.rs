use super::getenv;
use crate::rbx::Remodel;
use clap::Parser;

/// Import assets and maps
#[derive(Debug, Parser)]
pub struct ImportCommand {
    /// Whether to import assets
    #[clap(short = 'A', long, takes_value = false)]
    asset_flag: bool,
    /// Whether to import all maps
    #[clap(short = 'M', long, takes_value = false)]
    map_flag: bool,
    /// The path to the place file
    #[clap(short, long, value_parser)]
    file_path: Option<String>,
    /// The name of the map to import
    #[clap(short, long, value_parser)]
    map_name: Option<String>,
    /// The authentication token to use
    #[clap(short, long, value_parser)]
    auth: Option<String>,
}

impl ImportCommand {
    pub fn run(&self) -> anyhow::Result<Option<String>> {
        let auth = getenv(self.auth.clone(), "ROBLOSECURITY".to_string());
        let remodel = Remodel::new(auth);

        if self.asset_flag {
            remodel.run(&format!("import-{}.lua", "assets"), &[]);
        }

        if self.map_flag {
            remodel.run(&format!("import-{}.lua", "all-maps"), &[]);
        }

        if self.map_name.is_some() {
            if self.file_path.is_some() {
                remodel.run(
                    &format!("import-{}.lua", "local-map"),
                    &[
                        self.file_path.as_ref().unwrap(),
                        self.map_name.as_ref().unwrap(),
                    ],
                );
            } else {
                remodel.run(
                    &format!("import-{}.lua", "map"),
                    &[self.map_name.as_ref().unwrap()],
                );
            }
        }

        Ok(None)
    }
}
