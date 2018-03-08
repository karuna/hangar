pub static BUILD_HELP: &'static str = "Wrapper for `cargo build`.
Compile a local package and all of its dependencies

Usage:
    hangar build [options]

Options:
    -h, --help                   Print this message
    -p SPEC, --package SPEC ...  Package to build
    --all                        Build all packages in the workspace
    --exclude SPEC ...           Exclude packages from the build
    -j N, --jobs N               Number of parallel jobs, defaults to # of CPUs
    --lib                        Build only this package's library
    --bin NAME                   Build only the specified binary
    --bins                       Build all binaries
    --example NAME               Build only the specified example
    --examples                   Build all examples
    --test NAME                  Build only the specified test target
    --tests                      Build all tests
    --bench NAME                 Build only the specified bench target
    --benches                    Build all benches
    --all-targets                Build all targets (lib and bin targets by default)
    --release                    Build artifacts in release mode, with optimizations
    --features FEATURES          Space-separated list of features to also build
    --all-features               Build all available features
    --no-default-features        Do not build the `default` feature
    --target TRIPLE              Build for the target triple
    --manifest-path PATH         Path to the manifest to compile
    -v, --verbose ...            Use verbose output (-vv very verbose/build.rs output)
    -q, --quiet                  No output printed to stdout
    --color WHEN                 Coloring: auto, always, never
    --message-format FMT         Error format: human, json [default: human]
    --frozen                     Require Cargo.lock and cache are up to date
    --locked                     Require Cargo.lock is up to date
    -Z FLAG ...                  Unstable (nightly-only) flags to Cargo

If the --package argument is given, then SPEC is a package id specification
which indicates which package should be built. If it is not given, then the
current package is built. For more information on SPEC and its format, see the
`cargo help pkgid` command.

All packages in the workspace are built if the `--all` flag is supplied. The
`--all` flag is automatically assumed for a virtual manifest.
Note that `--exclude` has to be specified in conjunction with the `--all` flag.

Compilation can be configured via the use of profiles which are configured in
the manifest. The default profile for this command is `dev`, but passing
the --release flag will use the `release` profile instead.";

pub static DB_HELP: &'static str = "hangar db 1.1.0

USAGE:
    hangar db [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --database-url <DATABASE_URL>    Specifies the database URL to connect to. Falls back to the DATABASE_URL
                                         environment variable if unspecified.

SUBCOMMANDS:
    bash-completion    Generate bash completion script for the hangar db command.
    database           A group of commands for setting up and resetting your database.
    help               Prints this message or the help of the given subcommand(s)
    migration          A group of commands for generating, running, and reverting migrations.
    print-schema       Print table definitions for database schema.
    setup              Creates the migrations directory, creates the database specified in your DATABASE_URL, and
                       runs existing migrations.

You can also run `hangar db SUBCOMMAND -h` to get more information about that subcommand.";

pub static RUN_HELP: &'static str = "Wrapper for `cargo run`.
Run the main binary of the local package (src/main.rs)

Usage:
    hangar run [options] [--] [<args>...]

Options:
    -h, --help                   Print this message
    --bin NAME                   Name of the bin target to run
    --example NAME               Name of the example target to run
    -p SPEC, --package SPEC      Package with the target to run
    -j N, --jobs N               Number of parallel jobs, defaults to # of CPUs
    --release                    Build artifacts in release mode, with optimizations
    --features FEATURES          Space-separated list of features to also build
    --all-features               Build all available features
    --no-default-features        Do not build the `default` feature
    --target TRIPLE              Build for the target triple
    --manifest-path PATH         Path to the manifest to execute
    -v, --verbose ...            Use verbose output (-vv very verbose/build.rs output)
    -q, --quiet                  No output printed to stdout
    --color WHEN                 Coloring: auto, always, never
    --message-format FMT         Error format: human, json [default: human]
    --frozen                     Require Cargo.lock and cache are up to date
    --locked                     Require Cargo.lock is up to date
    -Z FLAG ...                  Unstable (nightly-only) flags to Cargo

If neither `--bin` nor `--example` are given, then if the project only has one
bin target it will be run. Otherwise `--bin` specifies the bin target to run,
and `--example` specifies the example target to run. At most one of `--bin` or
`--example` can be provided.

All of the trailing arguments are passed to the binary to run. If you're passing
arguments to both Cargo and the binary, the ones after `--` go to the binary,
the ones before go to Cargo.";
