use mc_core::entity::SingleEntityCodec;
use mc_core::entity_component;

use crate::util::GameMode;

use nbt::CompoundTag;


#[derive(Debug, Default)]
pub struct PlayerEntity {
    /// The game mode of the player.
    game_mode: GameMode,
    /// The previous game mode of the player.
    previous_game_mode: Option<GameMode>,
    /// The Score displayed upon death.
    score: u32,
}

entity_component!(PlayerEntity: PlayerEntityCodec);

pub struct PlayerEntityCodec;
impl SingleEntityCodec for PlayerEntityCodec {

    type Comp = PlayerEntity;

    fn encode(&self, src: &Self::Comp, dst: &mut CompoundTag) {
        dst.insert_i8("playerGameType", src.game_mode.get_id() as i8);
        if let Some(previous_game_mode) = src.previous_game_mode {
            dst.insert_i8("previousPlayerGameType", previous_game_mode.get_id() as i8);
        }
        dst.insert_i32("Score", src.score as i32);
    }

    fn decode(&self, src: &CompoundTag) -> Self::Comp {
        PlayerEntity {
            game_mode: GameMode::from_id(src.get_i8("playerGameType").unwrap_or(0) as u8),
            previous_game_mode: src.get_i8("previousPlayerGameType").ok().map(|id| GameMode::from_id(id as u8)),
            score: src.get_i32("Score").unwrap_or(0) as u32
        }
    }

}