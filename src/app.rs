use clap;

/// Produces the [clap](https://docs.rs/clap/)
/// [App](https://docs.rs/clap/2.32.0/clap/struct.App.html) for argument parsing for the ellipsis
/// CLI. The app's first argument will always be parsed as subcommand, which will be one of `config
/// | destroy | edit | git | home | init| link | unlink`. If the first argument passed to the app
/// is not one of these strings, the app will panic. All subsequent arguments are passed as
/// arguments and/or options to the specified subcommand. For more information on what flags and
/// arguments each subcommand accepts, check `man ellipsis-[subcommand]`
///
pub fn app<'a, 'b>() -> clap::App<'a, 'b> {
    clap::App::new(crate_name!())
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .subcommand(clap::SubCommand::with_name("config")
                    .about("Manages the configuation file")
                    .arg(clap::Arg::with_name("edit")
                         .long("edit")
                         .short("e")
                         .help("Open the config file for editing")))
        .subcommand(clap::SubCommand::with_name("destroy")
                     .about("Destroy the current installation")
                     .arg(clap::Arg::with_name("force")
                          .long("force")
                          .short("f")
                          .help("Force the deletion of the given files. Without this option,
                       this command will not modify the filesystem."))
                     .arg(clap::Arg::with_name("dry-run")
                          .long("dry-run")
                          .short("n")
                          .help("Don't actually do anything, just show what would happen"))
                     .group(clap::ArgGroup::with_name("effect")
                            .args(&["force", "dry-run"])
                            .required(true)))
        .subcommand(clap::SubCommand::with_name("edit")
                    .about("Edit the given config file")
                    .arg(clap::Arg::with_name("filepath")
                         .index(1)
                         .help("The file to edit")))
        .subcommand(clap::SubCommand::with_name("git")
                    .setting(clap::AppSettings::TrailingVarArg)
                    .arg(clap::Arg::with_name("gitargs") // The arguments to forward to the git command
                         .multiple(true)
                         .allow_hyphen_values(true)
                         .default_value("help")))
        .subcommand(clap::SubCommand::with_name("home")
                    .about("Print the ellipsis home directory"))
        .subcommand(clap::SubCommand::with_name("init")
                    .about("Init the repository")
                    .arg(clap::Arg::with_name("URI")
                         .index(1)
                         .help("The repository to fetch")))
        .subcommand(clap::SubCommand::with_name("link")
                    .about("Properly symlink files"))
        .subcommand(clap::SubCommand::with_name("unlink")
                    .about("Delete symlinks"))
}

