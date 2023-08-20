use crate::{
    constants::SYSTEMD_USER_UNIT_CONFIGURATION_DIRECTORY_SUFFIX, Result,
    SYSTEMD_UNIT_CONFIGURATION_DIRECTORY,
};

use std::io::Write;

fn create_from_path(path: &std::path::Path, buffer: &[u8]) -> Result<()> {
    std::fs::create_dir_all(path.parent().expect("Path should have a parent directory"))?;
    let file = std::fs::File::create(path)?;
    let mut writer = std::io::BufWriter::new(file);
    writer.write_all(buffer)?;
    writer.flush()?;
    Ok(())
}

pub fn create_unit_configuration_file(unit_name: &str, buffer: &[u8]) -> Result<()> {
    let mut path = std::path::PathBuf::from(SYSTEMD_UNIT_CONFIGURATION_DIRECTORY);
    path.push(unit_name);
    create_from_path(path.as_path(), buffer)
}

pub fn create_user_unit_configuration_file(unit_name: &str, buffer: &[u8]) -> Result<()> {
    let mut config_home = dirs::config_dir().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Unable to find user config directory",
        )
    })?;

    config_home.push(SYSTEMD_USER_UNIT_CONFIGURATION_DIRECTORY_SUFFIX);
    config_home.push(unit_name);
    create_from_path(config_home.as_path(), buffer)
}

pub fn delete_unit_configuration_file(unit_name: &str) -> Result<()> {
    let mut path = std::path::PathBuf::from(SYSTEMD_UNIT_CONFIGURATION_DIRECTORY);
    path.push(unit_name);
    if path.exists() {
        std::fs::remove_file(path)?;
    }
    Ok(())
}

pub fn delete_user_unit_configuration_file(unit_name: &str) -> Result<()> {
    let mut path = dirs::config_dir().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Unable to find user config directory",
        )
    })?;

    path.push(SYSTEMD_USER_UNIT_CONFIGURATION_DIRECTORY_SUFFIX);
    path.push(unit_name);
    if path.exists() {
        std::fs::remove_file(path)?;
    }
    Ok(())
}
