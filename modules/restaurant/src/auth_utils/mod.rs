pub fn login(creds: modules::Credentials) {
    crate::database::get_user();
}
fn logout() {}
pub mod modules;

