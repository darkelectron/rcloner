use clap:: {
    Args,
    Parser,
    Subcommand
};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct RclonerArgs {
    /// copy
    // #[arg(long, short)]
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// copy files
    Copy(CopyCommand),

    /// mount cloud drive to Cloud folder
    Mount(MountCommand),

    /// list files
    List(ListCommand),
}

#[derive(Debug, Args)]
pub struct CopyCommand {
    /// Enter source folder
    pub source: String,

    /// Enter target folder
    pub target: String,
}

#[derive(Debug, Args)]
pub struct MountCommand {
}

#[derive(Debug, Args)]
pub struct ListCommand {

}
