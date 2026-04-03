use clap::Args;

#[derive(Args)]
pub struct DevCommand {}

impl DevCommand {
    pub async fn run(self) {}
}
