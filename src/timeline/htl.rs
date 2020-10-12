use crate::{
    http::connection::{self, Toot},
    initialize::initialize::InstanceSetting,
};

pub fn get_toots(
    instance_setting: &InstanceSetting,
) -> Result<Vec<Toot>, Box<dyn std::error::Error>> {
    let toots = connection::get_toot(
        &instance_setting.host_name,
        &instance_setting.access_token
    );
    toots
}
