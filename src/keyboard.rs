use std::{thread::sleep, time::Duration};

use serde::{Serialize, Deserialize};
use web_sys::HidDevice;
use wasm_bindgen::prelude::*;
use leptos::*;

use crate::app::{UiState, ADC_Data};


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

#[derive(strum::Display, strum::EnumIter, strum::EnumVariantNames, num_enum::TryFromPrimitive)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[repr(i16)]
pub enum KeyCode {
    /*------------------------- HID report data -------------------------*/
    LCTRL = -8,
    LSHIFT = -7,
    LALT = -6,
    LGUI = -5,
    RCTRL = -4,
    RSHIFT = -3,
    RALT = -2,
    RGUI = -1,

    RESERVED = 0,ERROR_ROLL_OVER,POST_FAIL,ERROR_UNDEFINED,
    A,B,C,D,E,F,G,H,I,J,K,L,M,
    N,O,P,Q,R,S,T,U,V,W,X,Y,Z,
    #[strum(serialize="1")]
    NUM_1/*1!*/,
    #[strum(serialize="2")]
    NUM_2/*2@*/,
    #[strum(serialize="3")]
    NUM_3/*3#*/,
    #[strum(serialize="4")]
    NUM_4/*4$*/,
    #[strum(serialize="5")]
    NUM_5/*5%*/,
    #[strum(serialize="6")]
    NUM_6/*6^*/,
    #[strum(serialize="7")]
    NUM_7/*7&*/,
    #[strum(serialize="8")]
    NUM_8/*8**/,
    #[strum(serialize="9")]
    NUM_9/*9(*/,
    #[strum(serialize="0")]
    NUM_0/*0)*/,
    ENTER,ESC,BACKSPACE,TAB,SPACE,
    #[strum(serialize="-_")]
    MINUS/*-_*/,
    #[strum(serialize="=+")]
    EQUAL/*=+*/,
    #[strum(serialize=r"[{")]
    LEFT_U_BRACE/*[{*/,
    #[strum(serialize=r"]}")]
    RIGHT_U_BRACE/*]}*/,
    #[strum(serialize=r"\|")]
    BACKSLASH/*\|*/,
    NONE_US/* */,
    #[strum(serialize=";:")]
    SEMI_COLON/*;:*/,
    #[strum(serialize="\' \"")]
    QUOTE/*'"*/,
    #[strum(serialize="`~")]
    GRAVE_ACCENT/*`~*/,
    #[strum(serialize=",<")]
    COMMA/*,<*/,
    #[strum(serialize=".>")]
    PERIOD/*.>*/,
    #[strum(serialize="/?")]
    SLASH/*/?*/,
    CAP_LOCK,F1,F2,F3,F4,F5,F6,F7,F8,F9,F10,F11,F12,
    PRINT,SCROLL_LOCK,PAUSE,INSERT,HOME,PAGE_UP,DELETE,END,PAGE_DOWN,
    #[strum(serialize="➡")]
    RIGHT_ARROW,
    #[strum(serialize="⬅")]
    LEFT_ARROW,
    #[strum(serialize="⬇")]
    DOWN_ARROW,
    #[strum(serialize="⬆")]
    UP_ARROW,
    PAD_NUM_LOCK,PAD_SLASH,PAD_ASTERISK,PAD_MINUS,PAD_PLUS,PAD_ENTER,
    PAD_NUM_1,PAD_NUM_2,PAD_NUM_3,PAD_NUM_4,PAD_NUM_5,
    PAD_NUM_6,PAD_NUM_7,PAD_NUM_8,PAD_NUM_9,PAD_NUM_0,
    PAD_PERIOD, NONUS_BACKSLASH,APPLICATION,POWER,PAD_EQUAL,
    F13,F14,F15,F16,F17,F18,F19,F20,F21,F22,F23,F24, EXECUTE,
    HELP,MENU,SELECT,STOP,AGAIN,UNDO,CUT,COPY,PASTE,FIND,MUTE,VOLUME_UP,VOLUME_DOWN,
    FN = 1000,
    PROFILE1,
    PROFILE2,
    PROFILE3,
    PROFILE4,
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

#[derive(strum::Display, strum::EnumIter, strum::EnumString, strum::EnumVariantNames)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum RGB_MODE {
    STATIC,
    CYCLE,
    LINEAR,
    TRIGGER,
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
    pub bind_key: (i16, i16),
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
		      bind_key: (KeyCode::RESERVED as i16, KeyCode::RESERVED as i16),
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

#[allow(non_snake_case)]
#[derive(serde::Serialize)]
struct Filter {
    vendorId: Option<u16>,
    productId: Option<u16>,
    #[serde(rename = "usagePage")]
    pub usage_page: Option<u16>,
    pub usage: Option<u16>,
}

pub fn register_hid_event(

) {
    
}

pub async fn init_hid_device(
    uistate: RwSignal<UiState>,
    keyboard_state: RwSignal<Keyboard>,
    set_adc_datas: WriteSignal<ADC_Data>,
    adc_vec: RwSignal<Vec<u32>>,
) { 
    let window = web_sys::window().unwrap();
    let nav = window.navigator();
    let devices_promise = nav.hid()
	.request_device(&web_sys::HidDeviceRequestOptions::new(&serde_wasm_bindgen::to_value(&[Filter{
	    vendorId:Some(0x0484),
	    productId:Some(0x572f),
	    // usage_page:Some(0xff00),
	    // usage:Some(0x00),
	    usage_page: None,
	    usage: None,
	}]).unwrap()));
    let devices = wasm_bindgen_futures::JsFuture::from(devices_promise).await.unwrap();
    let devs_array = devices.dyn_ref::<js_sys::Array>().expect("FAILED to cast the returned value from `request_device()`.");
    let device: JsValue = devs_array.at(0); //interface 0
    let device: HidDevice  = device.dyn_into().expect("FAILED to cast `JsValue` in array into `HidDevice`.");
    //open device
    wasm_bindgen_futures::JsFuture::from(device.open()).await.expect("Cannot Open Device");

    let closure = Closure::<dyn FnMut(_)>::new(move |e: web_sys::HidConnectionEvent| {
	let event_dev = e.device();
	if let Some(dev) = uistate.get().hid_device {
	    if event_dev.product_name() == dev.product_name(){
		logging::log!("Disconnected");
		uistate.update(|v| v.hid_device=None)
	    }
	}
    });
    nav.hid().set_ondisconnect(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
  
    let closure = Closure::<dyn FnMut(_)>::new(move |e: web_sys::HidInputReportEvent| {
	let dataview = e.data();
	let rid = e.report_id();
	let ofs = dataview.byte_offset();
	let len = dataview.byte_length();
	let ba: Vec<u8> = (0..len).map(|i| { dataview.get_uint8(i+ofs) }).collect();
	if rid == 2 {
	    if ba[0] < 16 {
		adc_input(ba, set_adc_datas, uistate, adc_vec);
	    } else if ba[0] < 32 {
		performance_input(ba, keyboard_state);
	    } else if ba[0] < 48 {
		rgb_input(ba, keyboard_state);
	    } else if ba[0] < 64 {
		keymap_input(ba, keyboard_state);
	    }
	}
    });
    device.set_oninputreport(Some(closure.as_ref().unchecked_ref()));
    closure.forget();

    
    uistate.update(|state| state.hid_device=Some(device));


}

fn adc_input(
    ba: Vec<u8>,
    set_adc_datas: WriteSignal<ADC_Data>,
    uistate: RwSignal<UiState>,
    adc_vec: RwSignal<Vec<u32>>,
) {
    //			logging::log!("{:?}", ba);
    let adc_data_page = (ba[0]%16) as usize;
    let mut data: u32 = 0;
    for (idx, x) in ba[1..17].iter().enumerate() {
	if idx%2==1 {
	    data += x.clone() as u32;
	    set_adc_datas.update(|v| v.array[STM2RS[idx/2 + adc_data_page*8]] = data);
	    if uistate.get().key_monitor == STM2RS[idx/2 + adc_data_page*8] as u32 {
		adc_vec.update(|v| {
		    v.push(data);
		    if v.len() > 128 {
			v.remove(0);
		    }
		});
	    }
	    
	} else {
	    data = x.clone() as u32 *256;
	}
    }
}

fn performance_input(
    ba: Vec<u8>,
    keyboard_state: RwSignal<Keyboard>,
) {
//    logging::log!("{:?}", ba);
    let performance_data_page = (ba[0]%16) as usize;
    keyboard_state.update(|Keyboard{keys, ..}| {
	for (idx, x) in ba[1..17].chunks(4).enumerate() {
	    let key_index = performance_data_page*4 + idx;
	    keys[STM2RS[key_index]].mode = (x[0] >> 7) as u32;
	    keys[STM2RS[key_index]].value.0 = (x[0]&0x7f) as u32;
	    keys[STM2RS[key_index]].value.1 = x[1] as u32;
	    keys[STM2RS[key_index]].value.2 = x[2] as u32;
	    keys[STM2RS[key_index]].value.3 = x[3] as u32;
	}
    });
}

pub async fn request_input(
    device: &HidDevice,
) {
    let mut send_buf = [0u8; 64];
    send_buf[0] = 2;
    send_buf[1] = 255;
    send_report(device, &mut send_buf).await;
}

pub async fn send_performance_report(
    send_buf: &mut [u8],
    keyboard_state: &Keyboard,
    device: &HidDevice,
) {
    send_buf[0] = 2;
    for (page_num, keys) in keyboard_state.keys.chunks(4).enumerate() {
	send_buf[1] = page_num as u8;
	for i in 0..4 {
 	    send_buf[2 + i*4+0] = keys[i].value.0 as u8 | ((keys[i].mode << 7) as u8);
	    send_buf[2 + i*4+1] = keys[i].value.1 as u8;
	    send_buf[2 + i*4+2] = keys[i].value.2 as u8;
	    send_buf[2 + i*4+3] = keys[i].value.3 as u8;
	}
	logging::log!("test: [page: {} payload: {:?}]", page_num, &send_buf[0..18]);
	send_report(device, send_buf).await;
    }
}

fn rgb_input(
    ba: Vec<u8>,
    keyboard_state: RwSignal<Keyboard>,
) {
    let rgb_data_page = (ba[0]%16) as usize;
    keyboard_state.update(|Keyboard{keys, ..}| {
	for (idx, x) in ba[1..17].chunks(4).enumerate() {
	    let key_index = rgb_data_page*4 + idx;
	    keys[STM2RS[key_index]].rgb_raw = (x[0] as u32) << 24 | (x[1] as u32) << 16 | (x[2] as u32) << 8 | x[3] as u32;
	}
    });
}

pub async fn send_rgb_report(
    send_buf: &mut [u8],
    keyboard_state: &Keyboard,
    device: &HidDevice,
) {
    send_buf[0] = 2;
    for (page_num, keys) in keyboard_state.keys.chunks(4).enumerate() {
	send_buf[1] = page_num as u8 + 16;
	for i in 0..4 {
	    let rgb_raw = keys[i].rgb_raw;
 	    send_buf[2 + i*4+0] = (rgb_raw>>24) as u8;
	    send_buf[2 + i*4+1] = (rgb_raw>>16) as u8;
	    send_buf[2 + i*4+2] = (rgb_raw>>8) as u8;
	    send_buf[2 + i*4+3] = (rgb_raw>>0) as u8;
	}
	logging::log!("test: [page: {} payload: {:?}]", page_num, &send_buf[0..18]);
	send_report(device, send_buf).await;
    }
}

fn keymap_input(
    ba: Vec<u8>,
    keyboard_state: RwSignal<Keyboard>,
) {
    let keymap_page = (ba[0]%16) as usize;
    keyboard_state.update(|Keyboard{keys, ..}| {
	for (idx, x) in ba[1..17].chunks(4).enumerate() {
	    let key_index = keymap_page*4 + idx;
	    keys[STM2RS[key_index]].bind_key.0 = ((x[0] as u16) << 8 | x[1] as u16) as i16;
	    keys[STM2RS[key_index]].bind_key.1 = ((x[2] as u16) << 8 | x[3] as u16) as i16;
	}
    });
}

pub async fn send_keymap_report(
    send_buf: &mut [u8],
    keyboard_state: &Keyboard,
    device: &HidDevice,
) {
    send_buf[0] = 2;
    for (page_num, keys) in keyboard_state.keys.chunks(4).enumerate() {
	send_buf[1] = page_num as u8 + 32;
	for i in 0..4 {
	    let bindkey = keys[i].bind_key;
 	    send_buf[2 + i*4+0] = (bindkey.0>>8) as u8;
	    send_buf[2 + i*4+1] = (bindkey.0>>0) as u8;
	    send_buf[2 + i*4+2] = (bindkey.1>>8) as u8;
	    send_buf[2 + i*4+3] = (bindkey.1>>0) as u8;
	}
	logging::log!("test: [page: {} payload: {:?}]", page_num, &send_buf[0..18]);
	send_report(device, send_buf).await;
    }
}

async fn send_report(
    device: &HidDevice,
    send_buf: &mut [u8]
) {
    if device.opened() == false {
	wasm_bindgen_futures::JsFuture::from(device.open()).await.expect("Cannot Open Device");
    }
    if device.opened() {
	let res = wasm_bindgen_futures::JsFuture::from(device.send_report_with_u8_array(2, &mut send_buf[1..18])).await;
	match res {
	    Err(err) => logging::log!("{:?}", err),
	    _ => {},
	}
//	std::thread::sleep(std::time::Duration::from_millis(1));
    } else {
	logging::log!("Device is not opend");
    }
} 
