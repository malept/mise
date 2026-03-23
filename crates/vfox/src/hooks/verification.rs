use crate::hooks::pre_install::PreInstallAttestation;

/// Fields needed by `Vfox::verify()` to check checksums and attestation.
/// Shared between traditional plugins (`PreInstall`) and backend plugins
/// (`BackendPreInstallResponse`).
#[derive(Debug, Default)]
pub struct VerificationParams {
    pub sha256: Option<String>,
    pub sha512: Option<String>,
    pub sha1: Option<String>,
    pub md5: Option<String>,
    pub attestation: Option<PreInstallAttestation>,
}
