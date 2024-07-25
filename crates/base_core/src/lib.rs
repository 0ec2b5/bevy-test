pub mod cursor;
pub mod enemy;
pub mod menu;
pub mod player;
pub mod quick_close;

pub mod prelude {
    pub use crate::{
        enemy::EnemyPlugin, menu::MenuPlugin, player::PlayerPlugin, quick_close::QuickClosePlugin,
    };
}
