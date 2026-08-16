use std::process::ExitCode;

use prw_device_identity_provisioning::{
    provision_first_ubuntu_device_identity, sha256_hex,
};

fn main() -> ExitCode {
    match provision_first_ubuntu_device_identity() {
        Ok(provisioned) => {
            println!(
                "prw-device-identity-provision event=provisioned public_spki_sha256={} encrypted_credential_path={}",
                sha256_hex(provisioned.public_spki_sha256()),
                provisioned.encrypted_credential_path().display()
            );
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!(
                "prw-device-identity-provision event=failed kind={} exit=failure",
                error
            );
            ExitCode::FAILURE
        }
    }
}
