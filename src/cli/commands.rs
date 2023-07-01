use clap::Args;

#[derive(Args)]
pub struct FormatArgs {
    pub json: String,
}

#[derive(Args)]
pub struct Json2yamlArgs {
    pub json: String,
}