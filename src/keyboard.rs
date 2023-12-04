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

pub const STM2RS: [usize; 64] = [31,
30,
29,
28,
41,//Left Shift
42,
43,
44,
14,
15,
16,
17,
3,
2,
1,
0,
40,
39,
38,
37,
51,
52,//Right Shift
53,
54,
24,
25,
26,
27,
13,
12,
11,
10,
58,
47,
34,
20,
19,
6,
33,
46,
45,
32,
18,
5,
4,
55,//Left Control
56,//Left GUI
57,//Left Alt
59,//Right alt
60, //Fn
61,
62,
63,
50,
49,
36,
22,
23,
9,
8,
48,
35,
21,
7,];

#[derive(Serialize, Deserialize)]
pub struct MessageArgs<'a> {
    /// variable name should be the same as backend
    pub payload: &'a str,
}
    


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Key {
    pub index: u32,
    pub stm32_index: u32,
    pub bind_key: u32,
    /// 0: default; 1: RT
    pub mode: u32,
    /// 0: normal trigger travel 1: dynamic trigger travel 2: dynamic reset travel 3: lower deadzone
    pub value: (u32, u32, u32, u32),
    pub position: (u32, u32),
    pub size: (u32, u32),
    pub selected: bool,
    pub adc_value: u32,
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
		      bind_key: idx,
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
