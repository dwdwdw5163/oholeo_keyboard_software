use serde::{Serialize, Deserialize};


pub const WIDTH: u32 = 64;
pub const HEIGHT: u32 = 64;


pub const KEYBOARD_CHARS: [&str;64] = [
    "Esc", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "-", "=", "Backspace", 
    "Tab", "Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P", "[", "]", "\\",
    "Caps", "A", "S", "D", "F", "G", "H", "J", "K", "L", ";", ",", "Enter", 
    "LShift", "Z", "X", "C", "V", "B", "N", "M","<", ">", "/", "RS", "⬆", "Del", 
    "Ctrl", "Win", "Alt", "Space", "RAlt", "Fn", "⬅", "⬇", "➡",
];

/// index is idx of the keybuttonView, element is the index of stm32 array
pub const KEYMAP: [usize; 64] = [15, 14, 13, 12, 44, 43, 37, 63, 59, 58, 31, 30, 29, 28, 8, 9, 10, 11, 42, 36, 35, 62, 56, 57, 24, 25, 26, 27, 3, 2, 1, 0, 41, 38, 34, 61, 55, 19, 18, 17, 16, 4, 5, 6, 7, 40, 39, 33, 60, 54, 53, 20, 21, 22, 23, 45, 46, 47, 32, 48, 49, 50, 51, 52];

pub const STM2RS: [usize; 64] = [31,30,29,28,41,42,43,44,14,15,16,17,3,2,1,0,40,39,38,37,51,52,53,54,24,25,26,27,13,12,11,10,58,47,34,20,19,6,33,46,45,32,18,5,4,55,56,57,59,60,61,62,63,50,49,36,22,23,9,8,48,35,21,7,];

#[derive(strum::Display, strum::EnumIter, strum::EnumVariantNames)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum KeyCode {
    /*------------------------- HID report data -------------------------*/
    LEFT_CTRL = -8,LEFT_SHIFT = -7,LEFT_ALT = -6,LEFT_GUI = -5,
    RIGHT_CTRL = -4,RIGHT_SHIFT = -3,RIGHT_ALT = -2,RIGHT_GUI = -1,

    RESERVED = 0,ERROR_ROLL_OVER,POST_FAIL,ERROR_UNDEFINED,
    A,B,C,D,E,F,G,H,I,J,K,L,M,
    N,O,P,Q,R,S,T,U,V,W,X,Y,Z,
    NUM_1/*1!*/,NUM_2/*2@*/,NUM_3/*3#*/,NUM_4/*4$*/,NUM_5/*5%*/,
    NUM_6/*6^*/,NUM_7/*7&*/,NUM_8/*8**/,NUM_9/*9(*/,NUM_0/*0)*/,
    ENTER,ESC,BACKSPACE,TAB,SPACE,
    MINUS/*-_*/,EQUAL/*=+*/,LEFT_U_BRACE/*[{*/,RIGHT_U_BRACE/*]}*/,
    BACKSLASH/*\|*/,NONE_US/* */,SEMI_COLON/*;:*/,QUOTE/*'"*/,
    GRAVE_ACCENT/*`~*/,COMMA/*,<*/,PERIOD/*.>*/,SLASH/*/?*/,
    CAP_LOCK,F1,F2,F3,F4,F5,F6,F7,F8,F9,F10,F11,F12,
    PRINT,SCROLL_LOCK,PAUSE,INSERT,HOME,PAGE_UP,DELETE,END,PAGE_DOWN,
    RIGHT_ARROW,LEFT_ARROW,DOWN_ARROW,UP_ARROW,PAD_NUM_LOCK,
    PAD_SLASH,PAD_ASTERISK,PAD_MINUS,PAD_PLUS,PAD_ENTER,
    PAD_NUM_1,PAD_NUM_2,PAD_NUM_3,PAD_NUM_4,PAD_NUM_5,
    PAD_NUM_6,PAD_NUM_7,PAD_NUM_8,PAD_NUM_9,PAD_NUM_0,
    PAD_PERIOD , NONUS_BACKSLASH,APPLICATION,POWER,PAD_EQUAL,
    F13,F14,F15,F16,F17,F18,F19,F20,F21,F22,F23,F24, EXECUTE,
    HELP,MENU,SELECT,STOP,AGAIN,UNDO,CUT,COPY,PASTE,FIND,MUTE,VOLUME_UP,VOLUME_DOWN,
    FN = 1000
    /*------------------------- HID report data -------------------------*/
}

#[derive(strum::Display, strum::EnumIter, strum::EnumString)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum RGB_GLOBAL_MODE {
    RGB_GLOBAL_MODE_OFF,
    RGB_GLOBAL_MODE_INDIVIDUAL,
    RGB_GLOBAL_MODE_WAVE,
    RGB_GLOBAL_MODE_STRING,
    RGB_GLOBAL_MODE_FADING_STRING,
    RGB_GLOBAL_MODE_DIAMOND_RIPPLE,
    RGB_GLOBAL_MODE_FADING_DIAMOND_RIPPLE,
    RGB_GLOBAL_MODE_JELLY,
}

#[derive(strum::Display, strum::EnumIter, strum::EnumString)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum RGB_MODE {
    RGB_MODE_STATIC,
    RGB_MODE_CYCLE,
    RGB_MODE_REACT_LINEAR,
    RGB_MODE_REACT_TRIGGER,
}



#[derive(Serialize, Deserialize)]
pub struct MessageArgs<'a> {
    /// variable name should be the same as backend
    pub payload: &'a str,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Key {
    pub index: u32,
    pub stm32_index: u32,
    pub bind_key: usize,
    /// 0: default; 1: RT
    pub mode: u32,
    /// 0: normal trigger travel 1: dynamic trigger travel 2: dynamic reset travel 3: lower deadzone
    pub value: (u32, u32, u32, u32),
    pub position: (u32, u32),
    pub size: (u32, u32),
    pub selected: bool,
    pub adc_value: u32,
    /// [31:24] global mode | mode;[23:16] R;[15:8] G; [7:0] B;
    pub rgb_raw: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keyboard {
    pub keys: Vec<Key>,
}

impl Keyboard {
    pub fn new() -> Keyboard {
	let mut keys: Vec<Key> = Vec::with_capacity(64);
	let mut x_position: u32 = 0;
	for idx in 0..=63 {
	    let scaler = match idx {
		13|41 => 2.0,
		14|27 => 1.5,
		28 => 1.75,
		40 => 2.25,
		55|56|57 => 1.25,
		58 => 6.25,
		_ => 1.0,
	    };

	    keys.push(
		Key { index: idx,
		      stm32_index: KEYMAP[idx as usize] as u32,
		      bind_key: 8,
		      mode: 0,
		      value: (50, 5, 5, 35),
		      position: (x_position, match idx {
			  0..=13 => 0,
			  14..=27 => HEIGHT,
			  28..=40 => 2*HEIGHT,
			  41..=54 => 3*HEIGHT,
			  55..=64 => 4*HEIGHT,
			  _ => 0,}),
		      size: ((WIDTH as f32*scaler) as u32, HEIGHT),
		      selected: false,
		      adc_value: 0,
		      rgb_raw: 0,
		});
	    x_position += WIDTH + ((scaler-1.0)*WIDTH as f32) as u32;
	    match idx {
		13|27|40|54 => x_position = 0,
		_ => (),   
	    }
	}
	Keyboard { keys }
    }

}
