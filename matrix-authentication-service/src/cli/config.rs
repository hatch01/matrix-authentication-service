// Copyright 2021 The Matrix.org Foundation C.I.C.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use clap::Clap;
use schemars::schema_for;
use tracing::info;

use super::RootCommand;
use crate::config::RootConfig;

#[derive(Clap, Debug)]
pub(super) struct ConfigCommand {
    #[clap(subcommand)]
    subcommand: ConfigSubcommand,
}

#[derive(Clap, Debug)]
enum ConfigSubcommand {
    /// Dump the current config as YAML
    Dump,

    /// Print the JSON Schema that validates configuration files
    Schema,

    /// Check a config file
    Check,
}

impl ConfigCommand {
    pub async fn run(&self, root: &RootCommand) -> anyhow::Result<()> {
        use ConfigSubcommand as SC;
        match &self.subcommand {
            SC::Dump => {
                let config: RootConfig = root.load_config()?;

                serde_yaml::to_writer(std::io::stdout(), &config)?;

                Ok(())
            }
            SC::Schema => {
                let schema = schema_for!(RootConfig);

                serde_yaml::to_writer(std::io::stdout(), &schema)?;

                Ok(())
            }
            SC::Check => {
                let _config: RootConfig = root.load_config()?;
                info!(path = ?root.config, "Configuration file looks good");
                Ok(())
            }
        }
    }
}
