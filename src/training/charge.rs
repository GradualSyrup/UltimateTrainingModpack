use crate::common::consts::FighterId;
use crate::common::get_module_accessor;
use crate::training::character_specific::pikmin;
use serde::{Deserialize, Serialize};
use smash::app::{self, lua_bind::*, ArticleOperationTarget, FighterFacial, FighterUtil};
use smash::lib::lua_const::*;
use smash::phx::{Hash40, Vector3f};
use std::ptr;

#[skyline::from_offset(0xba0e60)]
fn copy_setup(
    module_accessor: *mut app::BattleObjectModuleAccessor,
    int: i32,
    fighter_kind: i32,
    bool_1: bool,
    bool_2: bool,
);

#[derive(Serialize, Deserialize, Default, Copy, Clone, Debug)]
pub struct ChargeState {
    pub int_x: Option<i32>,
    pub int_y: Option<i32>,
    pub int_z: Option<i32>,
    pub float_x: Option<f32>,
    pub float_y: Option<f32>,
    pub float_z: Option<f32>,
    pub has_charge: Option<bool>,
}

impl ChargeState {
    fn int_x(mut self, int_x: i32) -> Self {
        self.int_x = Some(int_x);
        self
    }

    fn int_y(mut self, int_y: i32) -> Self {
        self.int_y = Some(int_y);
        self
    }

    fn int_z(mut self, int_z: i32) -> Self {
        self.int_z = Some(int_z);
        self
    }

    fn float_x(mut self, float_x: f32) -> Self {
        self.float_x = Some(float_x);
        self
    }

    fn float_y(mut self, float_y: f32) -> Self {
        self.float_y = Some(float_y);
        self
    }

    fn float_z(mut self, float_z: f32) -> Self {
        self.float_z = Some(float_z);
        self
    }

    fn set_pikmin(
        mut self,
        pikmin_1: Option<i32>,
        pikmin_2: Option<i32>,
        pikmin_3: Option<i32>,
    ) -> Self {
        self.int_x = pikmin_1;
        self.int_y = pikmin_2;
        self.int_z = pikmin_3;
        self
    }

    fn has_charge(mut self, has_charge: bool) -> Self {
        self.has_charge = Some(has_charge);
        self
    }
}

pub unsafe fn get_charge(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    fighter_kind: i32,
) -> ChargeState {
    let charge_state = ChargeState::default();
    // Mario FLUDD
    if fighter_kind == FIGHTER_KIND_MARIO {
        let my_charge = WorkModule::get_int(
            module_accessor,
            *FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_CHARGE,
        );
        charge_state.int_x(my_charge)
    }
    // Donkey Kong Giant Punch
    else if fighter_kind == FIGHTER_KIND_DONKEY {
        let my_charge = WorkModule::get_int(
            module_accessor,
            *FIGHTER_DONKEY_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT,
        );
        charge_state.int_x(my_charge)
    }
    // Samus/Dark Samus Charge Shot
    else if fighter_kind == FIGHTER_KIND_SAMUS || fighter_kind == FIGHTER_KIND_SAMUSD {
        let my_charge = WorkModule::get_int(
            module_accessor,
            *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT,
        );
        charge_state.int_x(my_charge)
    }
    // Kirby Copy Abilities
    else if fighter_kind == FIGHTER_KIND_KIRBY {
        let hat_have =
            WorkModule::is_flag(module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_COPY);
        let chara_kind = WorkModule::get_int(
            module_accessor,
            *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA,
        );
        let kirby_charge = charge_state.has_charge(hat_have).int_z(chara_kind);
        // Get the charge of necessary abilities for kirby
        get_kirby_hat_charge(module_accessor, chara_kind, kirby_charge)
    }
    // Sheik Needles
    else if fighter_kind == FIGHTER_KIND_SHEIK {
        let my_charge = WorkModule::get_int(
            module_accessor,
            *FIGHTER_SHEIK_INSTANCE_WORK_ID_INT_NEEDLE_COUNT,
        );
        charge_state.int_x(my_charge)
    }
    // Mewtwo Shadowball
    else if fighter_kind == FIGHTER_KIND_MEWTWO {
        let my_charge = WorkModule::get_int(
            module_accessor,
            *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_SHADOWBALL_CHARGE_FRAME,
        );
        let prev_frame = WorkModule::get_int(
            module_accessor,
            *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_PREV_SHADOWBALL_CHARGE_FRAME,
        );
        let ball_had = WorkModule::is_flag(
            module_accessor,
            *FIGHTER_MEWTWO_INSTANCE_WORK_ID_FLAG_SHADOWBALL_HAD,
        );
        charge_state
            .int_x(my_charge)
            .int_y(prev_frame)
            .has_charge(ball_had)
    }
    // Game and Watch Bucket
    else if fighter_kind == FIGHTER_KIND_GAMEWATCH {
        let my_charge = WorkModule::get_float(
            module_accessor,
            *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_GAUGE,
        );
        let my_attack = WorkModule::get_float(
            module_accessor,
            *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_ATTACK,
        );
        charge_state.float_x(my_charge).float_y(my_attack)
    }
    // Wario Waft
    else if fighter_kind == FIGHTER_KIND_WARIO {
        let my_charge = WorkModule::get_int(
            module_accessor,
            *FIGHTER_WARIO_INSTANCE_WORK_ID_INT_GASS_COUNT,
        );
        charge_state.int_x(my_charge)
    }
    // Squirtle Water Gun
    else if fighter_kind == FIGHTER_KIND_PZENIGAME {
        let my_charge = WorkModule::get_int(
            module_accessor,
            *FIGHTER_PZENIGAME_INSTANCE_WORK_ID_INT_SPECIAL_N_CHARGE,
        );
        charge_state.int_x(my_charge)
    }
    // Olimar Pikmin
    else if fighter_kind == FIGHTER_KIND_PIKMIN {
        let pikmin_array = pikmin::get_current_pikmin(module_accessor);
        return charge_state.set_pikmin(pikmin_array[0], pikmin_array[1], pikmin_array[2]);
    }
    // Lucario Aura Sphere
    else if fighter_kind == FIGHTER_KIND_LUCARIO {
        let my_charge = WorkModule::get_int(
            module_accessor,
            *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AURABALL_CHARGE_FRAME,
        );
        let prev_frame = WorkModule::get_int(
            module_accessor,
            *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_PREV_AURABALL_CHARGE_FRAME,
        );
        let ball_had = WorkModule::is_flag(
            module_accessor,
            *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_AURABALL_HAD,
        );
        charge_state
            .int_x(my_charge)
            .int_y(prev_frame)
            .has_charge(ball_had)
    }
    // ROB Gyro/Laser/Fuel
    else if fighter_kind == FIGHTER_KIND_ROBOT {
        let laser_charge = WorkModule::get_float(
            module_accessor,
            *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BEAM_ENERGY_VALUE,
        );
        let gyro_charge = WorkModule::get_float(
            module_accessor,
            *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_GYRO_CHARGE_VALUE,
        );
        let fuel_charge = WorkModule::get_float(
            module_accessor,
            *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BURNER_ENERGY_VALUE,
        );
        charge_state
            .float_x(laser_charge)
            .float_y(gyro_charge)
            .float_z(fuel_charge)
    }
    // Wii Fit Sun Salutation
    else if fighter_kind == FIGHTER_KIND_WIIFIT {
        let my_charge = WorkModule::get_float(
            module_accessor,
            *FIGHTER_WIIFIT_INSTANCE_WORK_ID_FLOAT_SPECIAL_N_CHARGE_LEVEL_RATIO,
        );
        charge_state.float_x(my_charge)
    }
    // Pac-Man Bonus Fruit
    else if fighter_kind == FIGHTER_KIND_PACMAN {
        let my_charge = WorkModule::get_int(
            module_accessor,
            *FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_SPECIAL_N_CHARGE_RANK,
        );
        let fruit_have = WorkModule::is_flag(
            module_accessor,
            *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_PULL_THROW,
        );
        charge_state.int_x(my_charge).has_charge(fruit_have)
    }
    // Robin Thunder Tome Spells
    else if fighter_kind == FIGHTER_KIND_REFLET {
        let my_charge = WorkModule::get_int(
            module_accessor,
            *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_THUNDER_KIND,
        );
        charge_state.int_x(my_charge)
    }
    // Plant Poison Breath
    else if fighter_kind == FIGHTER_KIND_PACKUN {
        let my_charge = WorkModule::get_int(
            module_accessor,
            *FIGHTER_PACKUN_INSTANCE_WORK_ID_INT_SPECIAL_S_COUNT,
        );
        charge_state.int_x(my_charge)
    }
    // Hero (Ka)frizz(le)
    else if fighter_kind == FIGHTER_KIND_BRAVE {
        let my_charge = WorkModule::get_int(
            module_accessor,
            *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_N_HOLD_FRAME,
        );
        charge_state.int_x(my_charge)
    }
    // Banjo Wonderwing
    else if fighter_kind == FIGHTER_KIND_BUDDY {
        let my_charge = WorkModule::get_int(
            module_accessor,
            *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_S_REMAIN,
        );
        charge_state.int_x(my_charge)
    }
    // Mii Gunner Charge Blast
    else if fighter_kind == FIGHTER_KIND_MIIGUNNER {
        let my_charge = WorkModule::get_int(
            module_accessor,
            *FIGHTER_MIIGUNNER_INSTANCE_WORK_ID_INT_GUNNER_CHARGE_COUNT,
        );
        charge_state.int_x(my_charge)
    } else {
        charge_state
    }
}

pub unsafe fn handle_charge(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    fighter_kind: i32,
    charge: ChargeState,
) {
    // Mario Fludd - 0 to 80
    if fighter_kind == FIGHTER_KIND_MARIO {
        charge.int_x.map(|fludd_charge| {
            WorkModule::set_int(
                module_accessor,
                fludd_charge,
                *FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_CHARGE,
            );
            if fludd_charge == 80 {
                EffectModule::req_common(module_accessor, Hash40::new("charge_max"), 0.0);
            }
        });
    }
    // DK Punch - 0 to 110
    else if fighter_kind == FIGHTER_KIND_DONKEY {
        charge.int_x.map(|punch_charge| {
            WorkModule::set_int(
                module_accessor,
                punch_charge,
                *FIGHTER_DONKEY_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT,
            );
            if punch_charge == 110 {
                FighterUtil::set_face_motion_by_priority(
                    module_accessor,
                    FighterFacial(*FIGHTER_FACIAL_SPECIAL),
                    Hash40::new("special_n_max_face"),
                );
            }
        });
    }
    // Samus/Dark Samus Charge Shot - 0 to 112
    else if fighter_kind == FIGHTER_KIND_SAMUS || fighter_kind == FIGHTER_KIND_SAMUSD {
        charge.int_x.map(|shot_charge| {
            WorkModule::set_int(
                module_accessor,
                shot_charge,
                *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT,
            );
            if shot_charge == 112 {
                EffectModule::req_common(module_accessor, Hash40::new("charge_max"), 0.0);
                let samus_cshot_hash = if fighter_kind == FIGHTER_KIND_SAMUS {
                    Hash40::new("samus_cshot_max")
                } else {
                    Hash40::new("samusd_cshot_max")
                };
                let joint_hash = Hash40::new("armr");
                let pos = Vector3f {
                    x: 7.98004,
                    y: -0.50584,
                    z: -0.25092,
                };
                let rot = Vector3f {
                    x: -91.2728,
                    y: -1.7974,
                    z: 176.373,
                };
                let efh = EffectModule::req_follow(
                    module_accessor,
                    samus_cshot_hash,
                    joint_hash,
                    &pos,
                    &rot,
                    1.0,
                    false,
                    0,
                    0,
                    0,
                    0,
                    0,
                    false,
                    false,
                );
                WorkModule::set_int(
                    module_accessor,
                    efh as i32,
                    *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_EFH_CHARGE_MAX,
                );
            }
        });
    }
    // Kirby Copy Abilities
    else if fighter_kind == FIGHTER_KIND_KIRBY {
        charge.has_charge.map(|_has_copy_ability| {
            let cpu_module_accessor = &mut *get_module_accessor(FighterId::CPU);
            let player_module_accessor = &mut *get_module_accessor(FighterId::Player);
            let opponent_module_accessor: &mut app::BattleObjectModuleAccessor =
                if ptr::eq(module_accessor, player_module_accessor) {
                    cpu_module_accessor
                } else {
                    player_module_accessor
                };
            // Only try to set up Copy Ability when the current opponent matches the type of fighter from the save state
            let opponent_matches_fighter =
                is_kirby_hat_okay(opponent_module_accessor, charge.int_z);
            if opponent_matches_fighter == Some(true) {
                copy_setup(module_accessor, 1, charge.int_z.unwrap(), true, false);
                handle_kirby_hat_charge(module_accessor, charge.int_z.unwrap(), charge);
            }
        });
    }
    // Sheik Needles - 0 to 6
    else if fighter_kind == FIGHTER_KIND_SHEIK {
        charge.int_x.map(|needle_charge| {
            WorkModule::set_int(
                module_accessor,
                needle_charge,
                *FIGHTER_SHEIK_INSTANCE_WORK_ID_INT_NEEDLE_COUNT,
            );
            ArticleModule::generate_article_enable(
                module_accessor,
                *FIGHTER_SHEIK_GENERATE_ARTICLE_NEEDLEHAVE,
                false,
                -1,
            );
            let hash_main = Hash40::new("set_main");
            match needle_charge {
                6 => {
                    EffectModule::req_common(module_accessor, Hash40::new("charge_max"), 0.0);
                    ArticleModule::set_visibility(
                        module_accessor,
                        *FIGHTER_SHEIK_GENERATE_ARTICLE_NEEDLEHAVE,
                        hash_main,
                        Hash40::new("group_main_default"),
                        ArticleOperationTarget(0),
                    );
                }
                5 => {
                    ArticleModule::set_visibility(
                        module_accessor,
                        *FIGHTER_SHEIK_GENERATE_ARTICLE_NEEDLEHAVE,
                        hash_main,
                        Hash40::new("group_main_5"),
                        ArticleOperationTarget(0),
                    );
                }
                4 => {
                    ArticleModule::set_visibility(
                        module_accessor,
                        *FIGHTER_SHEIK_GENERATE_ARTICLE_NEEDLEHAVE,
                        hash_main,
                        Hash40::new("group_main_4"),
                        ArticleOperationTarget(0),
                    );
                }
                3 => {
                    ArticleModule::set_visibility(
                        module_accessor,
                        *FIGHTER_SHEIK_GENERATE_ARTICLE_NEEDLEHAVE,
                        hash_main,
                        Hash40::new("group_main_3"),
                        ArticleOperationTarget(0),
                    );
                }
                2 => {
                    ArticleModule::set_visibility(
                        module_accessor,
                        *FIGHTER_SHEIK_GENERATE_ARTICLE_NEEDLEHAVE,
                        hash_main,
                        Hash40::new("group_main_2"),
                        ArticleOperationTarget(0),
                    );
                }
                1 => {
                    ArticleModule::set_visibility(
                        module_accessor,
                        *FIGHTER_SHEIK_GENERATE_ARTICLE_NEEDLEHAVE,
                        hash_main,
                        Hash40::new("group_main_1"),
                        ArticleOperationTarget(0),
                    );
                }
                _ => {
                    ArticleModule::set_visibility(
                        module_accessor,
                        *FIGHTER_SHEIK_GENERATE_ARTICLE_NEEDLEHAVE,
                        hash_main,
                        Hash40::new("group_main_0"),
                        ArticleOperationTarget(0),
                    );
                }
            }
        });
    }
    // Mewtwo Shadowball - 0 to 120, Boolean
    else if fighter_kind == FIGHTER_KIND_MEWTWO {
        charge.int_x.map(|charge_frame| {
            WorkModule::set_int(
                module_accessor,
                charge_frame,
                *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_SHADOWBALL_CHARGE_FRAME,
            );
        });
        charge.int_y.map(|prev_frame| {
            WorkModule::set_int(
                module_accessor,
                prev_frame,
                *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_PREV_SHADOWBALL_CHARGE_FRAME,
            );
            if prev_frame == 120 {
                EffectModule::req_common(module_accessor, Hash40::new("charge_max"), 0.0);
                let effect_hash = Hash40::new("mewtwo_shadowball_max_hand");
                let joint_hash_1 = Hash40::new("handl");
                let joint_hash_2 = Hash40::new("handr");
                let pos = Vector3f {
                    x: 1.0,
                    y: 0.5,
                    z: 0.0,
                };
                let rot = Vector3f {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                };
                let efh_1 = EffectModule::req_follow(
                    module_accessor,
                    effect_hash,
                    joint_hash_1,
                    &pos,
                    &rot,
                    1.0,
                    false,
                    0,
                    0,
                    -1,
                    0,
                    0,
                    false,
                    false,
                );
                let efh_2 = EffectModule::req_follow(
                    module_accessor,
                    effect_hash,
                    joint_hash_2,
                    &pos,
                    &rot,
                    1.0,
                    false,
                    0,
                    0,
                    -1,
                    0,
                    0,
                    false,
                    false,
                );
                WorkModule::set_int(
                    module_accessor,
                    efh_1 as i32,
                    *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_EF_ID_SHADOWBALL_MAX_L,
                );
                WorkModule::set_int(
                    module_accessor,
                    efh_2 as i32,
                    *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_EF_ID_SHADOWBALL_MAX_R,
                );
            }
        });
        charge.has_charge.map(|has_shadowball| {
            WorkModule::set_flag(
                module_accessor,
                has_shadowball,
                *FIGHTER_MEWTWO_INSTANCE_WORK_ID_FLAG_SHADOWBALL_HAD,
            );
        });
    }
    // GnW Bucket - 0 to 3, Attack not tested
    else if fighter_kind == FIGHTER_KIND_GAMEWATCH {
        charge.float_x.map(|bucket_level| {
            WorkModule::set_float(
                module_accessor,
                bucket_level,
                *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_GAUGE,
            );
            if bucket_level == 3.0 {
                EffectModule::req_common(module_accessor, Hash40::new("charge_max"), 0.0);
            } else {
                // GnW flashes when successfully bucketing, and it will persist if state is loaded during that time, so we remove it here
                EffectModule::remove_common(module_accessor, Hash40::new("charge_max"));
            }
        });
        charge.float_y.map(|bucket_attack| {
            WorkModule::set_float(
                module_accessor,
                bucket_attack,
                *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_ATTACK,
            );
        });
    }
    // Wario Waft - 0 to 6000
    else if fighter_kind == FIGHTER_KIND_WARIO {
        charge.int_x.map(|waft_count| {
            WorkModule::set_int(
                module_accessor,
                waft_count,
                *FIGHTER_WARIO_INSTANCE_WORK_ID_INT_GASS_COUNT,
            );
        });
    }
    // Squirtle Water Gun - 0 to 45
    else if fighter_kind == FIGHTER_KIND_PZENIGAME {
        charge.int_x.map(|water_charge| {
            WorkModule::set_int(
                module_accessor,
                water_charge,
                *FIGHTER_PZENIGAME_INSTANCE_WORK_ID_INT_SPECIAL_N_CHARGE,
            );
            if water_charge == 45 {
                EffectModule::req_common(module_accessor, Hash40::new("charge_max"), 0.0);
            }
        });
    }
    // Olimar Pikmin - 0 to 4
    else if fighter_kind == FIGHTER_KIND_PIKMIN {
        ArticleModule::remove_exist(
            module_accessor,
            *FIGHTER_PIKMIN_GENERATE_ARTICLE_PIKMIN,
            app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL),
        );
        if ArticleModule::get_active_num(module_accessor, *FIGHTER_PIKMIN_GENERATE_ARTICLE_PIKMIN)
            == 0
        {
            charge.int_x.map(|pikmin_1| {
                pikmin::spawn_pikmin(module_accessor, pikmin_1);
            });
        }
        if ArticleModule::get_active_num(module_accessor, *FIGHTER_PIKMIN_GENERATE_ARTICLE_PIKMIN)
            == 1
        {
            charge.int_y.map(|pikmin_2| {
                pikmin::spawn_pikmin(module_accessor, pikmin_2);
            });
        }
        if ArticleModule::get_active_num(module_accessor, *FIGHTER_PIKMIN_GENERATE_ARTICLE_PIKMIN)
            == 2
        {
            charge.int_z.map(|pikmin_3| {
                pikmin::spawn_pikmin(module_accessor, pikmin_3);
            });
        }
        pikmin::follow(module_accessor);
    }
    // Lucario Aura Sphere - 0 to 90, Boolean
    else if fighter_kind == FIGHTER_KIND_LUCARIO {
        charge.int_x.map(|charge_frame| {
            WorkModule::set_int(
                module_accessor,
                charge_frame,
                *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AURABALL_CHARGE_FRAME,
            );
        });
        charge.int_y.map(|prev_frame| {
            WorkModule::set_int(
                module_accessor,
                prev_frame,
                *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_PREV_AURABALL_CHARGE_FRAME,
            );
            if prev_frame == 90 {
                EffectModule::req_common(module_accessor, Hash40::new("charge_max"), 0.0);
                let effect_hash_1 = Hash40::new("lucario_hadoudan_max_l");
                let effect_hash_2 = Hash40::new("lucario_hadoudan_max_r");
                let joint_hash_1 = Hash40::new("handl");
                let joint_hash_2 = Hash40::new("handr");
                let pos = Vector3f {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                };
                let rot = Vector3f {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                };
                let efh_1 = EffectModule::req_follow(
                    module_accessor,
                    effect_hash_1,
                    joint_hash_1,
                    &pos,
                    &rot,
                    1.0,
                    false,
                    0,
                    0,
                    -1,
                    0,
                    0,
                    false,
                    false,
                );
                let efh_2 = EffectModule::req_follow(
                    module_accessor,
                    effect_hash_2,
                    joint_hash_2,
                    &pos,
                    &rot,
                    1.0,
                    false,
                    0,
                    0,
                    -1,
                    0,
                    0,
                    false,
                    false,
                );
                WorkModule::set_int(
                    module_accessor,
                    efh_1 as i32,
                    *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_EF_ID_AURABALL_MAX_L,
                );
                WorkModule::set_int(
                    module_accessor,
                    efh_2 as i32,
                    *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_EF_ID_AURABALL_MAX_R,
                );
            }
        });
        charge.has_charge.map(|has_aurasphere| {
            WorkModule::set_flag(
                module_accessor,
                has_aurasphere,
                *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_AURABALL_HAD,
            );
        });
    }
    // ROB Gyro/Laser/Fuel - Gyro from 0 to 90, rest unchecked
    else if fighter_kind == FIGHTER_KIND_ROBOT {
        charge.float_x.map(|beam_energy| {
            WorkModule::set_float(
                module_accessor,
                beam_energy,
                *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BEAM_ENERGY_VALUE,
            );
        });
        charge.float_y.map(|gyro_charge| {
            WorkModule::set_float(
                module_accessor,
                gyro_charge,
                *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_GYRO_CHARGE_VALUE,
            );
            if gyro_charge == 90.0 {
                EffectModule::req_common(module_accessor, Hash40::new("charge_max"), 0.0);
            }
        });
        charge.float_z.map(|burner_energy| {
            WorkModule::set_float(
                module_accessor,
                burner_energy,
                *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BURNER_ENERGY_VALUE,
            );
        });
    }
    // Wii Fit Sun Salutation - 0 to 1
    else if fighter_kind == FIGHTER_KIND_WIIFIT {
        charge.float_x.map(|sun_ratio| {
            WorkModule::set_float(
                module_accessor,
                sun_ratio,
                *FIGHTER_WIIFIT_INSTANCE_WORK_ID_FLOAT_SPECIAL_N_CHARGE_LEVEL_RATIO,
            )
        });
    }
    // Pac-Man Bonus Fruit - 0 to 12
    else if fighter_kind == FIGHTER_KIND_PACMAN {
        let mut has_key = false;
        charge.int_x.map(|charge_rank| {
            WorkModule::set_int(
                module_accessor,
                charge_rank,
                *FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_SPECIAL_N_CHARGE_RANK,
            );

            if charge_rank == 12 {
                EffectModule::req_common(module_accessor, Hash40::new("charge_max"), 0.0);
                has_key = true;
            }
        });
        charge.has_charge.map(|has_fruit| {
            WorkModule::set_flag(
                module_accessor,
                has_fruit,
                *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_PULL_THROW,
            );
            if has_key {
                WorkModule::set_flag(
                    module_accessor,
                    has_key,
                    *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_MAX_HAVE_ITEM,
                );
            }
        });
    }
    // Robin Thunder Tome Spells - 0 to 3
    else if fighter_kind == FIGHTER_KIND_REFLET {
        charge.int_x.map(|thunder_kind| {
            WorkModule::set_int(
                module_accessor,
                thunder_kind,
                *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_THUNDER_KIND,
            );
            if thunder_kind == 3 {
                EffectModule::req_common(module_accessor, Hash40::new("charge_max"), 0.0);
                let reflet_hash = Hash40::new("reflet_thunder_max");
                let joint_hash = Hash40::new("handl");
                let pos = Vector3f {
                    x: 1.0,
                    y: 2.0,
                    z: 0.0,
                };
                let rot = Vector3f {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                };
                EffectModule::req_follow(
                    module_accessor,
                    reflet_hash,
                    joint_hash,
                    &pos,
                    &rot,
                    1.0,
                    false,
                    0,
                    0,
                    -1,
                    0,
                    0,
                    false,
                    false,
                );
            }
        });
    }
    // Mii Gunner Charge Blast - 0 to 120
    else if fighter_kind == FIGHTER_KIND_MIIGUNNER {
        charge.int_x.map(|blast_charge| {
            WorkModule::set_int(
                module_accessor,
                blast_charge,
                *FIGHTER_MIIGUNNER_INSTANCE_WORK_ID_INT_GUNNER_CHARGE_COUNT,
            );
            if blast_charge == 120 {
                EffectModule::req_common(module_accessor, Hash40::new("charge_max"), 0.0);
                let gunner_hash = Hash40::new("miigunner_cshot_max");
                let joint_hash = Hash40::new("armr");
                let pos = Vector3f {
                    x: 6.0,
                    y: 0.0,
                    z: 0.0,
                };
                let rot = Vector3f {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                };
                let efh = EffectModule::req_follow(
                    module_accessor,
                    gunner_hash,
                    joint_hash,
                    &pos,
                    &rot,
                    1.0,
                    false,
                    0,
                    0,
                    0,
                    0,
                    0,
                    false,
                    false,
                );
                WorkModule::set_int(
                    module_accessor,
                    efh as i32,
                    *FIGHTER_MIIGUNNER_INSTANCE_WORK_ID_INT_EFH_CHARGE_MAX,
                );
            }
        });
    }
    // Plant Poison - 0 to 75
    else if fighter_kind == FIGHTER_KIND_PACKUN {
        charge.int_x.map(|poison_count| {
            WorkModule::set_int(
                module_accessor,
                poison_count,
                *FIGHTER_PACKUN_INSTANCE_WORK_ID_INT_SPECIAL_S_COUNT,
            );
            if poison_count == 75 {
                EffectModule::req_common(module_accessor, Hash40::new("charge_max"), 0.0);
                let plant_hash = Hash40::new("packun_poison_max_smoke");
                let joint_hash = Hash40::new("hip");
                let pos = Vector3f {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                };
                let rot = Vector3f {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                };
                let efh = EffectModule::req_follow(
                    module_accessor,
                    plant_hash,
                    joint_hash,
                    &pos,
                    &rot,
                    1.0,
                    false,
                    32768,
                    0,
                    -1,
                    0,
                    0,
                    false,
                    false,
                );
                WorkModule::set_int(
                    module_accessor,
                    efh as i32,
                    *FIGHTER_PACKUN_INSTANCE_WORK_ID_INT_SPECIAL_S_CHARGE_MAX_EFFECT_HANDLE,
                );
            }
        });
    }
    // Hero (Ka)frizz(le) - 0 to 81
    else if fighter_kind == FIGHTER_KIND_BRAVE {
        EffectModule::remove_common(module_accessor, Hash40::new("charge_max"));
        WorkModule::off_flag(
            module_accessor,
            *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_SPECIAL_N_MAX_EFFECT,
        );
        charge.int_x.map(|frizz_charge| {
            WorkModule::set_int(
                module_accessor,
                frizz_charge,
                *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_N_HOLD_FRAME,
            );
        });
    }
    // Banjo Wonderwing - 0 to 5
    else if fighter_kind == FIGHTER_KIND_BUDDY {
        charge.int_x.map(|wing_remain| {
            WorkModule::set_int(
                module_accessor,
                wing_remain,
                *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_S_REMAIN,
            );
        });
    }
}

unsafe fn is_kirby_hat_okay(
    opponent_module_accessor: &mut app::BattleObjectModuleAccessor,
    save_state_fighter_option: Option<i32>,
) -> Option<bool> {
    let mut opponent_fighter_kind = app::utility::get_kind(opponent_module_accessor);
    let save_state_fighter_kind = save_state_fighter_option?;
    if opponent_fighter_kind == save_state_fighter_kind {
        return Some(true);
    }
    // We have a fighter but they don't match - see if it's an accepted transformation
    let trainer_kinds = [
        *FIGHTER_KIND_PZENIGAME,
        *FIGHTER_KIND_PFUSHIGISOU,
        *FIGHTER_KIND_PLIZARDON,
        -1, // Fighter Kind while switching pokemon
    ];
    let element_kinds = [*FIGHTER_KIND_EFLAME, *FIGHTER_KIND_ELIGHT];
    if opponent_fighter_kind == -1 {
        let trainer_boid = LinkModule::get_parent_object_id(
            opponent_module_accessor,
            *FIGHTER_POKEMON_LINK_NO_PTRAINER,
        ) as u32;
        if trainer_boid != *BATTLE_OBJECT_ID_INVALID as u32
            && app::sv_battle_object::is_active(trainer_boid)
        {
            opponent_fighter_kind = *FIGHTER_KIND_PZENIGAME; // ptrainer is in the match, so assume we have a ptrainer fighter
        }
    }
    let both_trainer = trainer_kinds.contains(&opponent_fighter_kind)
        && trainer_kinds.contains(&save_state_fighter_kind);
    let both_element = element_kinds.contains(&opponent_fighter_kind)
        && element_kinds.contains(&save_state_fighter_kind);
    Some(both_trainer || both_element)
}

pub unsafe fn get_kirby_hat_charge(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    opponent_fighter_kind: i32,
    charge_state: ChargeState,
) -> ChargeState {
    if opponent_fighter_kind == FIGHTER_KIND_SAMUS || opponent_fighter_kind == FIGHTER_KIND_SAMUSD {
        let shot_charge = WorkModule::get_int(
            module_accessor,
            *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT,
        );
        charge_state.int_x(shot_charge)
    } 
    // Sheik Needles
    else if opponent_fighter_kind == FIGHTER_KIND_SHEIK {
        let my_charge = WorkModule::get_int(
            module_accessor,
            *FIGHTER_SHEIK_INSTANCE_WORK_ID_INT_NEEDLE_COUNT,
        );
        charge_state.int_x(my_charge)
    }
    // Mewtwo Shadowball
    else if opponent_fighter_kind == FIGHTER_KIND_MEWTWO {
        let my_charge = WorkModule::get_int(
            module_accessor,
            *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_SHADOWBALL_CHARGE_FRAME,
        );
        let prev_frame = WorkModule::get_int(
            module_accessor,
            *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_PREV_SHADOWBALL_CHARGE_FRAME,
        );
        let ball_had = WorkModule::is_flag(
            module_accessor,
            *FIGHTER_MEWTWO_INSTANCE_WORK_ID_FLAG_SHADOWBALL_HAD,
        );
        charge_state
            .int_x(my_charge)
            .int_y(prev_frame)
            .has_charge(ball_had)
    }
    // Squirtle Water Gun
    else if opponent_fighter_kind == FIGHTER_KIND_PZENIGAME {
        let my_charge = WorkModule::get_int(
            module_accessor,
            *FIGHTER_PZENIGAME_INSTANCE_WORK_ID_INT_SPECIAL_N_CHARGE,
        );
        charge_state.int_x(my_charge)
    }
    // Olimar Pikmin
    else if opponent_fighter_kind == FIGHTER_KIND_PIKMIN {
        let pre_pikmin_variation = WorkModule::get_int(
            module_accessor,
            *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PRE_PIKMIN_VARIATION,
        );
        let before_pre_pikmin_variation = WorkModule::get_int(
            module_accessor,
            *FIGHTER_PIKMIN_INSTANCE_WORK_INT_BEFORE_PRE_PIKMIN_VARIATION,
        );
        charge_state.int_x(pre_pikmin_variation).int_y(before_pre_pikmin_variation)
    }
    // Lucario Aura Sphere
    else if opponent_fighter_kind == FIGHTER_KIND_LUCARIO {
        let my_charge = WorkModule::get_int(
            module_accessor,
            *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AURABALL_CHARGE_FRAME,
        );
        let prev_frame = WorkModule::get_int(
            module_accessor,
            *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_PREV_AURABALL_CHARGE_FRAME,
        );
        let ball_had = WorkModule::is_flag(
            module_accessor,
            *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_AURABALL_HAD,
        );
        charge_state
            .int_x(my_charge)
            .int_y(prev_frame)
            .has_charge(ball_had)
    }
    // ROB Gyro/Laser/Fuel
    else if opponent_fighter_kind == FIGHTER_KIND_ROBOT {
        let laser_charge = WorkModule::get_float(
            module_accessor,
            *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BEAM_ENERGY_VALUE,
        );
        charge_state
            .float_x(laser_charge)
    }
    // Wii Fit Sun Salutation
    else if opponent_fighter_kind == FIGHTER_KIND_WIIFIT {
        let my_charge = WorkModule::get_float(
            module_accessor,
            *FIGHTER_WIIFIT_INSTANCE_WORK_ID_FLOAT_SPECIAL_N_CHARGE_LEVEL_RATIO,
        );
        charge_state.float_x(my_charge)
    }
    // Pac-Man Bonus Fruit
    else if opponent_fighter_kind == FIGHTER_KIND_PACMAN {
        let my_charge = WorkModule::get_int(
            module_accessor,
            *FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_SPECIAL_N_CHARGE_RANK,
        );
        let fruit_have = WorkModule::is_flag(
            module_accessor,
            *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_PULL_THROW,
        );
        charge_state.int_x(my_charge).has_charge(fruit_have)
    }
    // Robin Thunder Tome Spells
    else if opponent_fighter_kind == FIGHTER_KIND_REFLET {
        let my_charge = WorkModule::get_int(
            module_accessor,
            *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_THUNDER_KIND,
        );
        charge_state.int_x(my_charge)
    }
    // Hero (Ka)frizz(le)
    else if opponent_fighter_kind == FIGHTER_KIND_BRAVE {
        let my_charge = WorkModule::get_int(
            module_accessor,
            *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_N_HOLD_FRAME,
        );
        charge_state.int_x(my_charge)
    }
    // No charge for this character's copy ability
    else {
        charge_state
    }
}

pub unsafe fn handle_kirby_hat_charge(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    opponent_fighter_kind: i32,
    charge: ChargeState,
) {
    // Samus/Dark Samus Charge Shot - 0 to 112
    if opponent_fighter_kind == FIGHTER_KIND_SAMUS || opponent_fighter_kind == FIGHTER_KIND_SAMUSD {
        charge.int_x.map(|shot_charge| {
            WorkModule::set_int(
                module_accessor,
                shot_charge,
                *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT,
            );
            if shot_charge == 112 {
                EffectModule::req_common(module_accessor, Hash40::new("charge_max"), 0.0);
                let samus_cshot_hash = if opponent_fighter_kind == FIGHTER_KIND_SAMUS {
                    Hash40::new("samus_cshot_max")
                } else {
                    Hash40::new("samusd_cshot_max")
                };
                let joint_hash = Hash40::new("handr");
                let pos = Vector3f {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                };
                let rot = Vector3f {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                };
                let efh = EffectModule::req_follow(
                        module_accessor,
                        samus_cshot_hash,
                        joint_hash,
                        &pos,
                        &rot,
                        1.0,
                        false,
                        0,
                        0,
                        0,
                        0,
                        0,
                        false,
                        false,
                );
                WorkModule::set_int(
                    module_accessor,
                    efh as i32,
                    *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_EFH_CHARGE_MAX,
                );
            }
        });
    }
    // Sheik Needles - 0 to 6
    else if opponent_fighter_kind == FIGHTER_KIND_SHEIK {
        charge.int_x.map(|needle_charge| {
            WorkModule::set_int(
                module_accessor,
                needle_charge,
                *FIGHTER_SHEIK_INSTANCE_WORK_ID_INT_NEEDLE_COUNT,
            );
            if needle_charge == 6 {
                EffectModule::req_common(module_accessor, Hash40::new("charge_max"), 0.0);
            }
        });
    }
    // Mewtwo Shadowball - 0 to 120, Boolean (not working)
    else if opponent_fighter_kind == FIGHTER_KIND_MEWTWO {
        charge.int_x.map(|charge_frame| {
            WorkModule::set_int(
                module_accessor,
                charge_frame,
                *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_SHADOWBALL_CHARGE_FRAME,
            );
        });
        charge.int_y.map(|prev_frame| {
            /*WorkModule::set_int(
                module_accessor,
                prev_frame,
                *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_PREV_SHADOWBALL_CHARGE_FRAME,
            );*/
            if prev_frame == 120 {
                EffectModule::req_common(module_accessor, Hash40::new("charge_max"), 0.0);
                let pos = Vector3f {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                };
                let rot = Vector3f {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                };
                let eff_hash = Hash40{hash:0x1ac6d446d8};
                let joint_hash_l = Hash40{hash:0x5e008fd84};
                let efh_l = EffectModule::req_follow(
                        module_accessor,
                        eff_hash,
                        joint_hash_l,
                        &pos,
                        &rot,
                        1.0,
                        false,
                        0,
                        0,
                        -1,
                        0,
                        0,
                        false,
                        false,
                );
                let joint_hash_r = Hash40{hash:0x51a07c0e7};
                let efh_r = EffectModule::req_follow(
                        module_accessor,
                        eff_hash,
                        joint_hash_r,
                        &pos,
                        &rot,
                        1.0,
                        false,
                        0,
                        0,
                        -1,
                        0,
                        0,
                        false,
                        false,
                );
                WorkModule::set_int(
                    module_accessor,
                    efh_l as i32,
                    *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_EF_ID_SHADOWBALL_MAX_L,
                );
                WorkModule::set_int(
                    module_accessor,
                    efh_r as i32,
                    *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_EF_ID_SHADOWBALL_MAX_R,
                );
            }
        });
    }
    // Squirtle Water Gun - 0 to 45
    else if opponent_fighter_kind == FIGHTER_KIND_PZENIGAME {
        charge.int_x.map(|water_charge| {
            WorkModule::set_int(
                module_accessor,
                water_charge,
                *FIGHTER_PZENIGAME_INSTANCE_WORK_ID_INT_SPECIAL_N_CHARGE,
            );
            if water_charge == 45 {
                EffectModule::req_common(module_accessor, Hash40::new("charge_max"), 0.0);
            }
        });
    }
    // Olimar Pikmin - 0 to 4 (not working)
    else if opponent_fighter_kind == FIGHTER_KIND_PIKMIN {
        charge.int_x.map(|pre| {
            WorkModule::set_int(
                module_accessor,
                pre,
                *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PRE_PIKMIN_VARIATION,
            );
        });
        charge.int_y.map(|before_pre| {
            WorkModule::set_int(
                module_accessor,
                before_pre,
                *FIGHTER_PIKMIN_INSTANCE_WORK_INT_BEFORE_PRE_PIKMIN_VARIATION,
            );
        });
    }
    // ROB Laster (Not Working)
    else if opponent_fighter_kind == FIGHTER_KIND_ROBOT {
        charge.float_x.map(|beam_energy| {
            WorkModule::set_float(
                module_accessor,
                beam_energy,
                *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BEAM_ENERGY_VALUE,
            );
        });
    }
    // Wii Fit Sun Salutation - 0 to 1 (Not Working) (Charge Effects show up, but sun is uncharged???)
    else if opponent_fighter_kind == FIGHTER_KIND_WIIFIT {
        charge.float_x.map(|sun_ratio| {
            WorkModule::set_float(
                module_accessor,
                sun_ratio,
                *FIGHTER_WIIFIT_INSTANCE_WORK_ID_FLOAT_SPECIAL_N_CHARGE_LEVEL_RATIO,
            )
        });
    }
    // Pac-Man Bonus Fruit - 0 to 12 (We correctly apply common effect, but fruit charge doesn't save)
    else if opponent_fighter_kind == FIGHTER_KIND_PACMAN {
        let mut has_key = false;
        charge.int_x.map(|charge_rank| {
            WorkModule::set_int(
                module_accessor,
                charge_rank,
                *FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_SPECIAL_N_CHARGE_RANK,
            );

            if charge_rank == 12 {
                EffectModule::req_common(module_accessor, Hash40::new("charge_max"), 0.0);
                has_key = true;
            }
        });
        charge.has_charge.map(|has_fruit| {
            WorkModule::set_flag(
                module_accessor,
                has_fruit,
                *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_PULL_THROW,
            );
            if has_key {
                WorkModule::set_flag(
                    module_accessor,
                    has_key,
                    *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_MAX_HAVE_ITEM,
                );
            }
        });
    }
    // Robin Thunder Tome Spells - 0 to 3 (Not Working)
    else if opponent_fighter_kind == FIGHTER_KIND_REFLET {
        charge.int_x.map(|thunder_kind| {
            WorkModule::set_int(
                module_accessor,
                thunder_kind,
                *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_THUNDER_KIND,
            );
            if thunder_kind == 3 {
                EffectModule::req_common(module_accessor, Hash40::new("charge_max"), 0.0);
                let eff_hash = Hash40{hash:0x12db3e4172};
                let joint_hash = Hash40{hash:0x5eb263e0d};
                let pos = Vector3f {
                        x: 0,
                        y: 0,
                        z: 0,
                };
                let rot = Vector3f {
                        x: 0,
                        y: 0,
                        z: 0,
                };
                let efh = EffectModule::req_follow(
                        module_accessor,
                        charge_hash,
                        joint_hash,
                        &pos,
                        &rot,
                        1,
                        false,
                        0,
                        0,
                        -1,
                        0,
                        0,
                        false,
                        false,
                );
            }
        });
    }
    // Hero (Ka)frizz(le) - 0 to 81
    else if opponent_fighter_kind == FIGHTER_KIND_BRAVE {
        EffectModule::remove_common(module_accessor, Hash40::new("charge_max"));
        WorkModule::off_flag(
            module_accessor,
            *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_SPECIAL_N_MAX_EFFECT,
        );
        charge.int_x.map(|frizz_charge| {
            WorkModule::set_int(
                module_accessor,
                frizz_charge,
                *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_N_HOLD_FRAME,
            );
        });
    }
}