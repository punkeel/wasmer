use crate::wasi::extra::*;

impl Rights {
    pub const fn all_socket() -> Self {
        Self::from_bits_truncate(
            Self::FD_FDSTAT_SET_FLAGS.bits()
                | Self::FD_FILESTAT_GET.bits()
                | Self::FD_READ.bits()
                | Self::FD_WRITE.bits()
                | Self::POLL_FD_READWRITE.bits()
                | Self::SOCK_SHUTDOWN.bits()
                | Self::SOCK_CONNECT.bits()
                | Self::SOCK_LISTEN.bits()
                | Self::SOCK_BIND.bits()
                | Self::SOCK_ACCEPT.bits()
                | Self::SOCK_RECV.bits()
                | Self::SOCK_SEND.bits()
                | Self::SOCK_ADDR_LOCAL.bits()
                | Self::SOCK_ADDR_REMOTE.bits()
                | Self::SOCK_RECV_FROM.bits()
                | Self::SOCK_SEND_TO.bits(),
        )
    }

    /// expects a single right, returns None if out of bounds or > 1 bit set
    pub fn to_str(self) -> Option<&'static str> {
        Some(match self {
            Rights::FD_DATASYNC => "Rights::FD_DATASYNC",
            Rights::FD_READ => "Rights::FD_READ",
            Rights::FD_SEEK => "Rights::FD_SEEK",
            Rights::FD_FDSTAT_SET_FLAGS => "Rights::FD_FDSTAT_SET_FLAGS",
            Rights::FD_SYNC => "Rights::FD_SYNC",
            Rights::FD_TELL => "Rights::FD_TELL",
            Rights::FD_WRITE => "Rights::FD_WRITE",
            Rights::FD_ADVISE => "Rights::FD_ADVISE",
            Rights::FD_ALLOCATE => "Rights::FD_ALLOCATE",
            Rights::PATH_CREATE_DIRECTORY => "Rights::PATH_CREATE_DIRECTORY",
            Rights::PATH_CREATE_FILE => "Rights::PATH_CREATE_FILE",
            Rights::PATH_LINK_SOURCE => "Rights::PATH_LINK_SOURCE",
            Rights::PATH_LINK_TARGET => "Rights::PATH_LINK_TARGET",
            Rights::PATH_OPEN => "Rights::PATH_OPEN",
            Rights::FD_READDIR => "Rights::FD_READDIR",
            Rights::PATH_READLINK => "Rights::PATH_READLINK",
            Rights::PATH_RENAME_SOURCE => "Rights::PATH_RENAME_SOURCE",
            Rights::PATH_RENAME_TARGET => "Rights::PATH_RENAME_TARGET",
            Rights::PATH_FILESTAT_GET => "Rights::PATH_FILESTAT_GET",
            Rights::PATH_FILESTAT_SET_SIZE => "Rights::PATH_FILESTAT_SET_SIZE",
            Rights::PATH_FILESTAT_SET_TIMES => "Rights::PATH_FILESTAT_SET_TIMES",
            Rights::FD_FILESTAT_GET => "Rights::FD_FILESTAT_GET",
            Rights::FD_FILESTAT_SET_SIZE => "Rights::FD_FILESTAT_SET_SIZE",
            Rights::FD_FILESTAT_SET_TIMES => "Rights::FD_FILESTAT_SET_TIMES",
            Rights::PATH_SYMLINK => "Rights::PATH_SYMLINK",
            Rights::PATH_REMOVE_DIRECTORY => "Rights::PATH_REMOVE_DIRECTORY",
            Rights::PATH_UNLINK_FILE => "Rights::PATH_UNLINK_FILE",
            Rights::POLL_FD_READWRITE => "Rights::POLL_FD_READWRITE",
            Rights::SOCK_SHUTDOWN => "Rights::SOCK_SHUTDOWN",
            Rights::SOCK_ACCEPT => "Rights::SOCK_ACCEPT",
            Rights::SOCK_CONNECT => "Rights::SOCK_CONNECT",
            Rights::SOCK_LISTEN => "Rights::SOCK_LISTEN",
            Rights::SOCK_BIND => "Rights::SOCK_BIND",
            Rights::SOCK_RECV => "Rights::SOCK_RECV",
            Rights::SOCK_SEND => "Rights::SOCK_SEND",
            Rights::SOCK_ADDR_LOCAL => "Rights::SOCK_ADDR_LOCAL",
            Rights::SOCK_ADDR_REMOTE => "Rights::SOCK_ADDR_REMOTE",
            Rights::SOCK_RECV_FROM => "Rights::SOCK_RECV_FROM",
            Rights::SOCK_SEND_TO => "Rights::SOCK_SEND_TO",
            _ => return None,
        })
    }
}

impl Default for Filestat {
    fn default() -> Self {
        Self {
            st_dev: Default::default(),
            st_ino: Default::default(),
            st_filetype: Filetype::Unknown,
            st_nlink: 1,
            st_size: Default::default(),
            st_atim: Default::default(),
            st_mtim: Default::default(),
            st_ctim: Default::default(),
        }
    }
}

/// Workaround implementation because `wit-bindgen` does not generate
/// type aliases, but instead creates the same filetype in each module
/// for `use` statements in the `.wit` file.
impl From<Clockid> for Snapshot0Clockid {
    fn from(other: Clockid) -> Self {
        match other {
            Clockid::Realtime => Self::Realtime,
            Clockid::Monotonic => Self::Monotonic,
        }
    }
}

impl From<Snapshot0Clockid> for Clockid {
    fn from(other: Snapshot0Clockid) -> Self {
        match other {
            Snapshot0Clockid::Realtime => Self::Realtime,
            Snapshot0Clockid::Monotonic => Self::Monotonic,
            Snapshot0Clockid::ProcessCputimeId => todo!("not implemented for now"),
            Snapshot0Clockid::ThreadCputimeId => todo!("not implemented for now"),
        }
    }
}

impl From<Snapshot0SubscriptionClock> for SubscriptionClock {
    fn from(other: Snapshot0SubscriptionClock) -> Self {
        // TODO: this removes Snapshot0SubscriptionClock::identifier, I don't
        // think this is how it should be
        Self {
            clock_id: Clockid::from(other.id),
            timeout: other.timeout,
            precision: other.precision,
            flags: other.flags,
        }
    }
}

impl From<Snapshot0SubscriptionEnum> for SubscriptionEnum {
    fn from(other: Snapshot0SubscriptionEnum) -> Self {
        match other {
            Snapshot0SubscriptionEnum::Clock(d) => Self::Clock(SubscriptionClock::from(d)),
            Snapshot0SubscriptionEnum::Read(d) => Self::Read(d),
            Snapshot0SubscriptionEnum::Write(d) => Self::Write(d),
        }
    }
}

impl From<Snapshot0Subscription> for Subscription {
    fn from(other: Snapshot0Subscription) -> Self {
        Self {
            userdata: other.userdata,
            data: SubscriptionEnum::from(other.data),
        }
    }
}

impl std::fmt::Display for BusDataFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::fmt::Display for Sockoption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            Self::Noop => "Sockoption::Noop",
            Self::ReusePort => "Sockoption::ReusePort",
            Self::ReuseAddr => "Sockoption::ReuseAddr",
            Self::NoDelay => "Sockoption::NoDelay",
            Self::DontRoute => "Sockoption::DontRoute",
            Self::OnlyV6 => "Sockoption::OnlyV6",
            Self::Broadcast => "Sockoption::Broadcast",
            Self::MulticastLoopV4 => "Sockoption::MulticastLoopV4",
            Self::MulticastLoopV6 => "Sockoption::MulticastLoopV6",
            Self::Promiscuous => "Sockoption::Promiscuous",
            Self::Listening => "Sockoption::Listening",
            Self::LastError => "Sockoption::LastError",
            Self::KeepAlive => "Sockoption::KeepAlive",
            Self::Linger => "Sockoption::Linger",
            Self::OobInline => "Sockoption::OobInline",
            Self::RecvBufSize => "Sockoption::RecvBufSize",
            Self::SendBufSize => "Sockoption::SendBufSize",
            Self::RecvLowat => "Sockoption::RecvLowat",
            Self::SendLowat => "Sockoption::SendLowat",
            Self::RecvTimeout => "Sockoption::RecvTimeout",
            Self::SendTimeout => "Sockoption::SendTimeout",
            Self::ConnectTimeout => "Sockoption::ConnectTimeout",
            Self::AcceptTimeout => "Sockoption::AcceptTimeout",
            Self::Ttl => "Sockoption::Ttl",
            Self::MulticastTtlV4 => "Sockoption::MulticastTtlV4",
            Self::Type => "Sockoption::Type",
            Self::Proto => "Sockoption::Proto",
        };
        write!(f, "{}", s)
    }
}
