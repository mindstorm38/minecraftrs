//! This is the base module for defining vanilla entities and their ECS components.
//! The ECS components are basically NBT structures that can be found on the wiki:
//! - https://minecraft.fandom.com/wiki/Entity_format

use std::num::NonZeroU32;

use mc_core::pos::{EntityPos, BlockPos};
use mc_core::hecs::Entity;
use mc_core::uuid::Uuid;
use mc_core::entities;
use mc_core::entity::EntityType;

mod snow_golem;
mod iron_golem;
mod parrot;
mod rabbit;
mod turtle;
mod slime;
mod sheep;
mod squid;
mod wolf;
mod fish;
mod pig;

pub use snow_golem::*;
pub use iron_golem::*;
pub use parrot::*;
pub use rabbit::*;
pub use turtle::*;
pub use slime::*;
pub use sheep::*;
pub use squid::*;
pub use wolf::*;
pub use fish::*;
pub use pig::*;


macro_rules! vanilla_entities {
    ([
        $($entity_id:ident $entity_name:literal [$($comp_construct:ty),*]),*
        $(,)?
    ]) => {
        mc_core::entities!(pub VANILLA_ENTITIES "minecraft" [
            $($entity_id $entity_name [VanillaEntity $(,$comp_construct)*]),*
        ]);
    }
}

vanilla_entities!([
    // Living entities //
    AXOLOTL "axolotl" [],
    BAT "bat" [],
    BEE "bee" [],
    BLAZE "blaze" [],
    CAT "cat" [],
    CAVE_SPIDER "cave_spider" [],
    CHICKEN "chicken" [],
    COD "cod" [MobEntity, FishEntity, CodEntity],
    COW "cow" [],
    CREEPER "creeper" [],
    DOLPHIN "dolphin" [],
    DONKEY "donkey" [],
    DROWNED "drowned" [],
    ELDER_GUARDIAN "elder_guardian" [],
    ENDER_DRAGON "ender_dragon" [],
    ENDERMAN "enderman" [],
    ENDERMITE "endermite" [],
    EVOKER "evoker" [],
    FOX "fox" [],
    GHAST "ghast" [],
    GIANT "giant" [],
    GLOW_SQUID "glow_squid" [MobEntity, GlowSquidEntity],
    GOAT "goat" [],
    GUARDIAN "guardian" [],
    HOGLIN "hoglin" [],
    HORSE "horse" [],
    HUSK "husk" [],
    ILLUSIONER "illusioner" [],
    IRON_GOLEM "iron_golem" [MobEntity, AngryEntity, IronGolemEntity],
    LLAMA "llama" [],
    MAGMA_CUBE "magma_cube" [],
    MOOSHROOM "mooshroom" [],
    MULE "mule" [],
    OCELOT "ocelot" [],
    PANDA "panda" [],
    PARROT "parrot" [MobEntity, TamableEntity, ParrotEntity],
    PHANTOM "phantom" [],
    PIG "pig" [MobEntity, BreedableEntity, PigEntity],
    PIGLIN "piglin" [],
    PIGLIN_BRUTE "piglin_brute" [],
    PILLAGER "pillager" [],
    POLAR_BEAR "polar_bear" [],
    PUFFERFISH "pufferfish" [MobEntity, FishEntity, PufferfishEntity],
    RABBIT "rabbit" [MobEntity, BreedableEntity, RabbitEntity],
    RAVAGER "ravager" [],
    SALMON "salmon" [MobEntity, FishEntity, SalmonEntity],
    SHEEP "sheep" [MobEntity, BreedableEntity, SheepEntity],
    SHULKER "shulker" [],
    SILVERFISH "silverfish" [],
    SKELETON "skeleton" [],
    SKELETON_HORSE "skeleton_horse" [],
    SLIME "slime" [MobEntity, SlimeEntity],
    SNOW_GOLEM "snow_golem" [MobEntity, SnowGolemEntity],
    SPIDER "spider" [],
    STRIDER "strider" [],
    SQUID "squid" [MobEntity, SquidEntity],
    STRAY "stray" [],
    TRADER_LLAMA "trader_llama" [],
    TROPICAL_FISH "tropical_fish" [MobEntity, FishEntity, TropicalFishEntity],
    TURTLE "turtle" [MobEntity, BreedableEntity, TurtleEntity],
    VEX "vex" [],
    VILLAGER "villager" [],
    VINDICATOR "vindicator" [],
    WANDERING_TRADER "wandering_trader" [],
    WITCH "witch" [],
    WITHER "wither" [],
    WITHER_SKELETON "wither_skeleton" [],
    WOLF "wolf" [MobEntity, TamableEntity, AngryEntity, BreedableEntity, WolfEntity],
    ZOGLIN "zoglin" [],
    ZOMBIE "zombie" [],
    ZOMBIE_HORSE "zombie_horse" [],
    ZOMBIE_VILLAGER "zombie_villager" [],
    ZOMBIFIED_PIGLIN "zombified_piglin" [],
    // Projectiles entities //
    ARROW "arrow" [],
    DRAGON_FIREBALL "dragon_fireball" [],
    EGG "egg" [],
    ENDER_PEARL "ender_pearl" [],
    EXPERIENCE_BOTTLE "experience_bottle" [],
    FIREBALL "fireball" [],
    FIREWORK_ROCKET "firework_rocket" [],
    LLAMA_SPIT "llama_spit" [],
    POTION "potion" [],
    SMALL_FIREBALL "small_fireball" [],
    SHULKER_BULLET "shulker_bullet" [],
    SNOWBALL "snowball" [],
    SPECTRAL_ARROW "spectral_arrow" [],
    TRIDENT "trident" [],
    WITHER_SKULL "wither_skull" [],
    // Drop entities //
    EXPERIENCE_ORB "experience_orb" [],
    ITEM "item" [],
    // Vehicle entities //
    BOAT "boat" [],
    MINECART "minecart" [],
    CHEST_MINECART "chest_minecart" [],
    COMMAND_BLOCK_MINECART "command_block_minecart" [],
    FURNACE_MINECART "furnace_minecart" [],
    HOPPER_MINECART "hopper_minecart" [],
    SPAWNER_MINECART "spawner_minecart" [],
    TNT_MINECART "tnt_minecart" [],
    // Dynamic tiles //
    FALLING_BLOCK "falling_block" [],
    TNT "tnt" [],
    // Other entities //
    AREA_EFFECT_CLOUD "area_effect_cloud" [],
    ARMOR_STAND "armor_stand" [],
    END_CRYSTAL "end_crystal" [],
    EVOKER_FANGS "evoker_fangs" [],
    EYE_OF_ENDER "eye_of_ender" [],
    FISHING_BOBBER "fishing_bobber" [],
    ITEM_FRAME "item_frame" [],
    LIGHTNING_BOLT "lightning_bolt" [],
    MARKER "marker" [],
    PAINTING "painting" [],
]);


// Common components //

#[derive(Debug, Default)]
pub struct VanillaEntity {
    /// The current dX, dY and dZ velocity of the entity in meters per tick.
    motion: EntityPos,
    /// The entity's rotation clockwise around the Y axis (called yaw). Due south is 0.
    /// Does not exceed 360 degrees.
    rotation_raw: f32,
    /// The entity's declination from the horizon (called pitch). Horizontal is 0.
    /// Positive values look downward. Does not exceed positive or negative 90 degrees.
    rotation_pitch: f32,
    /// How much air the entity has, in ticks. Decreases by 1 per tick when unable to breathe
    /// (except suffocating in a block). Increase by 1 per tick when it can breathe. If -20
    /// while still unable to breathe, the entity loses 1 health and its air is reset to 0.
    air: i16,
    /// Distance the entity has fallen.
    fall_distance: f32,
    /// The optional custom name JSON text component of this entity with a boolean
    /// for the visibility of the custom name.
    custom_name: Option<(String, bool)>, // TODO: Replace with a struct like "TextComponent".
    /// List of scoreboard tags of this entity.
    tags: Option<Vec<String>>,
    /// True if the entity should not take damage.
    invulnerable: bool,
    /// True if the entity has a glowing outline.
    glowing: bool,
    /// If true, the entity does not fall down naturally.
    no_gravity: bool,
    /// True if the entity is touching the ground.
    on_ground: bool,
    /// If true, this entity is silenced.
    silent: bool,
    /// Number of ticks until the fire is put out. Negative values reflect how long the entity can
    /// stand in fire before burning. Default -20 when not on fire.
    fire: i16,
    /// If true, the entity visually appears on fire, even if it is not actually on fire.
    has_visual_fire: bool,
    /// The number of ticks before which the entity may be teleported back through a nether portal.
    portal_cooldown: u32,
    /// How many ticks the entity has been freezing.
    ticks_frozen: u32,
    /// An optional ist of entities that are on top of this one.
    passengers: Option<Vec<Entity>>,
}

#[derive(Debug, Default)]
pub struct MobEntity {
    // TODO: To implement:
    //  - stuff
    //  - brain
    //  - loot table
    //  - 'sadle' is intentionnaly ommited since it's really weird????
    /// True if the mob can pick up loot (wear armor it picks up, use weapons it picks up).
    can_pick_up_loot: bool,
    /// Number of ticks the mob has been dead for. Controls death animations. 0 when alive.
    death_time: u16,
    /// True when the entity is flying elytra, setting this on player has no effect but this
    /// can make mobs gliding.
    fall_flying: bool,
    /// Amount of health the entity has.
    health: f32,
    /// The last time the mob was damaged, measured in the number of ticks since the mob's
    /// creation. Updates to a new value whenever the mob is damaged.
    hurt_timestamp: u32,
    /// Number of ticks the mob turns red for after being hit. 0 when not recently hit.
    hurt_time: u16,
    /// Optional leash configuration for this entity.
    leash: Option<LeashConfig>,
    /// True if the mob renders the main hand as being left.
    left_handed: bool,
    /// Setting to true disables the mob's AI.
    no_ai: bool,
    /// True if the mob must not despawn naturally.
    persistent: bool,
    /// Some position of the block where the entity is sleeping.
    sleeping_pos: Option<BlockPos>
}

#[derive(Debug, Default)]
pub struct BreedableEntity {
    age: Age,
    love: Love
}

#[derive(Debug, Default)]
pub struct TamableEntity {
    /// Some uuid of the player who owns this mob.
    owner: Option<Uuid>,
    /// True if the mob is sitting.
    sitting: bool
}

#[derive(Debug, Default)]
pub struct AngryEntity {
    anger_time: i32,
    angry_at: Option<Uuid>
}


// Utilities for common entities //

#[derive(Debug)]
pub enum LeashConfig {
    Entity(Uuid),
    Fence(BlockPos)
}

#[derive(Debug, Copy, Clone)]
pub enum Age {
    Baby {
        breed_cooldown_once_adult: Option<u32>
    },
    Adult {
        breed_cooldown: u32
    }
}

impl Default for Age {
    fn default() -> Self {
        Self::Baby { breed_cooldown_once_adult: None }
    }
}

#[derive(Debug)]
pub enum Love {
    NotInLove,
    InLove {
        searching_cooldown: NonZeroU32,
        cause: Uuid
    }
}

impl Default for Love {
    fn default() -> Self {
        Self::NotInLove
    }
}
