use std::convert::TryFrom;
use std::num::NonZeroU32;

use mc_core::entity::SingleEntityCodec;
use mc_core::pos::{EntityPos, BlockPos};
use mc_core::util::NbtExt;
use mc_core::entity_component;

use nbt::CompoundTag;
use uuid::Uuid;

// Common components //

#[derive(Debug, Default)]
pub struct VanillaEntity {
    /// The current dX, dY and dZ velocity of the entity in meters per tick.
    motion: EntityPos,
    /// The entity's rotation clockwise around the Y axis (called yaw). Due south is 0.
    /// Does not exceed 360 degrees.
    rotation_yaw: f32,
    /// The entity's declination from the horizon (called pitch). Horizontal is 0.
    /// Positive values look downward. Does not exceed positive or negative 90 degrees.
    rotation_pitch: f32,
    /// How much air the entity has, in ticks. Decreases by 1 per tick when unable to breathe
    /// (except suffocating in a block). Increase by 1 per tick when it can breathe. If -20
    /// while still unable to breathe, the entity loses 1 health and its air is reset to 0.
    air: i16,
    /// Distance the entity has fallen.
    fall_distance: f32,
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
    remaining_fire_ticks: i16,
    /// If true, the entity visually appears on fire, even if it is not actually on fire.
    has_visual_fire: bool,
    /// The number of ticks before which the entity may be teleported back through a nether portal.
    portal_cooldown: u32,
    /// How many ticks the entity has been freezing.
    ticks_frozen: u32
}

impl VanillaEntity {

    pub fn is_on_fire(&self) -> bool {
        self.remaining_fire_ticks > 0
    }

}

entity_component!(VanillaEntity: VanillaEntityCodec);

pub struct VanillaEntityCodec;
impl SingleEntityCodec for VanillaEntityCodec {

    type Comp = VanillaEntity;

    fn encode(&self, src: &Self::Comp, dst: &mut CompoundTag) {

        dst.insert_entity_pos("Motion", &src.motion);
        dst.insert_f32_vec("Rotation", [src.rotation_yaw, src.rotation_pitch]);
        dst.insert_i16("Air", src.air);
        dst.insert_f32("FallDistance", src.fall_distance);

        if let Some(tags) = &src.tags {
            if !tags.is_empty() {
                dst.insert_str_vec("Tags", tags);
            }
        }

        dst.insert_bool("Invulnerable", src.invulnerable);
        dst.insert_bool("Glowing", src.glowing);
        dst.insert_bool("NoGravity", src.no_gravity);
        dst.insert_bool("OnGround", src.on_ground);
        dst.insert_bool("Silent", src.silent);
        dst.insert_i16("Fire", src.remaining_fire_ticks);
        dst.insert_bool("HasVisualFire", src.has_visual_fire);
        dst.insert_i32("PortalCooldown", i32::try_from(src.portal_cooldown).unwrap_or_default());
        dst.insert_i32("TicksFrozen", i32::try_from(src.ticks_frozen).unwrap_or_default());

    }

    fn decode(&self, src: &CompoundTag) -> Self::Comp {

        let mut rotation_yaw = 0.0;
        let mut rotation_pitch = 0.0;

        if let Ok(tag_rotation) = src.get_f32_vec("Rotation") {
            if tag_rotation.len() == 2 {
                rotation_yaw = tag_rotation[0];
                rotation_pitch = tag_rotation[1];
            }
        }

        VanillaEntity {
            motion: src.get_entity_pos("Motion").unwrap_or_default(),
            rotation_yaw,
            rotation_pitch,
            air: src.get_i16_or("Air", 0),
            fall_distance: src.get_f32_or("FallDistance", 0.0),
            tags: src.get_string_vec("Tags").ok(),
            invulnerable: src.get_bool_or("Invulnerable", false),
            glowing: src.get_bool_or("Glowing", false),
            no_gravity: src.get_bool_or("NoGravity", false),
            on_ground: src.get_bool_or("OnGround", false),
            silent: src.get_bool_or("Silent", false),
            remaining_fire_ticks: src.get_i16_or("Fire", 0),
            has_visual_fire: src.get_bool_or("HasVisualFire", false),
            portal_cooldown: src.get_i32("PortalCooldown")
                .map_or(0, |raw| u32::try_from(raw).unwrap_or_default()),
            ticks_frozen: src.get_i32("TicksFrozen")
                .map_or(0, |raw| u32::try_from(raw).unwrap_or_default())
        }

    }

}


// This field is separated from VanillaEntity because Player entities doesn't include it.
#[derive(Debug, Default)]
pub struct NamedEntity {
    /// The optional custom name JSON text component of this entity with a boolean
    /// for the visibility of the custom name.
    custom_name: Option<(String, bool)>, // TODO: Replace with a struct like "TextComponent".
}

entity_component!(NamedEntity: NamedEntityCodec);

pub struct NamedEntityCodec;
impl SingleEntityCodec for NamedEntityCodec {

    type Comp = NamedEntity;

    fn encode(&self, src: &Self::Comp, dst: &mut CompoundTag) {
        if let Some((custom_name, custom_name_visible)) = &src.custom_name {
            dst.insert_str("custom_name", custom_name);
            if *custom_name_visible {
                dst.insert_bool("CustomNameVisible", true);
            }
        }
    }

    fn decode(&self, src: &CompoundTag) -> Self::Comp {
        NamedEntity {
            custom_name: {
                if let Ok(cn) = src.get_str("CustomName") {
                    Some((cn.to_string(), src.get_bool_or("CustomNameVisible", false)))
                } else {
                    None
                }
            }
        }
    }

}


#[derive(Debug, Default)]
pub struct LivingEntity {
    // TODO: To implement: attributes, brain
    /// Amount of health the entity has.
    health: f32,
    /// Number of ticks the mob turns red for after being hit. 0 when not recently hit.
    hurt_time: u16,
    /// The last time the mob was damaged, measured in the number of ticks since the mob's
    /// creation. Updates to a new value whenever the mob is damaged.
    hurt_timestamp: u32,
    /// Number of ticks the mob has been dead for. Controls death animations. 0 when alive.
    death_time: u16,
    /// Amount of absorption health.
    absorption_amount: f32,
    /// True when the entity is flying elytra, setting this on player has no effect but this
    /// can make mobs gliding.
    fall_flying: bool,
    /// Some position of the block where the entity is sleeping.
    sleeping_pos: Option<BlockPos>,
}

entity_component!(LivingEntity: LivingEntityCodec);

pub struct LivingEntityCodec;
impl SingleEntityCodec for LivingEntityCodec {

    type Comp = LivingEntity;

    fn encode(&self, src: &Self::Comp, dst: &mut CompoundTag) {
        dst.insert_f32("Health", src.health);
        dst.insert_i16("HurtTime", i16::try_from(src.hurt_time).unwrap_or_default());
        dst.insert_i32("HurtByTimestamp", i32::try_from(src.hurt_timestamp).unwrap_or_default());
        dst.insert_i16("DeathTime", i16::try_from(src.death_time).unwrap_or_default());
        dst.insert_f32("AbsorptionAmount", src.absorption_amount);
        dst.insert_bool("FallFlying", src.fall_flying);
        if let Some(sleeping_pos) = &src.sleeping_pos {
            dst.insert_split_block_pos("SleepingX", "SleepingY", "SleepingZ", sleeping_pos);
        }
    }

    fn decode(&self, src: &CompoundTag) -> Self::Comp {
        LivingEntity {
            health: src.get_f32_or("Health", 0.0),
            hurt_time: src.get_i16("HurtTime")
                .map_or(0, |raw| u16::try_from(raw).unwrap_or_default()),
            hurt_timestamp: src.get_i32("HurtByTimestamp")
                .map_or(0, |raw| u32::try_from(raw).unwrap_or_default()),
            death_time: src.get_i16("DeathTime")
                .map_or(0, |raw| u16::try_from(raw).unwrap_or_default()),
            absorption_amount: src.get_f32("AbsorptionAmount").unwrap_or_default(),
            fall_flying: src.get_bool_or("FallFlying", false),
            sleeping_pos: src.get_split_block_pos("SleepingX", "SleepingY", "SleepingZ").ok()
        }
    }

}


#[derive(Debug, Default)]
pub struct MobEntity {
    // TODO: To implement:
    //  - stuff
    //  - loot table
    //  - 'saddle' is intentionnaly omitted since it's really weird, is wiki wrong????
    /// True if the mob can pick up loot (wear armor it picks up, use weapons it picks up).
    can_pick_up_loot: bool,
    /// True if the mob renders the main hand as being left.
    left_handed: bool,
    /// Setting to true disables the mob's AI.
    no_ai: bool,
    /// True if the mob must not despawn naturally.
    persistent: bool,
    /// Optional leash configuration for this entity.
    leash: Option<LeashConfig>,
}

entity_component!(MobEntity: MobEntityCodec);

pub struct MobEntityCodec;
impl SingleEntityCodec for MobEntityCodec {

    type Comp = MobEntity;

    fn encode(&self, src: &Self::Comp, dst: &mut CompoundTag) {
        dst.insert_bool("CanPickUpLoot", src.can_pick_up_loot);
        dst.insert_bool("LeftHanded", src.left_handed);
        dst.insert_bool("NoAI", src.no_ai);
        dst.insert_bool("PersistenceRequired", src.persistent);
        if let Some(leash) = &src.leash {
            let mut tag_leash = CompoundTag::new();
            match leash {
                LeashConfig::Entity(uuid) => {
                    tag_leash.insert_uuid("UUID", uuid);
                }
                LeashConfig::Fence(pos) => {
                    tag_leash.insert_split_block_pos("X", "Y", "Z", pos);
                }
            }
        }
    }

    fn decode(&self, src: &CompoundTag) -> Self::Comp {
        MobEntity {
            can_pick_up_loot: src.get_bool_or("CanPickUpLoot", false),
            left_handed: src.get_bool_or("LeftHanded", false),
            no_ai: src.get_bool_or("NoAI", false),
            persistent: src.get_bool_or("PersistenceRequired", false),
            leash: {
                if let Ok(tag_leash) = src.get_compound_tag("Leash") {
                    if let Ok(uuid) = tag_leash.get_uuid("UUID") {
                        Some(LeashConfig::Entity(uuid))
                    } else if let Ok(pos) = tag_leash.get_split_block_pos("X", "Y", "Z") {
                        Some(LeashConfig::Fence(pos))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        }
    }

}


#[derive(Debug, Default)]
pub struct BreedableEntity {
    age: Age,
    love: Love,
    love_cause: Option<Uuid>
}

entity_component!(BreedableEntity: BreedableEntityCodec);

pub struct BreedableEntityCodec;
impl SingleEntityCodec for BreedableEntityCodec {

    type Comp = BreedableEntity;

    fn encode(&self, src: &Self::Comp, dst: &mut CompoundTag) {

        match src.age {
            Age::Baby { ticks_remaining, breed_cooldown_once_adult } => {
                dst.insert_i32("Age", -i32::try_from(ticks_remaining).unwrap_or_default());
                if let Some(breed_cooldown_once_adult) = breed_cooldown_once_adult {
                    dst.insert_i32("ForcedAge", i32::try_from(breed_cooldown_once_adult).unwrap_or_default());
                }
            }
            Age::Adult { breed_cooldown } => {
                dst.insert("Age", i32::try_from(breed_cooldown).unwrap_or_default());
            }
        }

        match src.love {
            Love::NotInLove => dst.insert_i32("InLove", 0),
            Love::InLove(ticks) => dst.insert_i32("InLove", i32::try_from(ticks.get()).unwrap_or_default())
        }

        if let Some(love_cause) = &src.love_cause {
            dst.insert_uuid("LoveCause", love_cause);
        }

    }

    fn decode(&self, src: &CompoundTag) -> Self::Comp {
        BreedableEntity {
            age: {
                let raw_age = src.get_i32_or("Age", 0);
                if raw_age < 0 {
                    Age::Baby {
                        ticks_remaining: -raw_age as u32,
                        breed_cooldown_once_adult: {
                            match src.get_i32("ForcedAge") {
                                Ok(forced_age) => u32::try_from(forced_age).ok(),
                                Err(_) => None
                            }
                        }
                    }
                } else {
                    Age::Adult {
                        breed_cooldown: raw_age as u32
                    }
                }
            },
            love: {
                if let Ok(raw_in_love) = src.get_i32("InLove") {
                    if raw_in_love <= 0 {
                        Love::NotInLove
                    } else {
                        Love::InLove(unsafe { NonZeroU32::new_unchecked(raw_in_love as u32) })
                    }
                } else {
                    Love::NotInLove
                }
            },
            love_cause: src.get_uuid("LoveCause").ok()
        }
    }
    
}


#[derive(Debug, Default)]
pub struct TamableEntity {
    /// Some uuid of the player who owns this mob.
    owner: Option<Uuid>,
    /// True if the mob is sitting.
    sitting: bool
}

entity_component!(TamableEntity: TamableEntityCodec);

pub struct TamableEntityCodec;
impl SingleEntityCodec for TamableEntityCodec {

    type Comp = TamableEntity;

    fn encode(&self, src: &Self::Comp, dst: &mut CompoundTag) {
        if let Some(owner) = &src.owner {
            dst.insert_uuid("Owner", owner);
        }
        dst.insert_bool("Sitting", src.sitting);
    }

    fn decode(&self, src: &CompoundTag) -> Self::Comp {
        TamableEntity {
            owner: src.get_uuid("Owner").ok(),
            sitting: src.get_bool("Sitting").unwrap_or_default()
        }
    }
    
}


#[derive(Debug, Default)]
pub struct AngryEntity {
    anger_time: i32,
    angry_at: Option<Uuid>
}

entity_component!(AngryEntity: AngryEntityCodec);

pub struct AngryEntityCodec;
impl SingleEntityCodec for AngryEntityCodec {

    type Comp = AngryEntity;

    fn encode(&self, src: &Self::Comp, dst: &mut CompoundTag) {
        dst.insert_i32("AngerTime", src.anger_time);
        if let Some(angry_at) = &src.angry_at {
            dst.insert_uuid("AngryAt", angry_at);
        }
    }

    fn decode(&self, src: &CompoundTag) -> Self::Comp {
        AngryEntity {
            anger_time: src.get_i32_or("AngerTime", 0),
            angry_at: src.get_uuid("AngryAt").ok()
        }
    }

}


#[derive(Debug, Default)]
pub struct FromBucketEntity {
    /// I true, the fish has been released from a bucket.
    from_bucket: bool
}

entity_component!(FromBucketEntity: FromBucketEntityCodec);

pub struct FromBucketEntityCodec;
impl SingleEntityCodec for FromBucketEntityCodec {

    type Comp = FromBucketEntity;

    fn encode(&self, src: &Self::Comp, dst: &mut CompoundTag) {
        dst.insert_bool("FromBucket", src.from_bucket);
    }

    fn decode(&self, src: &CompoundTag) -> Self::Comp {
        FromBucketEntity {
            from_bucket: src.get_bool_or("FromBucket", false)
        }
    }

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
        ticks_remaining: u32,
        breed_cooldown_once_adult: Option<u32>
    },
    Adult {
        breed_cooldown: u32
    }
}

impl Default for Age {
    fn default() -> Self {
        Self::Adult { breed_cooldown: 0 }
    }
}

impl Age {

    pub fn tick(&mut self) {
        match self {
            Age::Baby { ticks_remaining, .. } => {
                *ticks_remaining = ticks_remaining.saturating_sub(1);
            }
            Age::Adult { breed_cooldown, .. } => {
                *breed_cooldown = breed_cooldown.saturating_sub(1);
            }
        }
    }

    pub fn should_grow(&self) -> bool {
        matches!(self, Age::Baby { ticks_remaining: 0, .. })
    }

    pub fn can_breed(&self) -> bool {
        matches!(self, Age::Adult { breed_cooldown: 0, .. })
    }

}

#[derive(Debug)]
pub enum Love {
    NotInLove,
    InLove(NonZeroU32)
}

impl Default for Love {
    fn default() -> Self {
        Self::NotInLove
    }
}
