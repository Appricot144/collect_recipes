use std::io::{BufRead, BufReader};
use std::fs::File;
use crate::block::{
	Block,
	StatementBlock,
	ResourceBlock,
	IfBlock,
	CaseBlock,
	Property, UnknownBlock,
};

pub enum Status {
	Signature,
	Contents,
}

pub struct Parser<'a> {
	pub file_name: String,
	pub reader: BufReader<&'a std::fs::File>,
	pub block: Block,
	pub status: Status,
	pub position: (u32, u32),
}

impl<'a> Parser<'a> {
	pub fn new(f: &File, f_name: String) -> Parser {
		Parser {
			file_name: f_name,
			reader: BufReader::new(f),
			block: Block::Eof,
			status: Status::Signature,
			position: (0, 0),
		}
	}

	pub fn next_line(&mut self) -> String {
		let mut line = String::new();
		self.position.0 += self.position.1; //position start line set

		loop {
			let num_bytes = self.reader.read_line(&mut line).expect("failed to read line");
			self.position.1 += 1; 			//position end line ++

			//ret EoF
			if num_bytes == 0 {
				return line
			}


			//strip Comment-out	
			for (i, l) in line.as_str().chars().enumerate() {
				if l == '#' {
					line = (&line[..i]).to_string();
					break
				}
			}

			//skip Blank-line, space, tab
			let white: &[_] = &[' ', '\t', '\n', '\r'];
			line = line.trim_start_matches(white).to_string();
			if line.len() == 0 { continue }

			return line
		}
	}
	
	// do_parse
	// BufからBlockを作る
	// Buffer(8KB)を読み切ったら次の8KBをBufに読む
	// とりあえず、Buf内にファイルが全て入るとして進める
	pub fn do_parse(mut self) -> Parser<'a> {

		let line = self.next_line();
		if line == "" { self.block = Block::Eof; return self } //EOF
		let words: Vec<&str> = line.as_str().split_whitespace().collect();

		// Is there "do" ?
		let mut do_flag: bool = false;
		let ws = words.to_vec();
		for word in ws {
			if word == "do" {
				do_flag = true;
				break;
			}
		}
		
		// Resource or ... ?
		if Parser::is_resource(&words[0]) {
			if !do_flag {
				self.block = ResourceBlock::create_def_block(
					words,
					self.file_name.clone(),
					self.position.1
				);
				self
			} else {
				ResourceBlock::create_block(words, self)
			}
		} else if do_flag {
			StatementBlock::create_block(words, self)
		} else if words[0] == "if" {
			let if_block: IfBlock = IfBlock::new();
			if_block.create_whole_if_block(&words, self)
		} else if words[0] == "case" {
			let case_block: CaseBlock = CaseBlock::new();
			case_block.create_whole_case_block(&words, self)
		} else if words[0] == "elsif" {
			self.block = Block::Elsif(vec![words[0].to_string(),words[1].to_string()]);
			self
		} else if words[0] == "else" {
			self.block = Block::Else;
			self
		} else if words[0] == "when" {
			self.block = Block::When(vec![words[0].to_string(),words[1].to_string()]);
			self
		} else if words[0] == "end" {
			self.status = Status::Signature;
			self.block = Block::End;
			self
		} else {
			if let Status::Contents = self.status {
				self.block = Property::create_property(words);
				self
			} else { 		// 読めない構文
				self.block = UnknownBlock::create_block(words);
				self
			}
		}
	}

	fn is_resource(word: &str) -> bool {
		let resources = [
			"alternatives",
			"apt_package",
			"apt_preference",
			"apt_repository",
			"apt_update",
			"archive_file",
			"bash",
			"batch",
			"bff_package",
			"breakpoint",
			"build_essential",
			"cab_package",
			"chef_acl",
			"chef_client",
			"chef_client_config",
			"chef_client_cron",
			"chef_client_launchd",
			"chef_client_scheduled_task",
			"chef_client_systemd_timer",
			"chef_client_trusted_certificate",
			"chef_container",
			"chef_data_bag",
			"chef_data_bag_item",
			"chef_environment",
			"chef_gem",
			"chef_group",
			"chef_handler",
			"chef_node",
			"chef_organization",
			"chef_role",
			"chef_sleep",
			"chef_user",
			"chef_vault_secret",
			"chocolatey_config",
			"chocolatey_feature",
			"chocolatey_package",
			"chocolatey_source",
			"cookbook_file",
			"cron",
			"cron_access",
			"cron_d",
			"csh",
			"directory",
			"dmg_package",
			"dnf_package",
			"dpkg_package",
			"dsc_resource",
			"dsc_script",
			"execute",
			"file",
			"freebsd_package",
			"gem_package",
			"git",
			"group",
			"habitat_config",
			"habitat_install",
			"habitat_package",
			"habitat_service",
			"habitat_sup",
			"habitat_user_toml",
			"homebrew_cask",
			"homebrew_package",
			"homebrew_tap",
			"homebrew_update",
			"hostname",
			"http_request",
			"ifconfig",
			"inspec_input",
			"inspec_waiver",
			"inspec_waiver_file_entry",
			"ips_package",
			"kernel_module",
			"ksh",
			"launchd",
			"link",
			"locale",
			"log",
			"macos_userdefaults",
			"macports_package",
			"mdadm",
			"mount",
			"msu_package",
			"notify_group",
			"ohai",
			"ohai_hint",
			"openbsd_package",
			"openssl_dhparam",
			"openssl_ec_private_key",
			"openssl_ec_public_key",
			"openssl_rsa_private_key",
			"openssl_rsa_public_key",
			"openssl_x509_certificate",
			"openssl_x509_crl",
			"openssl_x509_request",
			"osx_profile",
			"package",
			"pacman_package",
			"paludis_package",
			"perl",
			"plist",
			"portage_package",
			"powershell_package",
			"powershell_package_source",
			"powershell_script",
			"python",
			"reboot",
			"registry_key",
			"remote_directory",
			"remote_file",
			"rhsm_errata",
			"rhsm_errata_level",
			"rhsm_register",
			"rhsm_repo",
			"rhsm_subscription",
			"route",
			"rpm_package",
			"ruby",
			"ruby_block",
			"script",
			"selinux_boolean",
			"selinux_fcontext",
			"selinux_install",
			"selinux_module",
			"selinux_permissive",
			"selinux_port",
			"selinux_state",
			"service",
			"smartos_package",
			"snap_package",
			"solaris_package",
			"ssh_known_hosts_entry",
			"subversion",
			"sudo",
			"swap_file",
			"sysctl",
			"systemd_unit",
			"template",
			"timezone",
			"user",
			"user_ulimit",
			"windows_ad_join",
			"windows_audit_policy",
			"windows_auto_run",
			"windows_certificate",
			"windows_defender",
			"windows_defender_exclusion",
			"windows_dfs_folder",
			"windows_dfs_namespace",
			"windows_dfs_server",
			"windows_dns_record",
			"windows_dns_zone",
			"windows_env",
			"windows_feature",
			"windows_feature_dism",
			"windows_feature_powershell",
			"windows_firewall_profile",
			"windows_firewall_rule",
			"windows_font",
			"windows_package",
			"windows_pagefile",
			"windows_path",
			"windows_printer",
			"windows_printer_port",
			"windows_security_policy",
			"windows_service",
			"windows_share",
			"windows_shortcut",
			"windows_task",
			"windows_uac",
			"windows_update_settings",
			"windows_user_privilege",
			"windows_workgroup",
			"yum_package",
			"yum_repository",
			"zypper_package",
			"zypper_repository",
		];
		
		for resource_name in resources {
			if word == resource_name {
				return true
			} 
		}
		false
	}

}
