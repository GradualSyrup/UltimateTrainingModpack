use skyline::nn::hid::NpadGcState;
use smash::app::{BattleObjectModuleAccessor, lua_bind::*};
use smash::lib::lua_const::*;
use lazy_static::lazy_static;
use parking_lot::Mutex;

// Need to define necesary structures here. Probably should move to consts or something. Realistically, should be in skyline smash prob tho.

// Final final controls used for controlmodule
#[repr(C)]
pub struct ControlModuleInternal {
    vtable: *mut u8,
    controller_index: i32,
    buttons: Buttons,
    stick_x: f32,
    stick_y: f32,
    padding: [f32; 2],
    unk: [u32; 8],
    clamped_lstick_x: f32,
    clamped_lstick_y: f32,
    padding2: [f32; 2],
    clamped_rstick_x: f32,
    clamped_rstick_y: f32,
}

// Re-ordered bitfield the game uses for buttons - TODO: Is this a problem? What's the original order?
type ButtonBitfield = i32; // may need to actually implement? Not for now though

/// Controller style declaring what kind of controller is being used
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
#[repr(u32)]
pub enum ControllerStyle {
    Handheld = 0x1,
    DualJoycon = 0x2,
    LeftJoycon = 0x3,
    RightJoycon = 0x4,
    ProController = 0x5,
    DebugPad = 0x6, // probably
    GCController = 0x7
}

#[repr(C)]
pub struct AutorepeatInfo {
    field: [u8; 0x18]
}

// Can map any of these over any button - what does this mean?
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum InputKind {
    Attack = 0x0,
    Special = 0x1,
    Jump = 0x2,
    Guard = 0x3,
    Grab = 0x4,
    SmashAttack = 0x5,
    AppealHi = 0xA,
    AppealS = 0xB,
    AppealLw = 0xC,
    Unset = 0xD,
}

// 0x50 Byte struct containing the information for controller mappings
#[derive(Debug)]
#[repr(C)]
pub struct ControllerMapping {
    pub gc_l: InputKind,
    pub gc_r: InputKind,
    pub gc_z: InputKind,
    pub gc_dup: InputKind,
    pub gc_dlr: InputKind,
    pub gc_ddown: InputKind,
    pub gc_a: InputKind,
    pub gc_b: InputKind,
    pub gc_cstick: InputKind,
    pub gc_y: InputKind,
    pub gc_x: InputKind,
    pub gc_rumble: bool,
    pub gc_absmash: bool,
    pub gc_tapjump: bool,
    pub gc_sensitivity: u8,
    // 0xF
    pub pro_l: InputKind,
    pub pro_r: InputKind,
    pub pro_zl: InputKind,
    pub pro_zr: InputKind,
    pub pro_dup: InputKind,
    pub pro_dlr: InputKind,
    pub pro_ddown: InputKind,
    pub pro_a: InputKind,
    pub pro_b: InputKind,
    pub pro_cstick: InputKind,
    pub pro_x: InputKind,
    pub pro_y: InputKind,
    pub pro_rumble: bool,
    pub pro_absmash: bool,
    pub pro_tapjump: bool,
    pub pro_sensitivity: u8,
    // 0x1F
    pub joy_shoulder: InputKind,
    pub joy_zshoulder: InputKind,
    pub joy_sl: InputKind,
    pub joy_sr: InputKind,
    pub joy_up: InputKind,
    pub joy_right: InputKind,
    pub joy_left: InputKind,
    pub joy_down: InputKind,
    pub joy_rumble: bool,
    pub joy_absmash: bool,
    pub joy_tapjump: bool,
    pub joy_sensitivity: u8,
    // 0x2B
    pub _2b: u8,
    pub _2c: u8,
    pub _2d: u8,
    pub _2e: u8,
    pub _2f: u8,
    pub _30: u8,
    pub _31: u8,
    pub _32: u8,
    pub is_absmash: bool,
    pub _34: [u8; 0x1C]
}


type Buttons = u32; // may need to actually implement (like label and such)? Not for now though

// Controller class used internally by the game
#[repr(C)]
pub struct Controller {
    pub vtable: *const u64,
    pub current_buttons: ButtonBitfield,
    pub previous_buttons: ButtonBitfield,
    pub left_stick_x: f32,
    pub left_stick_y: f32,
    pub left_trigger: f32,
    pub _left_padding: u32,
    pub right_stick_x: f32,
    pub right_stick_y: f32,
    pub right_trigger: f32,
    pub _right_padding: u32,
    pub gyro: [f32; 4],
    pub button_timespan: AutorepeatInfo,
    pub lstick_timespan: AutorepeatInfo,
    pub rstick_timespan: AutorepeatInfo,
    pub just_down: ButtonBitfield,
    pub just_release: ButtonBitfield,
    pub autorepeat_keys: u32,
    pub autorepeat_threshold: u32,
    pub autorepeat_initial_press_threshold: u32,
    pub style: ControllerStyle,
    pub controller_id: u32,
    pub primary_controller_color1: u32,
    pub primary_controller_color2: u32,
    pub secondary_controller_color1: u32,
    pub secondary_controller_color2: u32,
    pub led_pattern: u8,
    pub button_autorepeat_initial_press: bool,
    pub lstick_autorepeat_initial_press: bool,
    pub rstick_autorepeat_initial_press: bool,
    pub is_valid_controller: bool,
    pub _xB9: [u8; 2],
    pub is_connected: bool,
    pub is_left_connected: bool,
    pub is_right_connected: bool,
    pub is_wired: bool,
    pub is_left_wired: bool,
    pub is_right_wired: bool,
    pub _xC1: [u8; 3],
    pub npad_number: u32,
    pub _xC8: [u8; 8]
}

// SomeControllerStruct used in hooked function - need to ask blujay what this is again
#[repr(C)]
struct SomeControllerStruct {
    padding: [u8; 0x10],
    controller: &'static mut Controller
}

// Define struct used for final controller inputs
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MappedInputs {
    pub buttons: Buttons,
    pub lstick_x: i8,
    pub lstick_y: i8,
    pub rstick_x: i8,
    pub rstick_y: i8
}

impl MappedInputs {
    pub fn default() -> MappedInputs {
        MappedInputs {
            buttons: (0 as u32) as Buttons,
            lstick_x: 0,
            lstick_y: 0,
            rstick_x: 0,
            rstick_y: 0
        }
    }
}

lazy_static! {
    static ref P1_FINAL_MAPPING: Mutex<[MappedInputs; 90]> =
        Mutex::new([{
            MappedInputs::default()
        }; 90]);
}

pub static mut INPUT_RECORD: InputRecordState = InputRecordState::None;
pub static mut INPUT_RECORD_FRAME: usize = 0;
pub static mut CPU_CONTROL_ADDR: *mut ControlModuleInternal = 0 as *mut ControlModuleInternal;

#[derive(PartialEq)]
pub enum InputRecordState {
    None,
    Record,
    Playback,
}

use InputRecordState::*;

pub unsafe fn get_command_flag_cat(module_accessor: &mut BattleObjectModuleAccessor) {
    let entry_id_int =
            WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as i32;

    if entry_id_int == 0 {
        // Attack + Dpad Right: Playback
        if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
            && ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
            crate::common::raygun_printer::print_string(&mut *module_accessor, "PLAYBACK");
            playback();
            println!("Playback Command Received!"); //debug
        }
        // Attack + Dpad Left: Record
        else if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
            && ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L)
        {
           crate::common::raygun_printer::print_string(&mut *module_accessor, "RECORDING");
           record();
           println!("Record Command Received!"); //debug
        }


        // may need to move this to another func
        if INPUT_RECORD == Record || INPUT_RECORD == Playback {
            if INPUT_RECORD_FRAME >= P1_FINAL_MAPPING.lock().len() - 1 {
                if INPUT_RECORD == Record {
                    //INPUT_RECORD = Playback; // shouldn't do this, causes it to play twice. TODO: replace with line below once other things tested
                    INPUT_RECORD = None;
                } else if INPUT_RECORD == Playback {
                    INPUT_RECORD = None;
                }
                INPUT_RECORD_FRAME = 0;
            } else {
                INPUT_RECORD_FRAME += 1;
            }
        }
    }
}

pub unsafe fn record() {
    INPUT_RECORD = Record;
    // Reset mappings to nothing, and then start recording. Maybe this resetting is unnecessary? Unsure
    P1_FINAL_MAPPING.lock().iter_mut().for_each(|mapped_input| {
        *mapped_input = MappedInputs::default();
    });
    INPUT_RECORD_FRAME = 0;
}

pub unsafe fn playback() {
    INPUT_RECORD = Playback;
    INPUT_RECORD_FRAME = 0;
}

pub fn handle_get_npad_state( // Shouldn't need this hook anymore, keeping for reference for now
    state: *mut NpadGcState,
    controller_id: *const u32,
) {
    /*unsafe {
        if *controller_id == p1_controller_id() {
            if INPUT_RECORD == Record {
                P1_NPAD_STATES.lock()[INPUT_RECORD_FRAME] = *state;
            }
        } else if INPUT_RECORD == Record || INPUT_RECORD == Playback {
            let update_count = (*state).updateCount;
            *state = P1_NPAD_STATES.lock()[INPUT_RECORD_FRAME];
            (*state).updateCount = update_count;
        }
    }*/
}

// TODO: Explain
static FIM_OFFSET: usize = 0x17504a0; 
// TODO: Should we define all of our offsets in one file, like HDR? Should at least be a good start for changing to be based on ASM instructions
#[skyline::hook(offset = FIM_OFFSET)]
unsafe fn handle_final_input_mapping(
    mappings: *mut ControllerMapping,
    player_idx: i32, // Is this the player index, or plugged in controller index? Need to check, assuming player for now - is this 0 indexed or 1?
    out: *mut MappedInputs,
    controller_struct: &mut SomeControllerStruct,
    arg: bool
) {
    // go through the original mapping function first
    let ret = original!()(mappings, player_idx, out, controller_struct, arg);
    //println!("Player: {}, Out Addr: {:p}", player_idx, out);
    if player_idx == 0 { // if player 1 (what is going on here? switching from handheld to docked seems to make this change to 1 and 2 instead of 0)
        if INPUT_RECORD == Record {
            P1_FINAL_MAPPING.lock()[INPUT_RECORD_FRAME] = *out;
            //*out = MappedInputs::default() // don't control player while recording TODO: Change this for later, want off during dev and testing
        }
    } /*else if INPUT_RECORD == Record || INPUT_RECORD == Playback { // Shouldn't be needed, we take care of cpu elsewhere
        *out = P1_FINAL_MAPPING.lock()[INPUT_RECORD_FRAME];
        // updateCount gone now - what was this? Was this important?
    }*/

    // debug:
    let input_type;
    if INPUT_RECORD == Record {
        input_type = "Record";
    } else if INPUT_RECORD == Playback {
        input_type = "Playback";
    } else {
        input_type = "Other";
    }
    println!("{} PLAYER, Frame: {}", input_type, INPUT_RECORD_FRAME); //debug
}

#[skyline::hook(offset = 0x2da180)] // After cpu controls are assigned from ai calls
unsafe fn set_cpu_controls(p_data: *mut *mut u8) {
  call_original!(p_data);
  let controller_data = *p_data.add(1) as *mut ControlModuleInternal;
  let controller_no  = (*controller_data).controller_index;
  let input_type;
  if INPUT_RECORD == Record {
    input_type = "Record";
  } else if INPUT_RECORD == Playback {
    input_type = "Playback";
  } else {
    input_type = "Other";
  }
  if INPUT_RECORD == Record || INPUT_RECORD == Playback {
    //println!("Overriding Cpu Player: {}", controller_no); // cpu is normally 1, at least on handheld
    if INPUT_RECORD_FRAME > 0 {
        let saved_mapped_inputs = P1_FINAL_MAPPING.lock()[INPUT_RECORD_FRAME-1];
        (*controller_data).buttons = saved_mapped_inputs.buttons;
        (*controller_data).stick_x = (saved_mapped_inputs.lstick_x as f32) / (i8::MAX as f32);
        (*controller_data).stick_y = (saved_mapped_inputs.lstick_y as f32) / (i8::MAX as f32);
        println!("{} CPU, Frame: {}", input_type, INPUT_RECORD_FRAME); //debug
        /*println!("Saved stick x: {}, new stick x: {}", saved_mapped_inputs.lstick_x, *(controller_data.add(0x40) as *mut f32));
        println!("Saved stick y: {}, new stick y: {}", saved_mapped_inputs.lstick_y, *(controller_data.add(0x44) as *mut f32));
        */
        // Clamp stick inputs for separate part of structure
        const NEUTRAL: f32 = 0.2;
        const CLAMP_MAX: f32 = 120.0;
        let clamp_mul = 1.0 / CLAMP_MAX;
        let mut clamped_lstick_x = ((saved_mapped_inputs.lstick_x as f32) * clamp_mul).clamp(-1.0, 1.0);
        let mut clamped_lstick_y = ((saved_mapped_inputs.lstick_y as f32) * clamp_mul).clamp(-1.0, 1.0);
        clamped_lstick_x = if clamped_lstick_x.abs() >= NEUTRAL { clamped_lstick_x } else { 0.0 };
        clamped_lstick_y = if clamped_lstick_y.abs() >= NEUTRAL { clamped_lstick_y } else { 0.0 };
        (*controller_data).clamped_lstick_x = clamped_lstick_x;
        (*controller_data).clamped_lstick_y = clamped_lstick_y;
    }
    
    CPU_CONTROL_ADDR = controller_data;
    println!("Saving CPU Addr as {:p}", controller_data);
  } 
}

pub fn init() {
    skyline::install_hooks!(
        handle_final_input_mapping,
        set_cpu_controls
    );
}