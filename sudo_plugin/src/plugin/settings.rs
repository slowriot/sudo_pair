// Copyright 2018 Square Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or
// implied. See the License for the specific language governing
// permissions and limitations under the License.

use super::super::errors::*;
use super::option_map::*;
use super::traits::*;

use std::net::{AddrParseError, IpAddr};
use std::str;

#[derive(Debug)]
pub struct Settings {
    pub bsd_auth_type:        Option<String>,
    pub close_from:           Option<u64>,
    pub debug_flags:          Option<String>,
    pub debug_level:          Option<u64>,
    pub ignore_ticket:        bool,
    pub implied_shell:        bool,
    pub login_class:          Option<String>,
    pub login_shell:          bool,
    pub max_groups:           Option<u64>,
    pub network_addrs:        Vec<NetAddr>,
    pub noninteractive:       bool,
    pub plugin_dir:           String,
    pub plugin_path:          String,
    pub preserve_environment: bool,
    pub preserve_groups:      bool,
    pub progname:             String,
    pub prompt:               Option<String>,
    pub remote_host:          Option<String>,
    pub run_shell:            bool,
    pub runas_group:          Option<String>,
    pub runas_user:           Option<String>,
    pub selinux_role:         Option<String>,
    pub selinux_type:         Option<String>,
    pub set_home:             bool,
    pub sudoedit:             bool,

    pub raw: OptionMap,
}

impl Settings {
    pub fn new(raw: OptionMap) -> Result<Self> {
        Ok(Self {
            plugin_dir:  raw.get("plugin_dir")?,
            plugin_path: raw.get("plugin_path")?,
            progname:    raw.get("progname")?,

            bsd_auth_type:        raw.get("bsd_auth_type")       .ok(),
            close_from:           raw.get("closefrom")           .ok(),
            debug_flags:          raw.get("debug_flags")         .ok(),
            debug_level:          raw.get("debug_level")         .ok(),
            ignore_ticket:        raw.get("ignore_ticket")       .unwrap_or(false),
            implied_shell:        raw.get("implied_shell")       .unwrap_or(false),
            login_class:          raw.get("login_class")         .ok(),
            login_shell:          raw.get("login_shell")         .unwrap_or(false),
            max_groups:           raw.get("max_groups")          .ok(),
            network_addrs:        raw.get("network_addrs")       .unwrap_or_else(|_| vec![]),
            noninteractive:       raw.get("noninteractive")      .unwrap_or(false),
            preserve_environment: raw.get("preserve_environment").unwrap_or(false),
            preserve_groups:      raw.get("preserve_groups")     .unwrap_or(false),
            prompt:               raw.get("prompt")              .ok(),
            remote_host:          raw.get("remote_host")         .ok(),
            run_shell:            raw.get("run_shell")           .unwrap_or(false),
            runas_group:          raw.get("runas_group")         .ok(),
            runas_user:           raw.get("runas_user")          .ok(),
            selinux_role:         raw.get("selinux_role")        .ok(),
            selinux_type:         raw.get("selinux_type")        .ok(),
            set_home:             raw.get("set_home")            .unwrap_or(false),
            sudoedit:             raw.get("sudoedit")            .unwrap_or(false),

            raw,
        })
    }

    pub fn flags(&self) -> Vec<Vec<u8>> {
        let mut flags : Vec<Vec<u8>> = vec![];

        if let Some(ref runas_user) = self.runas_user {
            flags.push(b"-u".to_vec());
            flags.push(runas_user.as_bytes().to_vec());
        }

        if let Some(ref runas_group) = self.runas_group {
            flags.push(b"-g".to_vec());
            flags.push(runas_group.as_bytes().to_vec());
        }

        if self.login_shell {
            flags.push(b"-i".to_vec());
        }

        if self.run_shell {
            flags.push(b"-s".to_vec());
        }

        flags
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NetAddr {
    pub addr: IpAddr,
    pub mask: IpAddr,
}

impl FromSudoOption for NetAddr {
    type Err = AddrParseError;

    fn from_sudo_option(s: &str) -> ::std::result::Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        let mid   = bytes.iter()
            .position(|b| *b == b'/' )
            .unwrap_or_else(|| bytes.len());

        let addr = s[        .. mid].parse()?;
        let mask = s[mid + 1 ..    ].parse()?;

        Ok(Self {
            addr,
            mask,
        })
    }
}

impl FromSudoOptionList for NetAddr {
    const SEPARATOR: char = ' ';
}
