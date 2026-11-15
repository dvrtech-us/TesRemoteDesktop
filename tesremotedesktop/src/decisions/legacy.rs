//! Names from the previous product. Read only so an installed machine can be
//! upgraded. New code does not invent more of these strings.

/// Shared by Windows and Linux: the database file name in the old data folder.
pub const DB_FILE: &str = "teslapc.db";

/// Still accepted on the command line as an alias of `--no-bypass`.
pub const NO_BYPASS_FLAG: &str = "--no-tesla-bypass";

pub mod windows {
    /// Service Control Manager name of the previous service.
    pub const SERVICE_NAME: &str = "TeslaPC";
    /// `%ProgramData%` child directory the previous build used.
    pub const DATA_DIR: &str = "TeslaPC";
    /// Image name of the previous executable, matched by the process scan and
    /// the scheduled-task warning.
    pub const EXE_NAME: &str = "teslapc.exe";
}

pub mod linux {
    /// `$XDG_CONFIG_HOME` child directory the previous build used.
    pub const CONFIG_DIR: &str = "teslapc";
    /// `$XDG_DATA_HOME` child directory the previous build used.
    pub const DATA_DIR: &str = "teslapc";
    /// Unit file the previous `service install` wrote under `/etc/systemd/system/`.
    pub const UNIT_FILE: &str = "teslapc.service";
    /// nftables table the previous bypass created (`ip teslapc`).
    pub const NFT_TABLE: &str = "teslapc";
    /// Printed, never run, when the previous unit file is still on disk.
    pub const DISABLE_COMMAND: &str = "systemctl disable --now teslapc";
}
