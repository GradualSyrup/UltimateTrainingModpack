use smash::app;
use crate::common::consts::*;
use crate::common::*;
use smash::app::lua_bind::*;
use smash::app::{ArticleOperationTarget, BattleObjectModuleAccessor, Item, ItemKind, FighterModuleAccessor};
use smash::cpp::l2c_value::LuaConst;
use smash::lib::lua_const::*;

pub struct CharItem {
    pub fighter_kind: LuaConst,
    pub item_kind: Option<LuaConst>,
    pub article_kind: Option<LuaConst>,
    pub variation: Option<LuaConst>,
}

pub const ALL_CHAR_ITEMS: [CharItem; 45] = [
    CharItem {
        fighter_kind: FIGHTER_KIND_DIDDY,
        item_kind: None,
        article_kind: Some(FIGHTER_DIDDY_GENERATE_ARTICLE_ITEM_BANANA),
        variation: None,
    },
    CharItem {
        // Robin Tome
        fighter_kind: FIGHTER_KIND_REFLET,
        item_kind: Some(ITEM_KIND_BOOK),
        article_kind: None,
        variation: None, // TODO: Look at the lua const ITEM_BOOK_STATUS_KIND_BEFORE_BORN
    },
    CharItem {
        // Banjo-Kazooie Grenade Egg
        fighter_kind: FIGHTER_KIND_BUDDY,
        item_kind: Some(ITEM_KIND_BUDDYBOMB),
        article_kind: None,
        variation: None,
    },
    CharItem {
        // Turnip
        fighter_kind: FIGHTER_KIND_DAISY,
        item_kind: None,
        article_kind: Some(FIGHTER_DAISY_GENERATE_ARTICLE_DAIKON),
        variation: Some(ITEM_VARIATION_DAISYDAIKON_1), // Smile
    },
    CharItem {
        // Turnip
        fighter_kind: FIGHTER_KIND_DAISY,
        item_kind: None,
        article_kind: Some(FIGHTER_DAISY_GENERATE_ARTICLE_DAIKON),
        variation: Some(ITEM_VARIATION_DAISYDAIKON_6), // Winky
    },
    CharItem {
        // Turnip
        fighter_kind: FIGHTER_KIND_DAISY,
        item_kind: None,
        article_kind: Some(FIGHTER_DAISY_GENERATE_ARTICLE_DAIKON),
        variation: Some(ITEM_VARIATION_DAISYDAIKON_7), // Dot-Eyes
    },
    CharItem {
        // Turnip
        fighter_kind: FIGHTER_KIND_DAISY,
        item_kind: None,
        article_kind: Some(FIGHTER_DAISY_GENERATE_ARTICLE_DAIKON),
        variation: Some(ITEM_VARIATION_DAISYDAIKON_8), // Stitch-face
    },
    CharItem {
        // Mr Saturn
        fighter_kind: FIGHTER_KIND_DAISY,
        item_kind: Some(ITEM_KIND_DOSEISAN),
        article_kind: None,
        variation: None,
    },
    CharItem {
        // Bob-omb
        fighter_kind: FIGHTER_KIND_DAISY,
        item_kind: Some(ITEM_KIND_BOMBHEI),
        article_kind: None,
        variation: Some(ITEM_VARIATION_BOMBHEI_NORMAL),
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_DIDDY,
        item_kind: Some(ITEM_KIND_DIDDYPEANUTS),
        article_kind: None,
        variation: None,
    },
    CharItem {
        // Sheik Sideb Bomb
        fighter_kind: FIGHTER_KIND_SHEIK,
        item_kind: Some(ITEM_KIND_EXPLOSIONBOMB),
        article_kind: None,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_KROOL,
        item_kind: Some(ITEM_KIND_KROOLCROWN),
        article_kind: None,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_LINK,
        item_kind: Some(ITEM_KIND_LINKARROW),
        article_kind: None,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_LINK,
        item_kind: Some(ITEM_KIND_LINKBOMB),
        article_kind: None,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_KOOPAJR,
        item_kind: Some(ITEM_KIND_MECHAKOOPA),
        article_kind: None,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_ROCKMAN,
        item_kind: None, // Some(ITEM_KIND_METALBLADE),
        article_kind: Some(FIGHTER_ROCKMAN_GENERATE_ARTICLE_METALBLADE),
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_PACMAN,
        item_kind: Some(ITEM_KIND_PACMANCHERRY),
        article_kind: None,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_PACMAN,
        item_kind: Some(ITEM_KIND_PACMANSTRAWBERRY),
        article_kind: None,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_PACMAN,
        item_kind: Some(ITEM_KIND_PACMANORANGE),
        article_kind: None,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_PACMAN,
        item_kind: Some(ITEM_KIND_PACMANAPPLE),
        article_kind: None,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_PACMAN,
        item_kind: Some(ITEM_KIND_PACMANMELON),
        article_kind: None,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_PACMAN,
        item_kind: Some(ITEM_KIND_PACMANBOSS),
        article_kind: None,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_PACMAN,
        item_kind: Some(ITEM_KIND_PACMANBELL),
        article_kind: None,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_PACMAN,
        item_kind: Some(ITEM_KIND_PACMANKEY),
        article_kind: None,
        variation: None,
    },
    CharItem {
        // Turnip
        fighter_kind: FIGHTER_KIND_PEACH,
        item_kind: None,
        article_kind: Some(FIGHTER_PEACH_GENERATE_ARTICLE_DAIKON),
        variation: Some(ITEM_VARIATION_PEACHDAIKON_1), // Smile
    },
    CharItem {
        // Turnip
        fighter_kind: FIGHTER_KIND_PEACH,
        item_kind: None,
        article_kind: Some(FIGHTER_PEACH_GENERATE_ARTICLE_DAIKON),
        variation: Some(ITEM_VARIATION_PEACHDAIKON_6), // Winky
    },
    CharItem {
        // Turnip
        fighter_kind: FIGHTER_KIND_PEACH,
        item_kind: None,
        article_kind: Some(FIGHTER_PEACH_GENERATE_ARTICLE_DAIKON),
        variation: Some(ITEM_VARIATION_PEACHDAIKON_7), // Dot-Eyes
    },
    CharItem {
        // Turnip
        fighter_kind: FIGHTER_KIND_PEACH,
        item_kind: None,
        article_kind: Some(FIGHTER_PEACH_GENERATE_ARTICLE_DAIKON),
        variation: Some(ITEM_VARIATION_PEACHDAIKON_8), // Stitch-face
    },
    CharItem {
        // Mr Saturn
        fighter_kind: FIGHTER_KIND_PEACH,
        item_kind: Some(ITEM_KIND_DOSEISAN),
        article_kind: None,
        variation: None,
    },
    CharItem {
        // Bob-omb
        fighter_kind: FIGHTER_KIND_PEACH,
        item_kind: Some(ITEM_KIND_BOMBHEI),
        article_kind: None,
        variation: Some(ITEM_VARIATION_BOMBHEI_NORMAL),
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_RICHTER,
        item_kind: Some(ITEM_KIND_RICHTERHOLYWATER),
        article_kind: None,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_ROBOT,
        item_kind: Some(ITEM_KIND_ROBOTGYRO),
        article_kind: None,
        variation: Some(ITEM_VARIATION_ROBOTGYRO_1P),
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_ROBOT,
        item_kind: Some(ITEM_KIND_ROBOTGYRO),
        article_kind: None,
        variation: Some(ITEM_VARIATION_ROBOTGYRO_2P),
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_ROBOT,
        item_kind: Some(ITEM_KIND_ROBOTGYRO),
        article_kind: None,
        variation: Some(ITEM_VARIATION_ROBOTGYRO_3P),
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_ROBOT,
        item_kind: Some(ITEM_KIND_ROBOTGYRO),
        article_kind: None,
        variation: Some(ITEM_VARIATION_ROBOTGYRO_4P),
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_ROBOT,
        item_kind: Some(ITEM_KIND_ROBOTGYRO),
        article_kind: None,
        variation: Some(ITEM_VARIATION_ROBOTGYRO_5P),
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_ROBOT,
        item_kind: Some(ITEM_KIND_ROBOTGYRO),
        article_kind: None,
        variation: Some(ITEM_VARIATION_ROBOTGYRO_6P),
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_ROBOT,
        item_kind: Some(ITEM_KIND_ROBOTGYRO),
        article_kind: None,
        variation: Some(ITEM_VARIATION_ROBOTGYRO_7P),
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_ROBOT,
        item_kind: Some(ITEM_KIND_ROBOTGYRO),
        article_kind: None,
        variation: Some(ITEM_VARIATION_ROBOTGYRO_8P),
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_SIMON,
        item_kind: Some(ITEM_KIND_SIMONHOLYWATER),
        article_kind: None,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_SNAKE,
        item_kind: Some(ITEM_KIND_SNAKEGRENADE),
        article_kind: None,
        variation: None,
    },
    // CharItem {
    //     // Cardboard Box from Taunt
    //     fighter_kind: FIGHTER_KIND_SNAKE,
    //     item_kind: Some(ITEM_KIND_SNAKECBOX),
    //     article_kind: None,
    //     variation: None,
    // },
    CharItem {
        // Robin Levin Sword
        fighter_kind: FIGHTER_KIND_REFLET,
        item_kind: Some(ITEM_KIND_THUNDERSWORD),
        article_kind: None,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_TOONLINK,
        item_kind: Some(ITEM_KIND_TOONLINKBOMB),
        article_kind: None,
        variation: None,
    },
    // CharItem {
    //     fighter_kind: FIGHTER_KIND_WARIO,
    //     item_kind: Some(ITEM_KIND_WARIOBIKE),
    //     // Pretty sure these other ones are just the bike parts
    //     // ITEM_KIND_WARIOBIKEA,
    //     // ITEM_KIND_WARIOBIKEB,
    //     // ITEM_KIND_WARIOBIKEC,
    //     // ITEM_KIND_WARIOBIKED,
    //     // ITEM_KIND_WARIOBIKEE,
    //     article_kind: None,
    //     variation: None,
    // },
    CharItem {
        // Villager Wood Chip
        fighter_kind: FIGHTER_KIND_MURABITO,
        item_kind: Some(ITEM_KIND_WOOD),
        article_kind: None,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_YOUNGLINK,
        item_kind: Some(ITEM_KIND_YOUNGLINKBOMB),
        article_kind: None,
        variation: None,
    },
];

pub static mut TURNIP_CHOSEN : Option<u32> = None;
pub static mut TARGET_PLAYER : Option<*mut BattleObjectModuleAccessor> = None;

unsafe fn apply_single_item(player_fighter_kind: i32,
                            _cpu_fighter_kind: i32,
                            item: &CharItem) {
    let player_module_accessor = get_module_accessor(FighterId::Player);
    let cpu_module_accessor = get_module_accessor(FighterId::CPU);
    // Now we make sure the module_accessor we use to generate the item/article is the correct character
    let generator_module_accessor = if *(item.fighter_kind) == player_fighter_kind { player_module_accessor } else { cpu_module_accessor };
    if *(item.fighter_kind) == player_fighter_kind {
        println!("generator is player!");
    } else {
        println!("generator is cpu!");
        println!("item.fighter_kind = {}, player_fighter_kind = {}",*(item.fighter_kind),player_fighter_kind);
    }
    let variation = item.variation.as_ref()
        .map(|v| **v)
        .unwrap_or(0);
    item.item_kind.as_ref().map(|item_kind| {
        let item_kind = **item_kind;
        // For Link, use special article generation to link the bomb for detonation
        if item_kind == *ITEM_KIND_LINKBOMB {
            ArticleModule::generate_article_have_item(
                generator_module_accessor,
                *FIGHTER_LINK_GENERATE_ARTICLE_LINKBOMB,
                *FIGHTER_HAVE_ITEM_WORK_MAIN,
                smash::phx::Hash40::new("invalid")
            );
            if player_fighter_kind != *FIGHTER_KIND_LINK { // have the cpu drop the item if they generated it, and then have player pick it up
                ItemModule::drop_item(cpu_module_accessor, 0.0, 0.0, 0);
                //ItemModule::eject_have_item(cpu_module_accessor, 0, false, false);
                let item_mgr = *(ITEM_MANAGER_ADDR as *mut *mut app::ItemManager);
                let item_ptr = ItemManager::get_active_item(item_mgr, 0);
                ItemModule::have_item_instance(player_module_accessor,
                    item_ptr as *mut smash::app::Item, 0, false, false, false, false);
                
            }
        } else {
            //start WIP book
            if item_kind == *ITEM_KIND_BOOK {
                /*ArticleModule::generate_article_have_item(
                    player_module_accessor,
                    0, // this should be the book
                    *FIGHTER_HAVE_ITEM_WORK_MAIN,
                    smash::phx::Hash40::new("invalid")
                );*/
                //ItemModule::born_item(player_module_accessor, 0);
                /*
                let item_mgr = *(ITEM_MANAGER_ADDR as *mut *mut app::ItemManager);
                let item_ptr = ItemManager::get_active_item(item_mgr, 0);
                ItemModule::have_item_instance(player_module_accessor,
                item_ptr as *mut smash::app::Item, 0, false, false, false, false);
                */
                // create FighterModuleAccessor
                //let mut fighter_accessor = FighterModuleAccessor {battle_object_module_accessor: *generator_module_accessor};
                FighterSpecializer_Reflet::set_flag_to_table(generator_module_accessor as *mut FighterModuleAccessor, *FIGHTER_REFLET_MAGIC_KIND_GIGA_FIRE, true, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THROWAWAY_TABLE);
                //FighterSpecializer_Reflet::set_flag_to_table(generator_module_accessor as *mut FighterModuleAccessor, *FIGHTER_REFLET_MAGIC_KIND_THUNDER, false, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THROWAWAY_TABLE);
                    
            } else {
                ItemModule::have_item(player_module_accessor, ItemKind(item_kind),
                variation, 0, false, false);
            }

            //end
            
            
            
            //ItemModule::have_item(player_module_accessor, ItemKind(item_kind),
            //variation, 0, false, false);
        }
    });

    item.article_kind.as_ref().map(|article_kind| {
        TURNIP_CHOSEN = if [*ITEM_VARIATION_PEACHDAIKON_8, *ITEM_VARIATION_DAISYDAIKON_8]
            .contains(&variation) {
            Some(8)
        } else if [*ITEM_VARIATION_PEACHDAIKON_7, *ITEM_VARIATION_DAISYDAIKON_7]
            .contains(&variation) {
            Some(7)
        } else if [*ITEM_VARIATION_PEACHDAIKON_6, *ITEM_VARIATION_DAISYDAIKON_6]
            .contains(&variation) {
            Some(6)
        } else if [*ITEM_VARIATION_PEACHDAIKON_1, *ITEM_VARIATION_DAISYDAIKON_1]
            .contains(&variation) {
            Some(1)
        } else {
            None
        };

        let article_kind = **article_kind;
        if article_kind == FIGHTER_DIDDY_GENERATE_ARTICLE_ITEM_BANANA {
            ArticleModule::generate_article_have_item(
            generator_module_accessor,
            *FIGHTER_DIDDY_GENERATE_ARTICLE_ITEM_BANANA,
            *FIGHTER_HAVE_ITEM_WORK_MAIN,
            smash::phx::Hash40::new("invalid")
            );
            WorkModule::on_flag(generator_module_accessor,
                *FIGHTER_DIDDY_STATUS_SPECIAL_LW_FLAG_ITEM_THROW);
            ArticleModule::shoot(generator_module_accessor,
                *FIGHTER_DIDDY_GENERATE_ARTICLE_ITEM_BANANA,
                ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL),
                false
            );
            // Grab item from the middle of the stage where it gets shot
            let item_mgr = *(ITEM_MANAGER_ADDR as *mut *mut app::ItemManager);
            let item = ItemManager::get_active_item(item_mgr, 0);
            ItemModule::have_item_instance(player_module_accessor,
                item as *mut Item, 0, false, false, false, false);
        } else {
            TARGET_PLAYER = Some(player_module_accessor); // set so we generate CPU article on the player (in dittos, items always belong to player, even if cpu item is chosen)
            ArticleModule::generate_article(generator_module_accessor, // we want CPU's article
                                            article_kind,
                                            false,
                                            0
            );
            TARGET_PLAYER = None;
        }
        TURNIP_CHOSEN = None;
    });
}

pub unsafe fn apply_item(character_item: CharacterItem) {
    let player_module_accessor = get_module_accessor(FighterId::Player);
    let cpu_module_accessor = get_module_accessor(FighterId::CPU);
    let player_fighter_kind = app::utility::get_kind(&mut *player_module_accessor);
    let cpu_fighter_kind = app::utility::get_kind(&mut *cpu_module_accessor);
    let character_item_num = character_item.as_idx();
    let (item_fighter_kind, variation_idx) =
        if character_item_num <= CharacterItem::PlayerVariation8.as_idx() {
            (player_fighter_kind, (character_item_num - CharacterItem::PlayerVariation1.as_idx()) as usize)
        } else {
            (cpu_fighter_kind, (character_item_num - CharacterItem::CpuVariation1.as_idx()) as usize)
        };
    ALL_CHAR_ITEMS.iter()
        .filter(|item| item_fighter_kind == item.fighter_kind)
        .nth(variation_idx)
        .map(|item|
            apply_single_item(player_fighter_kind, cpu_fighter_kind, item));
}

macro_rules! daikon_replace {
    ($caps_char: ident, $char:ident, $num:literal) => {
        paste::paste! {
            extern "C" {
                #[link_name = "\u{1}_ZN3app11" $char "daikon31" $caps_char "_" $caps_char "DAIKON_DAIKON_" $num "_PROBEv"]
                pub fn [<$char daikon_ $num _prob>]() -> f32;
            }

            #[skyline::hook(replace = [<$char daikon_ $num _prob>])]
            pub unsafe fn [<handle_ $char daikon_ $num _prob>]() -> f32 {
                let orig = original!()();
                if is_training_mode() {
                    if TURNIP_CHOSEN == Some($num) {
                        return 58.0;
                    } else if TURNIP_CHOSEN != None {
                        return 0.0;
                    }
                }

                orig
            }
        }
    };
}

daikon_replace!(PEACH, peach, 8);
daikon_replace!(PEACH, peach, 7);
daikon_replace!(PEACH, peach, 6);
daikon_replace!(PEACH, peach, 5);
daikon_replace!(PEACH, peach, 4);
daikon_replace!(PEACH, peach, 3);
daikon_replace!(PEACH, peach, 2);
daikon_replace!(PEACH, peach, 1);
daikon_replace!(DAISY, daisy, 8);
daikon_replace!(DAISY, daisy, 7);
daikon_replace!(DAISY, daisy, 6);
daikon_replace!(DAISY, daisy, 5);
daikon_replace!(DAISY, daisy, 4);
daikon_replace!(DAISY, daisy, 3);
daikon_replace!(DAISY, daisy, 2);
daikon_replace!(DAISY, daisy, 1);

// GenerateArticleForTarget for Peach/Diddy(/Link?) item creation
static GAFT_OFFSET: usize = 0x03d40a0;
#[skyline::hook(offset = GAFT_OFFSET)]
pub unsafe fn handle_generate_article_for_target(
    article_module_accessor: *mut app::BattleObjectModuleAccessor,
    int_1: i32,
    module_accessor: *mut app::BattleObjectModuleAccessor, // this is always 0x0 normally
    bool_1: bool,
    int_2: i32,
) -> u64 { // unknown return value, gets cast to an (Article *)
    //println!("Custom Article Generation! Generating Article.");
    let target_module_accessor = TARGET_PLAYER.unwrap_or(module_accessor);
    let ori = original!()(article_module_accessor, int_1, target_module_accessor, bool_1, int_2);
    //println!("Article Generated!");
    return ori;
}

// RegisterArticle for Peach/Diddy(/Link?) item creation
static REG_ART_OFFSET: usize = 0x03d5e20;
#[skyline::hook(offset = REG_ART_OFFSET)]
pub unsafe fn handle_register_article(
    article_module_accessor: *mut app::BattleObjectModuleAccessor,
    article: *mut app::Article, // should this lua_bind? assume not but maybe?
) -> u64 { // unknown return value (if it has one)
    //println!("Registering Article!");
    let ori = original!()(article_module_accessor, article);
    //println!("Article Registered! (or invalid)");
    // check here if article invalid?
    return ori;
}


// Robin Vtable Book Func
pub mod FighterSpecializer_Reflet {
    extern "C" {
        #[link_name = "\u{1}_ZN3app25FighterSpecializer_Reflet17set_flag_to_tableERNS_21FighterModuleAccessorEibi"]
        pub fn set_flag_to_table(
            module_accessor: *mut smash::app::FighterModuleAccessor,
            magic_kind: i32,
            bool_1: bool,
            table: i32,
        ) -> u64;
    }
}

#[skyline::hook(replace = FighterSpecializer_Reflet::set_flag_to_table)] // returns status of summon dispatch if triggered, -1 as u64 otherwise
pub unsafe fn set_flag_to_table(
    module_accessor: *mut FighterModuleAccessor,
    magic_kind: i32,
    bool_1: bool,
    table: i32,
) -> u64 {
    if (table == *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THROWAWAY_TABLE) {
        println!("Throwaway Table! Magic Kind: {}, Bool: {}",magic_kind, bool_1);
    }
    //skyline::logging::print_stack_trace();
    let ori = original!()(module_accessor, magic_kind, bool_1, table); 
    return ori;
}

pub fn init() {
    skyline::install_hooks!(
        handle_peachdaikon_8_prob,
        handle_peachdaikon_7_prob,
        handle_peachdaikon_6_prob,
        handle_peachdaikon_5_prob,
        handle_peachdaikon_4_prob,
        handle_peachdaikon_3_prob,
        handle_peachdaikon_2_prob,
        handle_peachdaikon_1_prob,
        handle_daisydaikon_8_prob,
        handle_daisydaikon_7_prob,
        handle_daisydaikon_6_prob,
        handle_daisydaikon_5_prob,
        handle_daisydaikon_4_prob,
        handle_daisydaikon_3_prob,
        handle_daisydaikon_2_prob,
        handle_daisydaikon_1_prob,
        // Items
        handle_generate_article_for_target,
        handle_register_article,
        set_flag_to_table,
    );
}