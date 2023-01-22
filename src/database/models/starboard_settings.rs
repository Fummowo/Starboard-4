#[derive(Clone, Debug)]
pub struct StarboardSettings {
    // General Style
    pub display_emoji: Option<String>,
    pub ping_author: bool,
    pub use_server_profile: bool,
    pub extra_embeds: bool,
    pub use_webhook: bool,

    // Embed Style
    pub color: Option<i32>,
    pub jump_to_message: bool,
    pub attachments_list: bool,
    pub replied_to: bool,

    // Requirements
    pub required: i16,
    pub required_remove: i16,
    pub upvote_emojis: Vec<String>,
    pub downvote_emojis: Vec<String>,
    pub self_vote: bool,
    pub allow_bots: bool,
    pub require_image: bool,
    pub older_than: i64,
    pub newer_than: i64,

    // Behavior
    pub enabled: bool,
    pub autoreact_upvote: bool,
    pub autoreact_downvote: bool,
    pub remove_invalid_reactions: bool,
    pub link_deletes: bool,
    pub link_edits: bool,
    pub private: bool,
    pub xp_multiplier: f32,
    pub cooldown_enabled: bool,
    pub cooldown_count: i16,
    pub cooldown_period: i16,
    pub exclusive_group: Option<i32>,
    pub exclusive_group_priority: i16,
}

macro_rules! settings_from_record {
    ($has_settings: expr, $($name: ident),*) => {{
        use crate::database::models::starboard_settings::StarboardSettings;
        StarboardSettings {
            $(
                $name: $has_settings.$name,
            )*
        }
    }};
}
macro_rules! settings_from_row {
    ($has_settings: expr, $($name: ident),*) => {
        StarboardSettings {
            $(
                $name: $has_settings.get(stringify!($name)),
            )*
        }
    };
}

pub(crate) use settings_from_record;
pub(crate) use settings_from_row;
