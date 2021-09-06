use std::convert::TryFrom;
use std::num::NonZeroU32;

use mc_core::entity::{EntityCodec, EntityComponent};
use mc_core::pos::{EntityPos, BlockPos};
use mc_core::nbt::CompoundTag;
use mc_core::util::NbtExt;
use mc_core::uuid::Uuid;
use mc_core::hecs::{EntityRef, EntityBuilder};


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

pub struct VanillaEntityCodec;
impl EntityCodec for VanillaEntityCodec {

    fn encode(&self, src: &EntityRef, dst: &mut CompoundTag) -> Result<(), String> {

        if let Some(comp) = src.get::<VanillaEntity>() {

            dst.insert_entity_pos("Motion", &comp.motion);
            dst.insert_f32_vec("Rotation", [comp.rotation_yaw, comp.rotation_pitch]);
            dst.insert_i16("Air", comp.air);
            dst.insert_f32("FallDistance", comp.fall_distance);

            if let Some((custom_name, custom_name_visible)) = &comp.custom_name {
                dst.insert_str("custom_name", custom_name);
                if *custom_name_visible {
                    dst.insert_bool("CustomNameVisible", true);
                }
            }

            if let Some(tags) = &comp.tags {
                if !tags.is_empty() {
                    dst.insert_str_vec("Tags", tags);
                }
            }

            dst.insert_bool("Invulnerable", comp.invulnerable);
            dst.insert_bool("Glowing", comp.glowing);
            dst.insert_bool("NoGravity", comp.no_gravity);
            dst.insert_bool("OnGround", comp.on_ground);
            dst.insert_bool("Silent", comp.silent);
            dst.insert_i16("Fire", comp.remaining_fire_ticks);
            dst.insert_bool("HasVisualFire", comp.has_visual_fire);
            dst.insert_i32("PortalCooldown", i32::try_from(comp.portal_cooldown).unwrap_or_default());
            dst.insert_i32("TicksFrozen", i32::try_from(comp.ticks_frozen).unwrap_or_default());

        }

        Ok(())

    }

    fn decode(&self, src: &CompoundTag, dst: &mut EntityBuilder) -> Result<(), String> {

        let mut rotation_raw = 0.0;
        let mut rotation_pitch = 0.0;

        if let Ok(tag_rotation) = src.get_f32_vec("Rotation") {
            if tag_rotation.len() == 2 {
                rotation_raw = tag_rotation[0];
                rotation_pitch = tag_rotation[1];
            }
        }

        dst.add(VanillaEntity {
            motion: src.get_entity_pos("Motion").unwrap_or_default(),
            rotation_yaw: rotation_raw,
            rotation_pitch,
            air: src.get_i16("Air").unwrap_or_default(),
            fall_distance: src.get_f32("FallDistance").unwrap_or_default(),
            custom_name: {
                if let Ok(cn) = src.get_str("CustomName") {
                    Some((cn.to_string(), src.get_bool("CustomNameVisible").unwrap_or_default()))
                } else {
                    None
                }
            },
            tags: src.get_string_vec("Tags").ok(),
            invulnerable: src.get_bool("Invulnerable").unwrap_or_default(),
            glowing: src.get_bool("Glowing").unwrap_or_default(),
            no_gravity: src.get_bool("NoGravity").unwrap_or_default(),
            on_ground: src.get_bool("OnGround").unwrap_or_default(),
            silent: src.get_bool("Silent").unwrap_or_default(),
            remaining_fire_ticks: src.get_i16("Fire").unwrap_or_default(),
            has_visual_fire: src.get_bool("HasVisualFire").unwrap_or_default(),
            portal_cooldown: src.get_i32("PortalCooldown")
                .map_or(0, |raw| u32::try_from(raw).unwrap_or_default()),
            ticks_frozen: src.get_i32("TicksFrozen")
                .map_or(0, |raw| u32::try_from(raw).unwrap_or_default())
        });

        Ok(())

    }

    fn default(&self, dst: &mut EntityBuilder) {
        dst.add(VanillaEntity::default());
    }

}

impl EntityComponent for VanillaEntity {
    const CODEC: &'static dyn EntityCodec = &VanillaEntityCodec;
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

impl EntityComponent for LivingEntity {
    const CODEC: &'static dyn EntityCodec = &LivingEntityCodec;
}

pub struct LivingEntityCodec;
impl EntityCodec for LivingEntityCodec {

    fn encode(&self, src: &EntityRef, dst: &mut CompoundTag) -> Result<(), String> {
        if let Some(comp) = src.get::<LivingEntity>() {
            dst.insert_f32("Health", comp.health);
            dst.insert_i16("HurtTime", i16::try_from(comp.hurt_time).unwrap_or_default());
            dst.insert_i32("HurtByTimestamp", i32::try_from(comp.hurt_timestamp).unwrap_or_default());
            dst.insert_i16("DeathTime", i16::try_from(comp.death_time).unwrap_or_default());
            dst.insert_f32("AbsorptionAmount", comp.absorption_amount);
            dst.insert_bool("FallFlying", comp.fall_flying);
            if let Some(sleeping_pos) = &comp.sleeping_pos {
                dst.insert_split_block_pos("SleepingX", "SleepingY", "SleepingZ", sleeping_pos);
            }
        }
        Ok(())
    }

    fn decode(&self, src: &CompoundTag, dst: &mut EntityBuilder) -> Result<(), String> {
        dst.add(LivingEntity {
            health: src.get_f32("Health").unwrap_or_default(),
            hurt_time: src.get_i16("HurtTime")
                .map_or(0, |raw| u16::try_from(raw).unwrap_or_default()),
            hurt_timestamp: src.get_i32("HurtByTimestamp")
                .map_or(0, |raw| u32::try_from(raw).unwrap_or_default()),
            death_time: src.get_i16("DeathTime")
                .map_or(0, |raw| u16::try_from(raw).unwrap_or_default()),
            absorption_amount: src.get_f32("AbsorptionAmount").unwrap_or_default(),
            fall_flying: src.get_bool("FallFlying").unwrap_or_default(),
            sleeping_pos: src.get_split_block_pos("SleepingX", "SleepingY", "SleepingZ").ok()
        });
        Ok(())
    }

    fn default(&self, dst: &mut EntityBuilder) {
        dst.add(LivingEntity::default());
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

impl EntityComponent for MobEntity {
    const CODEC: &'static dyn EntityCodec = &MobEntityCodec;
}

pub struct MobEntityCodec;
impl EntityCodec for MobEntityCodec {

    fn encode(&self, src: &EntityRef, dst: &mut CompoundTag) -> Result<(), String> {
        if let Some(comp) = src.get::<MobEntity>() {
            dst.insert_bool("CanPickUpLoot", comp.can_pick_up_loot);
            dst.insert_bool("LeftHanded", comp.left_handed);
            dst.insert_bool("NoAI", comp.no_ai);
            dst.insert_bool("PersistenceRequired", comp.persistent);

            if let Some(leash) = &comp.leash {
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
        Ok(())
    }

    fn decode(&self, src: &CompoundTag, dst: &mut EntityBuilder) -> Result<(), String> {
        dst.add(MobEntity {
            can_pick_up_loot: src.get_bool("CanPickUpLoot").unwrap_or_default(),
            left_handed: src.get_bool("LeftHanded").unwrap_or_default(),
            no_ai: src.get_bool("NoAI").unwrap_or_default(),
            persistent: src.get_bool("PersistenceRequired").unwrap_or_default(),
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
        });
        Ok(())
    }

    fn default(&self, dst: &mut EntityBuilder) {
        dst.add(MobEntity::default());
    }

}


#[derive(Debug, Default)]
pub struct BreedableEntity {
    age: Age,
    love: Love,
    love_cause: Option<Uuid>
}

impl EntityComponent for BreedableEntity {
    const CODEC: &'static dyn EntityCodec = &BreedableEntityCodec;
}

pub struct BreedableEntityCodec;
impl EntityCodec for BreedableEntityCodec {
    
    fn encode(&self, src: &EntityRef, dst: &mut CompoundTag) -> Result<(), String> {
        if let Some(comp) = src.get::<BreedableEntity>() {

            match comp.age {
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

            match comp.love {
                Love::NotInLove => dst.insert_i32("InLove", 0),
                Love::InLove(ticks) => dst.insert_i32("InLove", i32::try_from(ticks.get()).unwrap_or_default())
            }

            if let Some(love_cause) = &comp.love_cause {
                dst.insert_uuid("LoveCause", love_cause);
            }

        }
        Ok(())
    }

    fn decode(&self, src: &CompoundTag, dst: &mut EntityBuilder) -> Result<(), String> {
        dst.add(BreedableEntity {
            age: {
                let raw_age = src.get_i32("Age").unwrap_or_default();
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
        });
        Ok(())
    }

    fn default(&self, dst: &mut EntityBuilder) {
        dst.add(BreedableEntity::default());
    }
    
}



#[derive(Debug, Default)]
pub struct TamableEntity {
    /// Some uuid of the player who owns this mob.
    owner: Option<Uuid>,
    /// True if the mob is sitting.
    sitting: bool
}

impl EntityComponent for TamableEntity {
    const CODEC: &'static dyn EntityCodec = &TamableEntityCodec;
}

pub struct TamableEntityCodec;
impl EntityCodec for TamableEntity {
    
    fn encode(&self, src: &EntityRef, dst: &mut CompoundTag) -> Result<(), String> {
        if let Some(comp) = src.get::<TamableEntity>() {
            if let Some(owner) = &comp.owner {
                dst.insert_uuid("Owner", owner);
            }
            dst.insert_bool("Sitting", comp.sitting);
        }
        Ok(())
    }

    fn decode(&self, src: &CompoundTag, dst: &mut EntityBuilder) -> Result<(), String> {
        dst.add(TamableEntity {
            owner: src.get_uuid("Owner").ok(),
            sitting: src.get_bool("Sitting").unwrap_or_default()
        });
        Ok(())
    }

    fn default(&self, dst: &mut EntityBuilder) {
        dst.add(TamableEntity::default());
    }
    
}

#[derive(Debug, Default)]
pub struct AngryEntity {
    anger_time: i32,
    angry_at: Option<Uuid>
}

impl EntityComponent for AngryEntity {
    const CODEC: &'static dyn EntityCodec = &AngryEntityCodec;
}

pub struct AngryEntityCodec;
impl EntityCodec for AngryEntityCodec {

    fn encode(&self, src: &EntityRef, dst: &mut CompoundTag) -> Result<(), String> {
        if let Some(comp) = src.get::<AngryEntity>() {
            dst.insert_i32("AngerTime", comp.anger_time);
            if let Some(angry_at) = &comp.angry_at {
                dst.insert_uuid("AngryAt", angry_at);
            }
        }
        Ok(())
    }

    fn decode(&self, src: &CompoundTag, dst: &mut EntityBuilder) -> Result<(), String> {
        dst.add(AngryEntity {
            anger_time: src.get_i32("AngerTime").unwrap_or_default(),
            angry_at: src.get_uuid("AngryAt").ok()
        });
        Ok(())
    }

    fn default(&self, dst: &mut EntityBuilder) {
        dst.add(AngryEntity::default());
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
                ticks_remaining.saturating_sub(1);
            }
            Age::Adult { breed_cooldown, .. } => {
                breed_cooldown.saturating_sub(1);
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
