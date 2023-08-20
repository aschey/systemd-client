use std::fmt::Display;

// configuration templates of systemd
pub struct UnitConfiguration<'a> {
    pub description: &'a str,
    pub after: Vec<&'a str>,
    pub wants: Vec<&'a str>,
}

impl<'a> Display for UnitConfiguration<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[Unit]")?;
        writeln!(f, "Description={}", self.description)?;
        writeln!(f, "After={}", self.after.join(" "))?;
        writeln!(f, "Wants={}", self.wants.join(" "))
    }
}

impl<'a> UnitConfiguration<'a> {
    pub fn builder() -> UnitConfigurationBuilder<'a> {
        UnitConfigurationBuilder::default()
    }
}

#[derive(Default)]
pub struct UnitConfigurationBuilder<'a> {
    pub description: &'a str,
    pub after: Vec<&'a str>,
    pub wants: Vec<&'a str>,
}

impl<'a> UnitConfigurationBuilder<'a> {
    pub fn description(mut self, description: &'a str) -> Self {
        self.description = description;
        self
    }

    pub fn after(mut self, after: &'a str) -> Self {
        self.after.push(after);
        self
    }

    pub fn wants(mut self, wants: &'a str) -> Self {
        self.wants.push(wants);
        self
    }

    pub fn build(self) -> UnitConfiguration<'a> {
        let description = self.description;
        let after = self.after;
        let wants = self.wants;
        UnitConfiguration {
            description,
            after,
            wants,
        }
    }
}

pub enum ServiceType {
    Simple,
    Exec,
    Forking,
    Notify,
}

impl Display for ServiceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ty = match self {
            ServiceType::Simple => "simple",
            ServiceType::Exec => "exec",
            ServiceType::Forking => "forking",
            ServiceType::Notify => "notify",
        };
        write!(f, "{}", ty)
    }
}

pub enum NotifyAccess {
    Main,
    Exec,
    All,
}

impl Display for NotifyAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let access = match self {
            NotifyAccess::Main => "main",
            NotifyAccess::Exec => "exec",
            NotifyAccess::All => "all",
        };
        write!(f, "{}", access)
    }
}

pub enum RestartPolicy {
    No,
    OnSuccess,
    OnFailure,
    Always,
}

impl Display for RestartPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let policy = match self {
            RestartPolicy::No => "no",
            RestartPolicy::OnSuccess => "on-success",
            RestartPolicy::OnFailure => "on-failure",
            RestartPolicy::Always => "always",
        };
        write!(f, "{}", policy)
    }
}

pub struct EnvironmentVariable<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

impl<'a> EnvironmentVariable<'a> {
    pub fn builder() -> EnvironmentVariableBuilder<'a> {
        EnvironmentVariableBuilder::default()
    }
}

impl<'a> Display for EnvironmentVariable<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}", self.key, self.value)
    }
}

pub struct EnvironmentVariableBuilder<'a> {
    pub key: Option<&'a str>,
    pub value: Option<&'a str>,
}

impl<'a> Default for EnvironmentVariableBuilder<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> EnvironmentVariableBuilder<'a> {
    pub fn new() -> Self {
        EnvironmentVariableBuilder {
            key: None,
            value: None,
        }
    }

    pub fn key(mut self, key: &'a str) -> Self {
        self.key = Some(key);
        self
    }

    pub fn value(mut self, value: &'a str) -> Self {
        self.value = Some(value);
        self
    }

    pub fn build(self) -> EnvironmentVariable<'a> {
        let key = self.key.expect("key undefined");
        let value = self.value.expect("value undefined");
        EnvironmentVariable { key, value }
    }
}

// https://www.freedesktop.org/software/systemd/man/systemd.service.html#Service%20Templates
pub struct ServiceConfiguration<'a> {
    pub ty: ServiceType,
    pub notify_access: Option<NotifyAccess>,
    pub exec_start: Vec<&'a str>,
    pub restart_policy: RestartPolicy,
    // a unit-less value in seconds, or a time span value such as "5min 20s"
    pub restart_sec: &'a str,
    pub working_directory: Option<&'a str>,
    pub user: Option<&'a str>,
    pub group: Option<&'a str>,
    pub envs: Vec<EnvironmentVariable<'a>>,
}

impl<'a> Display for ServiceConfiguration<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[Service]")?;
        if let Some(working_directory) = self.working_directory {
            writeln!(f, "WorkingDirectory={working_directory}")?;
        }
        if let Some(user) = self.user {
            writeln!(f, "User={user}")?;
        }
        if let Some(group) = self.group {
            writeln!(f, "Group={group}")?;
        }
        if let Some(notify_access) = &self.notify_access {
            writeln!(f, "NotifyAccess={notify_access}")?;
        }
        for env in self.envs.iter() {
            writeln!(f, r#"Environment="{env}""#)?;
        }
        writeln!(f, "ExecStart={}", self.exec_start.join(" "))?;
        writeln!(f, "Restart={}", self.restart_policy)?;
        writeln!(f, "RestartSec={}", self.restart_sec)?;
        writeln!(f, "Type={}", self.ty)
    }
}

impl<'a> ServiceConfiguration<'a> {
    pub fn builder() -> ServiceConfigurationBuilder<'a> {
        ServiceConfigurationBuilder::default()
    }
}

pub struct ServiceConfigurationBuilder<'a> {
    pub ty: ServiceType,
    pub notify_access: Option<NotifyAccess>,
    pub exec_start: Vec<&'a str>,
    pub restart_policy: RestartPolicy,
    pub restart_sec: &'a str,
    pub working_directory: Option<&'a str>,
    pub user: Option<&'a str>,
    pub group: Option<&'a str>,
    pub envs: Vec<EnvironmentVariable<'a>>,
}

impl<'a> Default for ServiceConfigurationBuilder<'a> {
    fn default() -> Self {
        ServiceConfigurationBuilder {
            ty: ServiceType::Simple,
            notify_access: None,
            exec_start: vec![],
            restart_policy: RestartPolicy::No,
            restart_sec: "100ms",
            working_directory: None,
            user: None,
            group: None,
            envs: vec![],
        }
    }
}

impl<'a> ServiceConfigurationBuilder<'a> {
    pub fn ty(mut self, ty: ServiceType) -> Self {
        self.ty = ty;
        self
    }

    pub fn notify_access(mut self, notify_access: NotifyAccess) -> Self {
        self.notify_access = Some(notify_access);
        self
    }

    pub fn exec_start(mut self, exec_start: Vec<&'a str>) -> Self {
        self.exec_start = exec_start;
        self
    }

    pub fn restart_policy(mut self, restart_policy: RestartPolicy) -> Self {
        self.restart_policy = restart_policy;
        self
    }

    pub fn restart_sec(mut self, restart_sec: &'a str) -> Self {
        self.restart_sec = restart_sec;
        self
    }

    pub fn working_directory(mut self, working_directory: &'a str) -> Self {
        self.working_directory = Some(working_directory);
        self
    }

    pub fn user(mut self, user: &'a str) -> Self {
        self.user = Some(user);
        self
    }

    pub fn group(mut self, group: &'a str) -> Self {
        self.group = Some(group);
        self
    }

    pub fn env(mut self, key: &'a str, value: &'a str) -> Self {
        self.envs
            .push(EnvironmentVariable::builder().key(key).value(value).build());
        self
    }

    pub fn build(self) -> ServiceConfiguration<'a> {
        let ty = self.ty;
        let notify_access = self.notify_access;
        let exec_start = self.exec_start;
        let restart_policy = self.restart_policy;
        let restart_sec = self.restart_sec;
        let working_directory = self.working_directory;
        let user = self.user;
        let group = self.group;
        let envs = self.envs;
        ServiceConfiguration {
            ty,
            notify_access,
            exec_start,
            restart_policy,
            restart_sec,
            working_directory,
            user,
            group,
            envs,
        }
    }
}

pub struct InstallConfiguration<'a> {
    pub wanted_by: Vec<&'a str>,
}

impl<'a> Display for InstallConfiguration<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[Install]")?;
        writeln!(f, "WantedBy={}", self.wanted_by.join(" "))
    }
}

impl<'a> InstallConfiguration<'a> {
    pub fn builder() -> InstallConfigurationBuilder<'a> {
        InstallConfigurationBuilder::default()
    }
}

#[derive(Default)]
pub struct InstallConfigurationBuilder<'a> {
    pub wanted_by: Vec<&'a str>,
}

impl<'a> InstallConfigurationBuilder<'a> {
    pub fn wanted_by(mut self, wanted_by: &'a str) -> Self {
        self.wanted_by.push(wanted_by);
        self
    }

    pub fn build(self) -> InstallConfiguration<'a> {
        let wanted_by = self.wanted_by;
        InstallConfiguration { wanted_by }
    }
}

// https://www.freedesktop.org/software/systemd/man/systemd.service.html#Service%20Templates
pub struct ServiceUnitConfiguration<'a> {
    pub unit: UnitConfiguration<'a>,
    pub service: ServiceConfiguration<'a>,
    pub install: InstallConfiguration<'a>,
}

impl<'a> Display for ServiceUnitConfiguration<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}\n{}", self.unit, self.service, self.install)
    }
}

impl<'a> ServiceUnitConfiguration<'a> {
    pub fn builder() -> ServiceUnitConfigurationBuilder<'a> {
        ServiceUnitConfigurationBuilder::default()
    }
}

#[derive(Default)]
pub struct ServiceUnitConfigurationBuilder<'a> {
    pub unit: UnitConfigurationBuilder<'a>,
    pub service: ServiceConfigurationBuilder<'a>,
    pub install: InstallConfigurationBuilder<'a>,
}

impl<'a> ServiceUnitConfigurationBuilder<'a> {
    pub fn unit(mut self, unit: UnitConfigurationBuilder<'a>) -> Self {
        self.unit = unit;
        self
    }

    pub fn service(mut self, service: ServiceConfigurationBuilder<'a>) -> Self {
        self.service = service;
        self
    }

    pub fn install(mut self, install: InstallConfigurationBuilder<'a>) -> Self {
        self.install = install;
        self
    }

    pub fn build(self) -> ServiceUnitConfiguration<'a> {
        let unit = self.unit.build();
        let service = self.service.build();
        let install = self.install.build();
        ServiceUnitConfiguration {
            unit,
            service,
            install,
        }
    }
}

#[derive(Default)]
pub struct SocketConfigurationBuilder<'a> {
    pub listen_stream: Vec<&'a str>,
    pub listen_datagram: Vec<&'a str>,
    pub service: Option<&'a str>,
    pub file_descriptor_name: Option<&'a str>,
    pub free_bind: Option<bool>,
    pub unit: UnitConfigurationBuilder<'a>,
    pub install: InstallConfigurationBuilder<'a>,
}

impl<'a> SocketConfigurationBuilder<'a> {
    pub fn listen_stream(mut self, listen_stream: &'a str) -> Self {
        self.listen_stream.push(listen_stream);
        self
    }

    pub fn listen_datagram(mut self, listen_datagram: &'a str) -> Self {
        self.listen_datagram.push(listen_datagram);
        self
    }

    pub fn service(mut self, service: &'a str) -> Self {
        self.service = Some(service);
        self
    }

    pub fn install(mut self, install: InstallConfigurationBuilder<'a>) -> Self {
        self.install = install;
        self
    }

    pub fn build(self) -> SocketConfiguration<'a> {
        SocketConfiguration {
            listen_stream: self.listen_stream,
            listen_datagram: self.listen_datagram,
            service: self.service,
            file_descriptor_name: self.file_descriptor_name,
            free_bind: self.free_bind,
            unit: self.unit.build(),
            install: self.install.build(),
        }
    }
}

pub struct SocketConfiguration<'a> {
    pub listen_stream: Vec<&'a str>,
    pub listen_datagram: Vec<&'a str>,
    pub service: Option<&'a str>,
    pub file_descriptor_name: Option<&'a str>,
    pub free_bind: Option<bool>,
    pub unit: UnitConfiguration<'a>,
    pub install: InstallConfiguration<'a>,
}

impl<'a> SocketConfiguration<'a> {
    pub fn builder() -> SocketConfigurationBuilder<'a> {
        SocketConfigurationBuilder::default()
    }
}

impl<'a> Display for SocketConfiguration<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.unit)?;
        writeln!(f)?;
        writeln!(f, "[Socket]")?;

        for listen_stream in &self.listen_stream {
            writeln!(f, "ListenStream={listen_stream}",)?;
        }
        for listen_datagram in &self.listen_datagram {
            writeln!(f, "ListenDatagram={listen_datagram}")?;
        }
        if let Some(service) = self.service {
            writeln!(f, "Service={service}")?;
        }
        if let Some(free_bind) = self.free_bind {
            writeln!(f, "FreeBind={free_bind}")?;
        }

        writeln!(f)?;
        writeln!(f, "{}", self.install)
    }
}
