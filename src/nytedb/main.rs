#![allow(dead_code)]
// NOTE: Look into clap_mangen for man page generation
// NOTE: Look into assert_cmd and assert_fs for testing

// Expose everything from common
mod common;
pub use common::*;

pub mod database;

use clap::{Parser, Subcommand};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use crate::database::find_database;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    // Updates the file database with all the files at a given path
    Update { path: String },
    Ignore { path: Option<String> },
    Verify,
    Compare { path_one: String, path_two: String },
    Find,
    Standalone { path: String },
}

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Setting default subscriber failed!");

    info!("Beginning NyteFDB Initialization");

    // Initialize Clap
    let _args = Args::parse();

    // TODO: Parse Command Line Arguments
    let path = None;

    // TODO: Open Existing Database
    let _database = find_database(false, true, path);

    // TODO: Verify Database

    // info!("NyteFDB Initialization Complete");
    //
    // match &args.command {
    //     // TODO: UPDATE command
    //         // Updates the database with all file data from the given path(s). If no path is provided,
    //         // UPDATE will attempt to update itself with information starting at the current user's
    //         // home directory.
    //         //
    //         // If invoked as root, it will instead search all directories from the root directory, and
    //         // the database used will be at the application's global data directory.
    //
    //     Commands::Update{ path} => {
    //         debug!("Update called with path: {}", path);
    //     },
    //     // TODO: IGNORE command
    //         // Adds a .ndbignore file to the passed directory, or the current directory if no path is
    //         // given. Will delete all entries from the current DB with that directory as a prefix,
    //         // unless the --keep-ignored argument is given or "keep-ignored" has been set to true in
    //         // the application's config.
    //
    //     // TODO: VERIFY command
    //         // Verify all files in the database. Forces a re-hash of all files either at the given
    //         // path or from the home directory or root directory, along the same lines as the UPDATE
    //         // command, ignoring heuristics for determining if the files have actually changed or not.
    //
    //     // TODO: COMPARE command
    //         // Compares two directory trees in the database. Alternatively can use two separate
    //         // databases.
    //
    //     // TODO: FIND command
    //         // Find files in the database based on queryable values, such as size, prefix, regex,
    //         // hash, or by comparison to each other. Syntax is complicated, as many combinations can
    //         // exist, but examples are as follows:
    //         //
    //         // # Find all file entries where the hash is identical
    //         // nytedb find --duplicate-hashes
    //         //
    //         // # Find all files under the two given path prefixes that have identical hashes
    //         // nytedb find --duplicate-hashes --with-prefix=<path> --with-prefix=<path2>
    //         //
    //         // # Find all files greater than or equal to 1GiB in size
    //         // nytedb find --size >=1GiB
    //         //
    //         // # Find all files that end in the "mkv" file extension. NOTE: The '.' preceding the
    //         // extension is optional. This will not match files ending in 'mkv' with no preceding dot.
    //         // nytedb find --with-extension=mkv
    //         //
    //         // # Find all files that contain the following pattern in their basename. Note that this
    //         // regex should be identical to the previous --with-extension invocation.
    //         // nytedb find --name ".*\.mkv$"
    //         //
    //         // # Note: If you wish to run the regex against the whole path, you can use --path
    //         // instead. If you wish to run the regex against each piece of the path separately, you
    //         // can use --path-contains.
    //         //
    //         // #Final notes: All arguments to the find command are implicitly "or"ed together.
    //         // #Therefore, if you perform a search such as:
    //         // nytedb find --size >=1GiB --duplicate-hashes
    //         //
    //         // The default behavior will be to find files that are EITHER 1GiB and greater in size, OR
    //         // have duplicate hashes to one another. If you instead would prefer to find files that
    //         // match both criteria, you must use the --and option like so:
    //         // nytedb find --size >=1GiB --and --duplicate-hashes
    //         //
    //         // As well as using -and, you can use -not before a predicate to invert the selection. So
    //         // for all files that do not have a duplicate hash, for instance, you can use:
    //         // nytedb find --not --duplicate-hashes
    //
    //     // TODO: STANDALONE command
    //         // Creates a standalone copy of the file database for the given path, placing the database
    //         // at the passed directory.
    //     _ => unimplemented!(),
    // }
}
