use crate::constants::{APPLICATION_DB_LOC_ENV_VAR, APPLICATION_DB_NAME, APPLICATION_NAME};
use core::{option::Option::None, unimplemented};
use std::{
    env,
    path::{Path, PathBuf},
};
use thiserror::Error;

pub struct FileDatabase {
    path: PathBuf,
}

#[derive(Error, Debug)]
pub enum FileDatabaseError {
    #[error("database was not found at the expected locations")]
    DatabaseNotFoundError,

    #[error("the database was unable to be created")]
    DatabaseCreationError,

    #[error("the database could not be opened")]
    DatabaseOpenError,

    #[error("a generic IO error occurred: {0}")]
    DatabaseIOError(std::io::Error),

    #[error("an error occurred while closing the database")]
    DatabaseCloseError,
}

/******************************************************************************/
/*   CREATION                                                                 */
/******************************************************************************/

pub fn find_database(
    _read_only: bool,
    create: bool,
    search_path: Option<&Path>,
) -> Result<FileDatabase, FileDatabaseError> {
    let maybe_path: Option<PathBuf> = attempt_locate_db(search_path);

    let path = match (maybe_path, create) {
        (Some(path), _) => path,
        (None, true) => {
            unimplemented!()
        }
        (None, false) => {
            return Err(FileDatabaseError::DatabaseNotFoundError);
        }
    };

    Ok(FileDatabase { path })
}

#[allow(clippy::unnecessary_wraps)]
fn create_database(path: &Path) -> Result<FileDatabase, FileDatabaseError> {
    Ok(FileDatabase {
        path: path.to_owned(),
    })
}

fn attempt_locate_db(search_path: Option<&Path>) -> Option<PathBuf> {
    search_path
        .and_then(attempt_locate_db_at_path)
        .or_else(attempt_locate_db_from_env)
        .or_else(attempt_locate_db_in_app_data_directory)
        .or_else(attempt_locate_db_in_cwd)
}

fn attempt_locate_db_at_path(path: &Path) -> Option<PathBuf> {
    let db_path = path.join(APPLICATION_DB_NAME);
    db_path.exists().then_some(db_path)
}

fn attempt_locate_db_from_env() -> Option<PathBuf> {
    env::var_os(APPLICATION_DB_LOC_ENV_VAR)
        .map(PathBuf::from)
        .as_deref()
        .and_then(attempt_locate_db_at_path)
}

fn attempt_locate_db_in_app_data_directory() -> Option<PathBuf> {
    let data_dir: PathBuf = dirs::data_local_dir()?.join(APPLICATION_NAME);
    attempt_locate_db_at_path(&data_dir)
}

fn attempt_locate_db_in_cwd() -> Option<PathBuf> {
    let cwd = env::current_dir().ok()?;
    attempt_locate_db_at_path(&cwd)
}

//fn create_database_path(

#[cfg(test)]
mod tests {
    // TODO: Write Unit Tests
}
