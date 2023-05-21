use crate::{Result, ServiceProps};
use zbus::{blocking, dbus_proxy, zvariant::OwnedObjectPath, Connection};

#[dbus_proxy(
    interface = "org.freedesktop.systemd1.Service",
    default_service = "org.freedesktop.systemd1"
)]
trait SystemdService {
    #[dbus_proxy(property, name = "ExecMainPID")]
    fn exec_main_pid(&self) -> zbus::Result<u32>;
    #[dbus_proxy(property)]
    fn exec_main_code(&self) -> zbus::Result<i32>;
    #[dbus_proxy(property)]
    fn exec_main_status(&self) -> zbus::Result<i32>;
}

impl SystemdServiceProxyBlocking<'_> {
    pub fn get_properties(&self) -> zbus::Result<ServiceProps> {
        let exec_main_pid = self.exec_main_pid()?;
        let exec_main_code = self.exec_main_code()?;
        let exec_main_status = self.exec_main_status()?;
        let service_props = ServiceProps::builder()
            .exec_main_pid(exec_main_pid)
            .exec_main_code(exec_main_code)
            .exec_main_status(exec_main_status)
            .build();

        Ok(service_props)
    }
}

impl SystemdServiceProxy<'_> {
    pub async fn get_properties(&self) -> zbus::Result<ServiceProps> {
        let exec_main_pid = self.exec_main_pid().await?;
        let exec_main_code = self.exec_main_code().await?;
        let exec_main_status = self.exec_main_status().await?;
        let service_props = ServiceProps::builder()
            .exec_main_pid(exec_main_pid)
            .exec_main_code(exec_main_code)
            .exec_main_status(exec_main_status)
            .build();

        Ok(service_props)
    }
}

pub async fn build_nonblock_proxy(object: OwnedObjectPath) -> Result<SystemdServiceProxy<'static>> {
    let connection = Connection::system().await?;
    let proxy = SystemdServiceProxy::builder(&connection)
        .path(object)?
        .build()
        .await?;
    Ok(proxy)
}

pub fn build_blocking_proxy(
    object: OwnedObjectPath,
) -> Result<SystemdServiceProxyBlocking<'static>> {
    let connection = blocking::Connection::system()?;
    let proxy = SystemdServiceProxyBlocking::builder(&connection)
        .path(object)?
        .build()?;
    Ok(proxy)
}

pub async fn build_nonblock_user_proxy(
    object: OwnedObjectPath,
) -> Result<SystemdServiceProxy<'static>> {
    let connection = Connection::session().await?;
    let proxy = SystemdServiceProxy::builder(&connection)
        .path(object)?
        .build()
        .await?;
    Ok(proxy)
}

pub fn build_blocking_user_proxy(
    object: OwnedObjectPath,
) -> Result<SystemdServiceProxyBlocking<'static>> {
    let connection = blocking::Connection::session()?;
    let proxy = SystemdServiceProxyBlocking::builder(&connection)
        .path(object)?
        .build()?;
    Ok(proxy)
}
