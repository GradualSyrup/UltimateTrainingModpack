use crate::common::consts::*;
use crate::common::*;
use smash::app;
use smash::app::lua_bind::*;
use smash::app::utility;
use smash::cpp::l2c_value::LuaConst;
use smash::lib::lua_const::*;
use smash::app::ItemKind;

// We need to store which fighter kinds exist in the match here

static mut FIGHTER_KINDS: [i32; 2] = [-1; 2]; // Does this reset if you close training mode then open it back up? Assume so

pub fn store_fighter_kinds() {
    unsafe {
        FIGHTER_KINDS[0] = utility::get_kind(&mut *get_module_accessor(FighterId::Player));
        FIGHTER_KINDS[1] = utility::get_kind(&mut *get_module_accessor(FighterId::CPU));
    }
}

// Define struct to store item info

pub struct CharItem {
    fighter_kind: LuaConst,
    item_kind: LuaConst,
    variation: Option<LuaConst>,
}

pub const CHARITEM_ALL: [CharItem; 41] = [ // Make a Vector?
    CharItem {
        fighter_kind: FIGHTER_KIND_DIDDY,
        item_kind: ITEM_KIND_BANANA,
        variation: None,
    },
    CharItem {
        // Robin Tome
        fighter_kind: FIGHTER_KIND_REFLET,
        item_kind: ITEM_KIND_BOOK,
        variation: None, // TODO: Look at the lua const ITEM_BOOK_STATUS_KIND_BEFORE_BORN
    },
    CharItem {
        // Banjo-Kazooie Grenade Egg
        fighter_kind: FIGHTER_KIND_BUDDY,
        item_kind: ITEM_KIND_BUDDYBOMB,
        variation: None,
    },
    CharItem {
        // Turnip
        fighter_kind: FIGHTER_KIND_DAISY,
        item_kind: ITEM_KIND_DAISYDAIKON,
        variation: Some(ITEM_VARIATION_DAISYDAIKON_1), // Smile
    },
    CharItem {
        // Turnip
        fighter_kind: FIGHTER_KIND_DAISY,
        item_kind: ITEM_KIND_DAISYDAIKON,
        variation: Some(ITEM_VARIATION_DAISYDAIKON_6), // Winky
    },
    CharItem {
        // Turnip
        fighter_kind: FIGHTER_KIND_DAISY,
        item_kind: ITEM_KIND_DAISYDAIKON,
        variation: Some(ITEM_VARIATION_DAISYDAIKON_7), // Dot-Eyes
    },
    CharItem {
        // Turnip
        fighter_kind: FIGHTER_KIND_DAISY,
        item_kind: ITEM_KIND_DAISYDAIKON,
        variation: Some(ITEM_VARIATION_DAISYDAIKON_8), // Stitch-face
    },
    CharItem {
        // Mr Saturn
        fighter_kind: FIGHTER_KIND_DAISY,
        item_kind: ITEM_KIND_DOSEISAN,
        variation: None,
    },
    CharItem {
        // Bob-omb
        fighter_kind: FIGHTER_KIND_DAISY,
        item_kind: ITEM_KIND_BOMBHEI,
        variation: Some(ITEM_VARIATION_BOMBHEI_NORMAL),
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_DIDDY,
        item_kind: ITEM_KIND_DIDDYPEANUTS,
        variation: None,
    },
    CharItem {
        // Sheik Sideb Bomb
        fighter_kind: FIGHTER_KIND_SHEIK,
        item_kind: ITEM_KIND_EXPLOSIONBOMB,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_KROOL,
        item_kind: ITEM_KIND_KROOLCROWN,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_LINK,
        item_kind: ITEM_KIND_LINKARROW,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_LINK,
        item_kind: ITEM_KIND_LINKBOMB,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_KOOPAJR,
        item_kind: ITEM_KIND_MECHAKOOPA,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_ROCKMAN,
        item_kind: ITEM_KIND_METALBLADE,
        variation: None,
    },
    CharItem {
        // Villager Apple
        fighter_kind: FIGHTER_KIND_MURABITO,
        item_kind: ITEM_KIND_MURABITOFRUIT,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_PACMAN,
        item_kind: ITEM_KIND_PACMANAPPLE,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_PACMAN,
        item_kind: ITEM_KIND_PACMANBELL,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_PACMAN,
        item_kind: ITEM_KIND_PACMANBOSS,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_PACMAN,
        item_kind: ITEM_KIND_PACMANCHERRY,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_PACMAN,
        item_kind: ITEM_KIND_PACMANKEY,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_PACMAN,
        item_kind: ITEM_KIND_PACMANMELON,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_PACMAN,
        item_kind: ITEM_KIND_PACMANORANGE,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_PACMAN,
        item_kind: ITEM_KIND_PACMANSTRAWBERRY,
        variation: None,
    },
    CharItem {
        // Turnip
        fighter_kind: FIGHTER_KIND_PEACH,
        item_kind: ITEM_KIND_PEACHDAIKON,
        variation: Some(ITEM_VARIATION_PEACHDAIKON_1), // Smile
    },
    CharItem {
        // Turnip
        fighter_kind: FIGHTER_KIND_PEACH,
        item_kind: ITEM_KIND_PEACHDAIKON,
        variation: Some(ITEM_VARIATION_PEACHDAIKON_6), // Winky
    },
    CharItem {
        // Turnip
        fighter_kind: FIGHTER_KIND_PEACH,
        item_kind: ITEM_KIND_PEACHDAIKON,
        variation: Some(ITEM_VARIATION_PEACHDAIKON_7), // Dot-Eyes
    },
    CharItem {
        // Turnip
        fighter_kind: FIGHTER_KIND_PEACH,
        item_kind: ITEM_KIND_PEACHDAIKON,
        variation: Some(ITEM_VARIATION_PEACHDAIKON_8), // Stitch-face
    },
    CharItem {
        // Mr Saturn
        fighter_kind: FIGHTER_KIND_PEACH,
        item_kind: ITEM_KIND_DOSEISAN,
        variation: None,
    },
    CharItem {
        // Bob-omb
        fighter_kind: FIGHTER_KIND_PEACH,
        item_kind: ITEM_KIND_BOMBHEI,
        variation: Some(ITEM_VARIATION_BOMBHEI_NORMAL),
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_RICHTER,
        item_kind: ITEM_KIND_RICHTERHOLYWATER,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_ROBOT,
        item_kind: ITEM_KIND_ROBOTGYRO,
        variation: Some(ITEM_VARIATION_ROBOTGYRO_1P),
    }, // Red and White Gyro, only one required.
    CharItem {
        fighter_kind: FIGHTER_KIND_SIMON,
        item_kind: ITEM_KIND_SIMONHOLYWATER,
        variation: None,
    },
    CharItem {
        // Cardboard Box from Taunt
        fighter_kind: FIGHTER_KIND_SNAKE,
        item_kind: ITEM_KIND_SNAKECBOX,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_SNAKE,
        item_kind: ITEM_KIND_SNAKEGRENADE,
        variation: None,
    },
    CharItem {
        // Robin Levin Sword
        fighter_kind: FIGHTER_KIND_REFLET,
        item_kind: ITEM_KIND_THUNDERSWORD,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_TOONLINK,
        item_kind: ITEM_KIND_TOONLINKBOMB,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_WARIO,
        item_kind: ITEM_KIND_WARIOBIKE,
        // Pretty sure these other ones are just the bike parts
        // ITEM_KIND_WARIOBIKEA,
        // ITEM_KIND_WARIOBIKEB,
        // ITEM_KIND_WARIOBIKEC,
        // ITEM_KIND_WARIOBIKED,
        // ITEM_KIND_WARIOBIKEE,
        variation: None,
    },
    CharItem {
        // Villager Wood Chip
        fighter_kind: FIGHTER_KIND_MURABITO,
        item_kind: ITEM_KIND_WOOD,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_YOUNGLINK,
        item_kind: ITEM_KIND_YOUNGLINKBOMB,
        variation: None,
    },
];

// Handle whether to give an item or not (and then give the item) when loading a save state
pub unsafe fn handle_state_item(module_accessor: &mut app::BattleObjectModuleAccessor) { // should handle menu item inside function, not pass
    // We first want to check if any items are selected. If no item menu options are selected, don't waste any time in here.
    let menu_state;
    unsafe {
        menu_state = MENU.character_item;
    }

    if menu_state == CharacterItem::None { // Is it worth having this here to save a few calls, or does it add clutter since we have a match in a few lines?
        return;
    }
    // If we're moving forward, we want to make sure the fighters we have stored are correct
    //if FIGHTER_KINDS[0] == -1 {
    store_fighter_kinds();
    //}
    
    let fighter_kind; // fighter whose item we'll be accessing
    let variation_index; // where in the list (not a list!) of items the one we want to access is, according to our menu selection

    // Next we want to see which menu item is selected (or roll menu items?) to know which fighter to access
    match menu_state {
        CharacterItem::Player1 => {
            fighter_kind = FIGHTER_KINDS[0];
            variation_index = 0;
        }
        CharacterItem::Player2 => {
            fighter_kind = FIGHTER_KINDS[0];
            variation_index = 1;
        }
        CharacterItem::Cpu1 => {
            fighter_kind = FIGHTER_KINDS[1];
            variation_index = 0;
        }
        CharacterItem::Cpu2 => {
            fighter_kind = FIGHTER_KINDS[1];
            variation_index = 1;
        }
        _ => return, // may need to add ;?
    }
    
    // Then we want to use this fighter_id to grab an item from, if possible
    // We then want to check what items this fighter has access to

    println!("Made it to Vector Sort!");
    //let validItems: Vec<&CharItem> = CHARITEM_ALL.iter().filter(|itemStruct| itemStruct.fighter_kind == fighter_kind).collect();

    let bop = CHARITEM_ALL; // Doesn't this defeat the purpose of having CHARITEM_ALL not be declared in this function? Need to learn correct way
                            // to filter borrowed, non let array
    let bop2 = bop.iter().filter(|itemStruct| itemStruct.fighter_kind == fighter_kind);
    let validItems: Vec<&CharItem> = bop2.collect();

    println!("Length of Valid Items: {}", validItems.len());
    // Now we check to see if our menu selection exists for the fighter.
    println!("Made it to option select!");
    let current_CharItem = validItems.get(variation_index); // use of camel case is bad?
    println!("Made it to matching!");
    match current_CharItem {
        // The item exists
        Some(x) => { // Should change x probably // Also, is it okay that x is a &&CharItem?
            let current_item = &x.item_kind;
            let current_variation_option = &x.variation; // If this = none make it 0, otherwise make it equal?
            let current_variation;
            match current_variation_option {
                Some(lua_variation) => current_variation = **lua_variation, // changing var type?
                None => current_variation = 0,
            }
            println!("Made it to giving item!");
            ItemModule::have_item(module_accessor, smash::app::ItemKind(**current_item), current_variation, 0, false, false);
            println!("Gave item successfully?");
        }
        // The item does not exist
        None    => return,
    }
    
    
    /*
    
    if validItems.get(variation_index).is_none() {
        return; // item doesn't exist
    } else { // If it does exist, give the fighter the item
        let current_CharItem = validItems.get(variation_index);
        let current_item = current_CharItem.item_kind;
        let current_variation = current_CharItem.variation; // If this = none make it 0, otherwise make it equal?
        //ItemModule::have_item(module_accessor, smash::app::ItemKind(*ITEM_KIND_ROBOTGYRO), *ITEM_VARIATION_ROBOTGYRO_2P, 0, false, false);

        // How do I open up "Some"s? Should have them here.
        ItemModule::have_item(module_accessor, smash::app::ItemKind(*current_item), *current_variation, 0, false, false);
    }
    */
}
