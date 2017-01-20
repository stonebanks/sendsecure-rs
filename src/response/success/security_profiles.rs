use helpers::securityprofile;

#[derive(RustcDecodable, Debug)]
pub struct SecurityProfiles {
    pub security_profiles: Vec<securityprofile::SecurityProfile>,
}
