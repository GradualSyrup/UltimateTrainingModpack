#![allow(unused_variables)]
#![allow(dead_code)] // TODO: Yeah don't do this
use bitflags::bitflags;

// Need to define necesary structures here. Probably should move to consts or something. Realistically, should be in skyline smash prob tho.

// Final final controls used for controlmodule
// can I actually derive these?
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct ControlModuleInternal {
    pub vtable: *mut u8,
    pub controller_index: i32,
    pub buttons: Buttons,
    pub stick_x: f32,
    pub stick_y: f32,
    pub padding: [f32; 2],
    pub unk: [u32; 8],
    pub clamped_lstick_x: f32,
    pub clamped_lstick_y: f32,
    pub padding2: [f32; 2],
    pub clamped_rstick_x: f32,
    pub clamped_rstick_y: f32,
}

impl ControlModuleInternal {
    pub fn clear(&mut self) { // Try to nullify controls so we can't control player 1 during recording
        self.stick_x = 0.0;
        self.stick_y = 0.0;
        self.buttons = Buttons::NONE;
        self.clamped_lstick_x = 0.0;
        self.clamped_lstick_y = 0.0;
        self.clamped_rstick_x = 0.0;
        self.clamped_rstick_y = 0.0;
    }
    pub fn construct_stored(& self) -> ControlModuleStored { // Try to nullify controls so we can't control player 1 during recording
        ControlModuleStored {    
            buttons: self.buttons,
            stick_x: self.stick_x,
            stick_y: self.stick_y,
            padding: self.padding,
            unk: self.unk,
            clamped_lstick_x: self.clamped_lstick_x,
            clamped_lstick_y: self.clamped_lstick_y,
            padding2: self.padding2,
            clamped_rstick_x: self.clamped_rstick_x,
            clamped_rstick_y: self.clamped_rstick_y
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct ControlModuleStored { // Custom type for saving only necessary controls/not saving vtable
    pub buttons: Buttons,
    pub stick_x: f32,
    pub stick_y: f32,
    pub padding: [f32; 2],
    pub unk: [u32; 8],
    pub clamped_lstick_x: f32,
    pub clamped_lstick_y: f32,
    pub padding2: [f32; 2],
    pub clamped_rstick_x: f32,
    pub clamped_rstick_y: f32,
}

impl ControlModuleStored {
    pub fn construct_internal(& self, curr_vtable: *mut u8, curr_index: i32) -> ControlModuleInternal { // Try to nullify controls so we can't control player 1 during recording
        ControlModuleInternal {    
            vtable: curr_vtable,
            controller_index: curr_index,
            buttons: self.buttons,
            stick_x: self.stick_x,
            stick_y: self.stick_y,
            padding: self.padding,
            unk: self.unk,
            clamped_lstick_x: self.clamped_lstick_x,
            clamped_lstick_y: self.clamped_lstick_y,
            padding2: self.padding2,
            clamped_rstick_x: self.clamped_rstick_x,
            clamped_rstick_y: self.clamped_rstick_y
        }
    }
    pub fn default() -> ControlModuleStored {
        ControlModuleStored {
            buttons: Buttons::NONE,
            stick_x: 0.0,
            stick_y: 0.0,
            padding: [0.0; 2],
            unk: [0; 8],
            clamped_lstick_x: 0.0,
            clamped_lstick_y: 0.0,
            padding2: [0.0; 2],
            clamped_rstick_x: 0.0,
            clamped_rstick_y: 0.0
        }
    }
}

// Re-ordered bitfield the game uses for buttons - TODO: Is this a problem? What's the original order?
pub type ButtonBitfield = i32; // may need to actually implement? Not for now though

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


//type Buttons = u32; // may need to actually implement (like label and such)? Not for now though
bitflags! {
    pub struct Buttons: u32 {
        const NONE        = 0x0; // does adding this cause problems?
        const ATTACK      = 0x1;
        const SPECIAL     = 0x2;
        const JUMP        = 0x4;
        const GUARD       = 0x8;
        const CATCH       = 0x10;
        const SMASH       = 0x20;
        const JUMP_MINI    = 0x40;
        const CSTICK_ON    = 0x80;
        const STOCK_SHARE  = 0x100;
        const ATTACK_RAW   = 0x200;
        const APPEAL_HI    = 0x400;
        const SPECIAL_RAW  = 0x800;
        const APPEAL_LW    = 0x1000;
        const APPEAL_SL    = 0x2000;
        const APPEAL_SR    = 0x4000;
        const FLICK_JUMP   = 0x8000;
        const GUARD_HOLD   = 0x10000;
        const SPECIAL_RAW2 = 0x20000;
    }
}

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
    pub _x_b9: [u8; 2],
    pub is_connected: bool,
    pub is_left_connected: bool,
    pub is_right_connected: bool,
    pub is_wired: bool,
    pub is_left_wired: bool,
    pub is_right_wired: bool,
    pub _x_c1: [u8; 3],
    pub npad_number: u32,
    pub _x_c8: [u8; 8]
}

// SomeControllerStruct used in hooked function - need to ask blujay what this is again
#[repr(C)]
pub struct SomeControllerStruct {
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

impl MappedInputs { // pub needed?
    pub fn default() -> MappedInputs {
        MappedInputs {
            buttons: Buttons::NONE,
            lstick_x: 0,
            lstick_y: 0,
            rstick_x: 0,
            rstick_y: 0
        }
    }
}
