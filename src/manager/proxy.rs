use zbus::proxy;
use zbus::{blocking, zvariant::OwnedObjectPath, Connection};

use crate::{Result, UnitTuple};

#[proxy(
    interface = "org.freedesktop.systemd1.Manager",
    default_service = "org.freedesktop.systemd1",
    default_path = "/org/freedesktop/systemd1"
)]
pub trait SystemdManager {
    fn get_unit(&self, name: &str) -> zbus::Result<OwnedObjectPath>;
    fn list_units(&self) -> zbus::Result<Vec<UnitTuple>>;
    fn list_unit_files(&self) -> zbus::Result<Vec<(String, String)>>;
    fn load_unit(&self, name: &str) -> zbus::Result<OwnedObjectPath>;
    fn reload_unit(&self, name: &str, mode: &str) -> zbus::Result<OwnedObjectPath>;
    fn restart_unit(&self, name: &str, mode: &str) -> zbus::Result<OwnedObjectPath>;
    fn start_unit(&self, name: &str, mode: &str) -> zbus::Result<OwnedObjectPath>;
    fn stop_unit(&self, name: &str, mode: &str) -> zbus::Result<OwnedObjectPath>;
    fn kill_unit(&self, name: &str, whom: &str, signal: i32) -> zbus::Result<()>;
    fn reset_failed_unit(&self, name: &str) -> zbus::Result<()>;
    #[allow(clippy::type_complexity)]
    fn enable_unit_files(
        &self,
        files: &[&str],
        runtime: bool,
        force: bool,
    ) -> zbus::Result<(bool, Vec<(String, String, String)>)>;
    fn disable_unit_files(
        &self,
        files: &[&str],
        runtime: bool,
    ) -> zbus::Result<Vec<(String, String, String)>>;
    fn get_unit_file_state(&self, arg_1: &str) -> zbus::Result<String>;
    #[zbus(property)]
    fn architecture(&self) -> zbus::Result<String>;
    #[zbus(property)]
    fn environment(&self) -> zbus::Result<Vec<String>>;
    fn reload(&self) -> zbus::Result<()>;
    fn reset_failed(&self) -> zbus::Result<()>;
}

pub async fn build_nonblocking_proxy() -> Result<SystemdManagerProxy<'static>> {
    let connection = Connection::system().await?;
    let proxy = SystemdManagerProxy::new(&connection).await?;
    Ok(proxy)
}

pub async fn build_nonblocking_user_proxy() -> Result<SystemdManagerProxy<'static>> {
    let connection = Connection::session().await?;
    let proxy = SystemdManagerProxy::new(&connection).await?;
    Ok(proxy)
}

pub fn build_blocking_proxy() -> Result<SystemdManagerProxyBlocking<'static>> {
    let connection = blocking::Connection::system()?;
    let proxy = SystemdManagerProxyBlocking::new(&connection)?;
    Ok(proxy)
}

pub fn build_blocking_user_proxy() -> Result<SystemdManagerProxyBlocking<'static>> {
    let connection = blocking::Connection::session()?;
    let proxy = SystemdManagerProxyBlocking::new(&connection)?;
    Ok(proxy)
}
