use anyhow::Ok;

use crate::setting::AppSetting;

pub fn get_group_names() -> Vec<String> {
    let app_setting = AppSetting::get_instance();
    app_setting.groups.iter().map(|g| g.name.clone()).collect()
}

pub fn parse_group(s: &str) -> anyhow::Result<Vec<String>> {
    let group_names = get_group_names();

    if group_names.contains(&s.to_string()) {
        Ok(group_names)
    } else {
        anyhow::bail!("Invalid group. Valid options: {:?}", group_names)
    }
}
