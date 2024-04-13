#[derive(Debug, PartialEq, Clone)]
pub struct MyGameInfo {
    pub current_group_id: Option<i64>,
    pub current_level_id: Option<i64>,
}

impl MyGameInfo {
    pub const NONE: MyGameInfo = MyGameInfo {
        current_group_id: None,
        current_level_id: None,
    };
    pub fn get_library_path(&self, lib: &my_levels_finder::CollectedGame) -> Option<String> {
        if let Some(group_id) = self.current_group_id {
            if let Some(level_id) = self.current_level_id {
                if let Some(group) = lib.groups.get(&group_id) {
                    if let Some(level) = group.levels.get(&level_id) {
                        return Some(level.link.path().clone());
                    }
                }
            }
        }
        None
    }
}
