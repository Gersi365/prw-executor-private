//! Narrow first-production Ubuntu device-identity provisioning executable.

use std::process::ExitCode;

use prw_device_identity_provisioning::provision_first_ubuntu_device_identity;

fn main() -> ExitCode {
    if std::env::args_os().nth(1).is_some() {
        eprintln!("prw-device-identity-provisioning result=failure kind=invalid_arguments");
        return ExitCode::FAILURE;
    }

    match provision_first_ubuntu_device_identity() {
        Ok(result) => {
            println!(
                "prw-device-identity-provisioning result=success public_spki_sha256={} ciphertext_path={}",
                result.public_spki_sha256(),
                result.encrypted_credential_path().display()
            );
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!(
                "prw-device-identity-provisioning result=failure kind={}",
                error.token()
            );
            ExitCode::FAILURE
        }
    }
}
