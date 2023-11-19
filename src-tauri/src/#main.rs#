// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use std::fs::File;
use std::io::{Write, BufWriter};
use std::sync::Arc;
use std::thread::{self, sleep};
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tracing::info;
use tracing_subscriber;
use std::time::Duration;
use tauri::{Manager, Window};
use hidapi::{HidApi, HidDevice};

struct Hid_Handle {
    api: HidApi,
    device: Option<HidDevice>,
    recv_buf: [u8;64],
}

struct AsyncProcInputTx {
    inner: Mutex<mpsc::Sender<String>>,
}


#[derive(Debug, Clone, serde::Deserialize)]
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
}


struct State {
    keys: Vec<Key>,
    key_monitor: u32,
    request_send: bool,
}


#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn upload_settings(
    payload: &str,
    state: tauri::State<'_, Arc<Mutex<State>>>,
)-> Result<(), String> {
    // let file = File::create("./results/keyboard.json").expect("Failed to create ");
    // let mut f = BufWriter::new(file);
    // f.write_all(payload.as_bytes()).expect("unable to write
    let mut state_lock = state.lock().await;
    state_lock.keys = serde_json::from_str(payload).unwrap();
    println!("{:?}", state_lock.keys[0]);
    println!("size: {}", state_lock.keys.len());
    state_lock.request_send = true;
    drop(state_lock);
    Ok(())
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn set_adc_num(
    payload: &str,
    state: tauri::State<'_, Arc<Mutex<State>>>,
)-> Result<(), String> {
    // let file = File::create("./results/keyboard.json").expect("Failed to create ");
    // let mut f = BufWriter::new(file);
    // f.write_all(payload.as_bytes()).expect("unable to write
    let mut state_lock = state.lock().await;
    state_lock.key_monitor = payload.parse::<u32>().unwrap();
    println!("adc moniter number: {:?}", state_lock.key_monitor);
    drop(state_lock);
    Ok(())
}



#[tauri::command]
fn rs2js<R: tauri::Runtime>(message: String, manager: &impl Manager<R>) {
    // println!("{:?}", message);
    manager
        .emit_all("rs2js", message)
        .unwrap();
}


async fn hid_thread(
    mut input_rx: mpsc::Receiver<String>,
    output_tx: mpsc::Sender<String>,
    state: Arc<Mutex<State>>
) {
    let api = HidApi::new().expect("Cannot create hidapi");
    let device: Option<HidDevice> = None;
    let mut handle = Hid_Handle{api, device, recv_buf: [0u8;64]};
    let mut send_interval = 0;
    let mut adc_data = [0u32;64];
    let mut send_buf = [0u8; 64];
    
    loop {
	if handle.device.is_none() {
	    loop {
		match handle.api.open(0x0484, 0x572f) {
		    Ok(dev) => {
			handle.device = Some(dev);
			break;
		    },
		    Err(e) => {
			println!("{:?}",e);
		    },
		}
		sleep(Duration::from_secs(3));
	    }
	    println!("ok");
	} else {
	    loop {
		let mut state_lock = state.lock().await;
		if state_lock.request_send {
		    state_lock.request_send = false;
		    for (page_num, keys) in state_lock.keys.chunks(4).enumerate() {
			send_buf[0] = 2;
			send_buf[1] = page_num as u8;
			for i in 0..4 {
 			    send_buf[2 + i*4+0] = keys[i].value.0 as u8 | ((keys[i].mode << 7) as u8);
			    send_buf[2 + i*4+1] = keys[i].value.1 as u8;
			    send_buf[2 + i*4+2] = keys[i].value.2 as u8;
			    send_buf[2 + i*4+3] = keys[i].value.3 as u8;
			}
			if let Ok(res) = handle.device.as_ref().unwrap().write(&send_buf[0..18]) {
			    sleep(Duration::from_millis(1));
			    println!("test: [send: {} page: {} payload: {:?}]", res, page_num, &send_buf[0..18]);
			} else {
			    handle.device = None;
			    break;
			}
			
			
		    }
		}
		drop(state_lock);
		
		if let Ok(res) = handle.device.as_ref().unwrap().read(&mut handle.recv_buf[..]) {

		    if handle.recv_buf[0] == 2 {
			let adc_data_page = handle.recv_buf[1] as usize;
			for (idx, x) in handle.recv_buf[2..18].iter().enumerate() {
			    if idx%2==1 {
				adc_data[idx/2 + adc_data_page*8] += x.clone() as u32;
			    }
			    else {
				adc_data[idx/2 + adc_data_page*8] = x.clone() as u32 *256;
			    }
			} 
			// println!("{:?}", res);
			send_interval+=1;
			if send_interval>8 {send_interval=0;}
			if send_interval==0 {
			    let adc_data_num = state.lock().await.key_monitor as usize;
			    output_tx.send(adc_data[adc_data_num].to_string()).await.unwrap();
			    
			}
		    }
		} else {
		    handle.device = None;
		    break;
		}

	    }
	    println!("Disconnected");
	}
    }
}

fn main() {
    tracing_subscriber::fmt::init();

    let (async_proc_input_tx, async_proc_input_rx) = mpsc::channel(1);
    let (async_proc_output_tx, mut async_proc_output_rx) = mpsc::channel(1);
    let state = Arc::new(Mutex::new(State{keys: vec![], key_monitor: 0, request_send: false}));
    
    tauri::Builder::default()
        .manage(
	    state.clone()
	)
	.manage(AsyncProcInputTx {
            inner: Mutex::new(async_proc_input_tx),
        })
	.setup(|app| {
	    tauri::async_runtime::spawn(async move {
		hid_thread(async_proc_input_rx, async_proc_output_tx, state.clone()).await;
	    });

	    let app_handle = app.handle();
            tauri::async_runtime::spawn(async move {
                loop {
                    if let Some(output) = async_proc_output_rx.recv().await {
                        // rs2js(output, &app_handle);
			app_handle.emit_all("adc_data", Payload{message: output}).unwrap();
                    }
                }
            });
	    Ok(())
	})
        .invoke_handler(tauri::generate_handler![upload_settings, set_adc_num])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


