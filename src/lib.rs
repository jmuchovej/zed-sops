use zed_extension_api as zed;

use std::path::Path;
use directories::ProjectDirs;

struct SOPSExtension {
    enable: Option<bool>,
    bin_path: Option<String>,
    config_path: Option<String>,
    creation_enabled: Option<bool>,
    age_key_file: Option<String>,
};

impl zed::Extension for SOPSExtension {
    fn new() -> Self {
        let default_age_key_path = Path::new(
            ProjectDir::from("", "", "sops").config_dir()
        ).join("age").join("keys.txt");

        Self {
            enable: True,
            bin_path: "sops",
            config_path: ".sops.yaml",
            creation_enabled: false,
            age_key_file: default_age_key_path,
        }
    }
};

zed::register_extension!(SOPSExtension);
