//! # rpm-rs
//!
//! A library providing API to parse rpms as well as
//! creating rpms from individual files.
//!
//! # Example
//!
//! ```rust
//!
//! use rpm::{
//! 	signature::pgp::{
//! 		Signer,
//! 		Verifier
//! 	}
//! };
//! use std::str::FromStr;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let raw_secret_key = std::fs::read("./test_assets/secret_key.asc")?;
//! let pkg = rpm::RPMBuilder::new("test", "1.0.0", "MIT", "x86_64", "some awesome package")
//!             .compression(rpm::Compressor::from_str("gzip")?)
//!             .with_file(
//!                 "./test_assets/awesome.toml",
//!                 rpm::RPMFileOptions::new("/etc/awesome/config.toml")
//! 					.is_config(),
//!             )?
//!             // file mode is inherited from source file
//!             .with_file(
//!                 "./test_assets/awesome.py",
//!                 rpm::RPMFileOptions::new("/usr/bin/awesome"),
//!             )?
//!              .with_file(
//!                 "./test_assets/awesome.toml",
//!                 // you can set a custom mode and custom user too
//!                 rpm::RPMFileOptions::new("/etc/awesome/second.toml")
//! 						.mode(0o100744)
//! 						.user("hugo"),
//!             )?
//!             .pre_install_script("echo preinst")
//!             .add_changelog_entry("me", "was awesome, eh?", 123123123)
//!             .add_changelog_entry("you", "yeah, it was", 12312312)
//!             .requires(rpm::Dependency::any("wget"))
//!             .build_and_sign(
//! 				Signer::load_from_asc_bytes(&raw_secret_key)?
//! 			)?;
//! let mut f = std::fs::File::create("./target/awesome.rpm")?;
//! pkg.write(&mut f)?;
//!
//! // reading
//! let raw_pub_key = std::fs::read("./test_assets/public_key.asc")?;
//! let rpm_file = std::fs::File::open("./target/awesome.rpm")?;
//! let mut buf_reader = std::io::BufReader::new(rpm_file);
//! let pkg = rpm::RPMPackage::parse(&mut buf_reader)?;
//! // verifying
//! pkg.verify_signature(Verifier::load_from_asc_bytes(&raw_pub_key)?)?;
//! # Ok(())
//! # }
//! ```

mod constants;
mod errors;
mod rpm;
mod sequential_cursor;

#[cfg(feature = "signature-meta")]
pub use crate::rpm::{Empty, SignatureHeaderBuilder, WithDigest, WithSignature};
pub(crate) use crate::rpm::{IndexData, IndexEntry};
pub use crate::{
    constants::*,
    errors::RPMError,
    rpm::{
        signature, Compressor, Dependency, FileCategory, FileDigest, FileDigestAlgorithm, FileEntry, FileMode,
        FileOwnership, Header, Lead, RPMBuilder, RPMFileEntry, RPMFileOptions, RPMFileOptionsBuilder, RPMPackage,
        RPMPackageMetadata,
    },
};

#[cfg(test)]
mod tests;

#[cfg(all(test, any(feature = "test-with-podman", feature = "test-with-docker")))]
mod compat_tests;
