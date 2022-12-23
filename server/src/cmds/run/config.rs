// This file is part of Moonfire NVR, a security camera network video recorder.
// Copyright (C) 2022 The Moonfire NVR Authors; see AUTHORS and LICENSE.txt.
// SPDX-License-Identifier: GPL-v3.0-or-later WITH GPL-3.0-linking-exception.

//! Runtime configuration file (`/etc/moonfire-nvr.toml`).

use std::path::PathBuf;

use serde::Deserialize;

fn default_db_dir() -> PathBuf {
    "/var/lib/moonfire-nvr/db".into()
}

fn default_ui_dir() -> PathBuf {
    "/usr/local/lib/moonfire-nvr/ui".into()
}

/// Top-level configuration file object.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConfigFile {
    pub binds: Vec<BindConfig>,

    /// Directory holding the SQLite3 index database.
    #[serde(default = "default_db_dir")]
    pub db_dir: PathBuf,

    /// Directory holding user interface files (`.html`, `.js`, etc).
    #[serde(default = "default_ui_dir")]
    pub ui_dir: PathBuf,

    /// The number of worker threads used by the asynchronous runtime.
    ///
    /// Defaults to the number of cores on the system.
    #[serde(default)]
    pub worker_threads: Option<usize>,
}

/// Per-bind configuration.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BindConfig {
    /// The address to bind to.
    #[serde(flatten)]
    pub address: AddressConfig,

    /// Allow unauthenticated API access on this bind, with the given
    /// permissions (defaults to empty).
    ///
    /// Note that even an empty string allows some basic access that would be rejected if the
    /// argument were omitted.
    #[serde(default)]
    pub allow_unauthenticated_permissions: Option<Permissions>,

    /// Trusts `X-Real-IP:` and `X-Forwarded-Proto:` headers on the incoming request.
    ///
    /// Set this only after ensuring your proxy server is configured to set them
    /// and that no untrusted requests bypass the proxy server. You may want to
    /// specify a localhost bind address.
    #[serde(default)]
    pub trust_forward_headers: bool,

    /// On Unix-domain sockets, treat clients with the Moonfire NVR server's own
    /// effective UID as privileged.
    #[serde(default)]
    pub own_uid_is_privileged: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
#[serde(deny_unknown_fields)]
pub enum AddressConfig {
    /// IPv4 address such as `0.0.0.0:8080` or `127.0.0.1:8080`.
    Ipv4(std::net::SocketAddrV4),

    /// IPv6 address such as `[::]:8080` or `[::1]:8080`.
    Ipv6(std::net::SocketAddrV6),

    /// Unix socket path such as `/var/lib/moonfire-nvr/sock`.
    Unix(PathBuf),
    // TODO: SystemdFileDescriptorName(String), see
    // https://www.freedesktop.org/software/systemd/man/systemd.socket.html
}

/// JSON analog of `Permissions` defined in `db/proto/schema.proto`.
#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Permissions {
    #[serde(default)]
    view_video: bool,

    #[serde(default)]
    read_camera_configs: bool,

    #[serde(default)]
    update_signals: bool,

    #[serde(default)]
    admin_users: bool,
}

impl Permissions {
    pub fn as_proto(&self) -> db::schema::Permissions {
        db::schema::Permissions {
            view_video: self.view_video,
            read_camera_configs: self.read_camera_configs,
            update_signals: self.update_signals,
            admin_users: self.admin_users,
            ..Default::default()
        }
    }
}
