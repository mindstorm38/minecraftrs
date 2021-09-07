//! This is the base module for defining vanilla entities and their ECS components.
//! The ECS components are basically NBT structures that can be found on the wiki:
//! - https://minecraft.fandom.com/wiki/Entity_format

use mc_core::entities;

pub mod ai;

mod common;
mod snow_golem;
mod iron_golem;
mod axolotl;
mod parrot;
mod rabbit;
mod turtle;
mod slime;
mod sheep;
mod squid;
mod pet;
mod fish;
mod pig;
mod bat;
mod bee;

pub use common::*;
pub use snow_golem::*;
pub use iron_golem::*;
pub use axolotl::*;
pub use parrot::*;
pub use rabbit::*;
pub use turtle::*;
pub use slime::*;
pub use sheep::*;
pub use squid::*;
pub use pet::*;
pub use fish::*;
pub use pig::*;
pub use bat::*;
pub use bee::*;


macro_rules! vanilla_entities {
    ([
        $($entity_id:ident $entity_name:literal [$($comp_id:ident),*]),*
        $(,)?
    ]) => {
        mc_core::entities!(pub VANILLA_ENTITIES "minecraft" [
            $($entity_id $entity_name [VanillaEntity $(,$comp_id)*]),*
        ]);
    }
}

vanilla_entities!([
    // Living entities //
    AXOLOTL "axolotl" [MobEntity, LivingEntity, BreedableEntity, FromBucketEntity, AxolotlEntity],
    BAT "bat" [MobEntity, LivingEntity, BatEntity],
    BEE "bee" [MobEntity, LivingEntity, BreedableEntity, AngryEntity, BeeEntity],
    BLAZE "blaze" [MobEntity, LivingEntity],
    CAT "cat" [MobEntity, LivingEntity, BreedableEntity, TamableEntity, CatEntity],
    CAVE_SPIDER "cave_spider" [MobEntity, LivingEntity],
    CHICKEN "chicken" [],
    COD "cod" [MobEntity, LivingEntity, FromBucketEntity, CodEntity],
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
    GLOW_SQUID "glow_squid" [MobEntity, LivingEntity, GlowSquidEntity],
    GOAT "goat" [],
    GUARDIAN "guardian" [],
    HOGLIN "hoglin" [],
    HORSE "horse" [],
    HUSK "husk" [],
    ILLUSIONER "illusioner" [],
    IRON_GOLEM "iron_golem" [MobEntity, LivingEntity, AngryEntity, IronGolemEntity],
    LLAMA "llama" [],
    MAGMA_CUBE "magma_cube" [],
    MOOSHROOM "mooshroom" [],
    MULE "mule" [],
    OCELOT "ocelot" [],
    PANDA "panda" [],
    PARROT "parrot" [MobEntity, LivingEntity, TamableEntity, ParrotEntity],
    PHANTOM "phantom" [],
    PIG "pig" [MobEntity, LivingEntity, BreedableEntity, PigEntity],
    PIGLIN "piglin" [],
    PIGLIN_BRUTE "piglin_brute" [],
    PILLAGER "pillager" [],
    POLAR_BEAR "polar_bear" [],
    PUFFERFISH "pufferfish" [MobEntity, LivingEntity, FromBucketEntity, PufferfishEntity],
    RABBIT "rabbit" [MobEntity, LivingEntity, BreedableEntity, RabbitEntity],
    RAVAGER "ravager" [],
    SALMON "salmon" [MobEntity, LivingEntity, FromBucketEntity, SalmonEntity],
    SHEEP "sheep" [MobEntity, LivingEntity, BreedableEntity, SheepEntity],
    SHULKER "shulker" [],
    SILVERFISH "silverfish" [],
    SKELETON "skeleton" [],
    SKELETON_HORSE "skeleton_horse" [],
    SLIME "slime" [MobEntity, LivingEntity, SlimeEntity],
    SNOW_GOLEM "snow_golem" [MobEntity, LivingEntity, SnowGolemEntity],
    SPIDER "spider" [],
    STRIDER "strider" [],
    SQUID "squid" [MobEntity, LivingEntity, SquidEntity],
    STRAY "stray" [],
    TRADER_LLAMA "trader_llama" [],
    TROPICAL_FISH "tropical_fish" [MobEntity, LivingEntity, FromBucketEntity, TropicalFishEntity],
    TURTLE "turtle" [MobEntity, LivingEntity, BreedableEntity, TurtleEntity],
    VEX "vex" [],
    VILLAGER "villager" [],
    VINDICATOR "vindicator" [],
    WANDERING_TRADER "wandering_trader" [],
    WITCH "witch" [],
    WITHER "wither" [],
    WITHER_SKELETON "wither_skeleton" [],
    WOLF "wolf" [MobEntity, LivingEntity, TamableEntity, AngryEntity, BreedableEntity, PetEntity, WolfEntity],
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
