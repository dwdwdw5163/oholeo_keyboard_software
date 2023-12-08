use std::ops::Not;
use leptos::*;
use leptos_router::*;
use plotters::coord::ranged1d::NoDefaultFormatting;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use tauri_sys::{event, tauri};
use serde::Deserialize;
use futures::StreamExt;
use web_sys::HidDevice;


use crate::{keyboard::{Keyboard, KEYBOARD_CHARS, KEYMAP, MessageArgs, STM2RS}, app::{UiState, ADC_Data}};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}


#[derive(Clone, serde::Deserialize)]
struct Payload {
  message: String,
}





pub const WIDTH: u32 = 64;
pub const HEIGHT: u32 = 64;




#[component]
pub fn Analog_Chart() -> impl IntoView {
//    let set_adc_datas = use_context::<WriteSignal<[u32;64]>>().unwrap();
    let event_vec = use_context::<RwSignal<Vec<u32>>>().unwrap();
//    create_resource(move || set_adc_datas, listen_on_adc_event);
//    let adc_vec_memo = create_memo(move |_| event_vec.get());
    
    use plotters::prelude::*;
    use plotters_canvas::CanvasBackend;

    let canvas_ref = create_node_ref::<html::Canvas>();
    create_effect(move |_| {
	let adc_datas = event_vec.get();
	let _canvas = canvas_ref.get().unwrap();
	let backend = CanvasBackend::with_canvas_object(canvas_ref.get().as_deref().unwrap().to_owned()).unwrap();
	let drawing_area = backend.into_drawing_area();
	drawing_area.fill(&RGBAColor(255,255,255,1.0)).unwrap();

	let drawing_area_vec = drawing_area.split_evenly((1,2));
	
	let mut chart = ChartBuilder::on(&drawing_area_vec[0])
            .caption("ADC_DATA", ("sans-serif", 14).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(0..128, 0..2048).unwrap();
	
	chart.configure_mesh().disable_mesh().draw().unwrap();
	
	chart.draw_series(
	    LineSeries::new(adc_datas.iter().enumerate().map(|(idx, x)| (idx as i32, *x as i32)), &GREEN)
	).unwrap();

	drawing_area.present().unwrap();
	
    });
    
    view! {


	<div class="card card-chart">
        <div class="card-header card-chart card-header-warning text-center">
	  <canvas id="canvas" style:width="100%" _ref=canvas_ref/>

            </div>

	    <div class="card-body">
        <h4 class="card-title">Daily Sales</h4>
        <p class="card-category"><span class="text-success"><i class="fa fa-long-arrow-up"></i> 55%  </span> increase in today sales.</p>
      </div>
      <div class="card-footer">
        <div class="stats">
          <i class="material-icons">access_time</i> updated 4 minutes ago
        </div>
      </div>
	</div>

    }

}

  
#[component]
fn Keycap_value(
    index: usize,
) -> impl IntoView{
    let keyboard_state = use_context::<RwSignal<Keyboard>>().unwrap();
    let pathname = use_location().pathname;
    let adc_datas = use_context::<ReadSignal<ADC_Data>>().unwrap();
    let adc_data = create_memo(move |_| adc_datas.get().array[index]);
    // let (adc_data, set_adc_data) = create_signal(0);
    // let adc_data_cnt = create_memo(move |_| adc_datas.get().cnt);
    // create_effect(move |_| {
	
    // });
    
    view! {
	<p class="m-0" style:font-weight="bold" style:font-size="20px">{move || KEYBOARD_CHARS[keyboard_state.get().keys[index].bind_key as usize]}</p>
	{
	    move || {
		if pathname.get().as_str() == "/performance" {
		    match keyboard_state.get().keys[index].mode {
			0 => {
			    view! {
				<p class="mb-0" style:font-size="12px">{move || keyboard_state.get().keys[index].value.0}</p>
				//	<p style:font-size="10px" style:height="8px" style:margin="0 0">{move || keyboard_state.get().keys[index].value.0}</p>
				//	<p style:font-size="10px" style:height="8px" style:margin="auto">" "</p>
			    }.into_view()
			},
			1 => {
			    view! {<p style:font-size="10px">{move || keyboard_state.get().keys[index].value.1}{" "}
				   {move || keyboard_state.get().keys[index].value.2}{" "}
				   {move || keyboard_state.get().keys[index].value.3}</p>
				   //   <p style:font-size="10px" style:height="8px" style:margin="0 0">{move || keyboard_state.get().keys[index].value.1}</p>
				   //   <p style:font-size="10px" style:height="8px" style:margin="0 0">{move || keyboard_state.get().keys[index].value.2}</p>
			    }.into_view()
			}
			_ => {view! {}.into_view()},
			
		    }
		} else if pathname.get().as_str() == "/debug" {
		    view! {
			<p>{move || adc_data.get()}</p>
		    }.into_view()
		} else {
		    view! {}.into_view()
		}
	    }
	}
    }

    
    // view! {
    // 	{
    // 	    move || {
    // 		if pathname.get().as_str() == "/performance" {
    // 		    match keyboard_state.get().keys[index].mode {
    // 			0 => {
    // 			    view! {
    // 				<p class="text-xs">"[Default]"</p>
    // 				//	<p style:font-size="10px" style:height="8px" style:margin="0 0">{move || keyboard_state.get().keys[index].value.0}</p>
    // 				//	<p style:font-size="10px" style:height="8px" style:margin="auto">" "</p>
    // 			    }.into_view()
    // 			},
    // 			1 => {
    // 			    view! {<p class="text-xs">"[RT]"</p>
    // 				   //   <p style:font-size="10px" style:height="8px" style:margin="0 0">{move || keyboard_state.get().keys[index].value.1}</p>
    // 				   //   <p style:font-size="10px" style:height="8px" style:margin="0 0">{move || keyboard_state.get().keys[index].value.2}</p>
    // 			    }.into_view()
    // 			}
    // 			_ => {view! {}.into_view()},
			
    // 		    }
    // 		}
    // 	    }
    // 	}
    // }
}


#[component]
fn KeyboardButton(
    index: usize,
) -> impl IntoView {
    let keyboard_state = use_context::<RwSignal<Keyboard>>().unwrap();
    let ui_state = use_context::<RwSignal<UiState>>().unwrap();
    
    let location = use_location().pathname;
    let on_click = move |_| {
	


	if location.get().as_str() == "/debug" {
	    //set monitor
	    ui_state.update(|v| v.key_monitor=index as u32);
	    keyboard_state.update(|Keyboard{keys, ..}| {
		for (idx, key) in keys.iter_mut().enumerate() {
		    if idx == index {
			key.selected = key.selected.not();
		    } else {
			key.selected = false;
		    }
		}
	    });
	} else {
	    ui_state.update(|state| {
		state.mode=keyboard_state.get().keys[index].mode;
	    });
	    keyboard_state.update(|Keyboard{keys, ..}| {
		for (idx, key) in keys.iter_mut().enumerate() {
		    if idx == index {
			key.selected = key.selected.not();
		    } else {
			//		    key.selected = false;
		    }
		}
	    });

	}
    };

    view! {
	<div
	    class="keycap-border"
	    	    //style:position="relative"
	    style:width=move || format!("{}px", keyboard_state.get().keys[index].size.0)
	    style:height=move || format!("{}px", keyboard_state.get().keys[index].size.1)>


	    <div class="keybutton"
	    style:position="relative"
	    style:width="100%"
	    style:top="0px"
	    on:click=on_click
	    class:active=move || keyboard_state.get().keys[index].selected>

	    <Keycap_value index/>
	    // <p style:font-weight="bold" style:height="24px" style:margin="0 0">{move || KEYBOARD_CHARS[keyboard_state.get().keys[index].bind_key as usize]}</p>
	    
	    // <div><Keycap_value index/></div>
	    
	    </div>
	    </div>
    }
}


 #[component]
 pub fn Keyboard_View(

 ) -> impl IntoView {
     

     view! {

	<div class="card">

	    <div class="card-body text-center" style:overflow="scroll" style:padding="0 0">
	 <div style:min-width=move || format!("{}px", 15*WIDTH)>

	 {(0..=13)
	  .map(|idx|
	       view! {<KeyboardButton index=idx/>}
	  ).collect_view()}
	 <br/>
	 {(14..=27)
	  .map(|idx|
	       view! {<KeyboardButton index=idx/>}
	  ).collect_view()}
	 <br/>
	 {(28..=40)
	  .map(|idx|
	       view! {<KeyboardButton index=idx/>}
	  ).collect_view()}
	 <br/>
	 {(41..=54)
	  .map(|idx|
	       view! {<KeyboardButton index=idx/>}
	  ).collect_view()}
	 <br/>
	 {(55..=63)
	  .map(|idx|
	       view! {<KeyboardButton index=idx/>}
	  ).collect_view()}

	 </div>
	     </div>
	     	 </div>
	     
     }




 }




#[component]
pub fn Sidebar() -> impl IntoView {
    let path_name = use_location().pathname;

    view! {
	  <div class="sidebar" data-color="purple" data-background-color="black" data-image="public/assets/img/sidebar-1.jpg">
	    <div class="logo"><a class="simple-text logo-normal">
              OHoleO Keyboard
            </a>
	    </div>
	    
	    <div class="sidebar-wrapper">
            <ul class="nav">
	    <li class="nav-item" class:active=move || path_name.get().as_str() == "/performance">
            <a class="nav-link" href="/performance">
            <i class="material-icons">bolt</i>
            <p>Performance</p>
            </a>
            </li>
	    <li class="nav-item" class:active=move || path_name.get().as_str() == "/keymap">
            <a class="nav-link" href="/keymap">
            <i class="material-icons">keyboard</i>
            <p>KeyMap</p>
            </a>
            </li>
            <li class="nav-item " class:active=move || path_name.get().as_str() == "/rgb">
            <a class="nav-link" href="/rgb">
            <i class="material-icons">palette</i>
            <p>R G B</p>
            </a>
            </li>
            <li class="nav-item " class:active=move || path_name.get().as_str() == "/debug">
            <a class="nav-link" href="/debug">
            <i class="material-icons">buildsharp</i>
            <p>DEBUG</p>
            </a>
            </li>
            </ul>
      </div>
	    <div class="sidebar-background" style="background-image: url(public/assets/img/sidebar-1.jpg) "></div>
	  </div>

    }


}

#[component]
pub fn Navbar(
    navbar_switch: (ReadSignal<bool>, WriteSignal<bool>),
) -> impl IntoView {
    let keyboard_state = use_context::<RwSignal<Keyboard>>().unwrap();
    let path_name = use_location().pathname;
    let uistate = use_context::<RwSignal<UiState>>().unwrap();
    let set_adc_datas = use_context::<WriteSignal<ADC_Data>>().unwrap();

    let adc_vec = use_context::<RwSignal<Vec<u32>>>().unwrap();
    let (dialog_switch, set_dialog_switch) = create_signal(false);
    // let title = move || match path_name.get().as_str(). {
    // 	"/performance" => "Performance",
    // 	"/keymap" => "KeyMap",
    // 	"/rgb" => "R G B",
    // 	_ => "HOME"
    // };
    let title = move || path_name.get().to_ascii_uppercase().as_str()[1..].to_string();
    let upload = move |_| {
	spawn_local(async move {
	    #[derive(serde::Serialize)]
	    struct Filter {
		vendorId: Option<u16>,
		productId: Option<u16>,
		#[serde(rename = "usagePage")]
		pub usage_page: Option<u16>,
		pub usage: Option<u16>,
	    }
	    if uistate.get().hid_device.is_none() {
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
		//		let device: &HidDevice = device .dyn_ref::<HidDevice>().expect("FAILED to cast `JsValue` in array into `HidDevice`.");
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
			//			logging::log!("{:?}", ba);
			let adc_data_page = ba[0] as usize;
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
		});
		device.set_oninputreport(Some(closure.as_ref().unchecked_ref()));
		closure.forget();
		
		uistate.update(|state| state.hid_device=Some(device));
	    } else {
		if let Some(device) = uistate.get().hid_device {

		    //open device

		    if device.opened().not() {
			wasm_bindgen_futures::JsFuture::from(device.open()).await.expect("Cannot Open Device");
		    }


		    let mut send_buf = [0u8; 64];
		    for (page_num, keys) in keyboard_state.get().keys.chunks(4).enumerate() {
			send_buf[0] = 2;
			send_buf[1] = page_num as u8;
			for i in 0..4 {
 			    send_buf[2 + i*4+0] = keys[i].value.0 as u8 | ((keys[i].mode << 7) as u8);
			    send_buf[2 + i*4+1] = keys[i].value.1 as u8;
			    send_buf[2 + i*4+2] = keys[i].value.2 as u8;
			    send_buf[2 + i*4+3] = keys[i].value.3 as u8;
			}
			//logging::log!("test: [page: {} payload: {:?}]", page_num, &send_buf[0..18]);
			if device.opened() {
			    let res = wasm_bindgen_futures::JsFuture::from(device.send_report_with_u8_array(2, &mut send_buf[1..18])).await;
			    match res {
				Err(err) => logging::log!("{:?}", err),
				_ => {},
			    }
			    //		    std::thread::sleep(std::time::Duration::from_millis(1));
			} else {
			    logging::log!("Device is not opend");
			}

		    }
		    logging::log!("Send Report");
		    set_dialog_switch.set(true);
		    //close device
		    //		wasm_bindgen_futures::JsFuture::from(device.close()).await.expect("Error While Closing the Device");
		}
	    }
//	    logging::log!("{:?}", result);
	    // let payload = keyboard_state.get().keys;
	    // let args = to_value(&MessageArgs {payload: &serde_json::to_string_pretty(&payload).unwrap()}).unwrap();
	    // let msg = invoke("upload_settings", args).await.as_string().unwrap();
	    // logging::log!("{}", msg);
	});
    };

	
    view! {

	<nav class="navbar navbar-expand-lg navbar-transparent navbar-absolute fixed-top " id="navigation-example">
            <div class="container-fluid">
            <div class="navbar-wrapper">
            <a class="navbar-brand" >{title}<div class="ripple-container"></div></a>
            </div>

	    <button type="submit" class="btn ml-auto" class:btn-success=move||uistate.get().hid_device.is_some() on:click=upload>{move|| {
		if uistate.get().hid_device.is_some() {
		    "Save to Keyboard"
		} else {
		    "Connect"
		}
	    }}</button>

<div class="modal fade" class:show=move||dialog_switch.get() tabindex="-1" role="dialog" class:block=move||dialog_switch.get()>
  <div class="modal-dialog" role="document">
    <div class="modal-content">
      <div class="modal-header">
        <h5 class="modal-title">Dialog</h5>
        <button type="button" class="close"  on:click=move|_|set_dialog_switch.set(false)>
            <i class="material-icons">close</i>
        </button>
      </div>
      <div class="modal-body">
            "Successfully Send Settings"
      </div>
      <div class="modal-footer">
        <button type="button" class="btn btn-primary" on:click=move|_|set_dialog_switch.set(false) >Close</button>
      </div>
    </div>
  </div>
 </div>
	    
            <button class="navbar-toggler collapsed" class:toggled=navbar_switch.0 type="button" on:click=move |_| {navbar_switch.1.update(|n| *n = n.not()) }>





	    
            <span class="sr-only">Toggle navigation</span>
            <span class="navbar-toggler-icon icon-bar"></span>
            <span class="navbar-toggler-icon icon-bar"></span>
            <span class="navbar-toggler-icon icon-bar"></span>
            </button>
	    
            </div>
	    </nav>

    }

}


#[component]
pub fn DashBoard(
    
) -> impl IntoView {
    
    view! {
	<KeySettings/>
    }

}


#[component]
pub fn KeySettings() -> impl IntoView {
    let uistate = use_context::<RwSignal<UiState>>().unwrap();

    let activation_value = create_signal("50".to_string());
    let trigger_value = create_signal("5".to_string());
    let reset_value = create_signal("5".to_string());
    let lower_deadzone = create_signal("35".to_string());

    
    let mode = create_memo(move |_| uistate.get().mode);
    let keyboard_state = use_context::<RwSignal<Keyboard>>().unwrap();
    let selected_num = create_memo(move |_| keyboard_state.get().keys.iter().filter(|key| key.selected).count());

    // create_effect(move |_| {
    // 	if selected_num.get() == 1 {
    // 	    let state = keyboard_state.get();
    // 	    let index = state.keys.iter().enumerate().filter(|(_idx, x)| x.selected == true).collect::<Vec<_>>()[0].0;
    // 	    mode.1.set(state.keys[index].mode.to_string());
    // 	    activation_value.1.set(state.keys[index].value.0.to_string());
    // 	    trigger_value.1.set(state.keys[index].value.1.to_string());
    // 	    reset_value.1.set(state.keys[index].value.2.to_string());
    // 	    lower_deadzone.1.set(state.keys[index].value.3.to_string());
    // 	}
    // });
 
    
    let update_activation_value =  move |ev| {
        let v = event_target_value(&ev);
	let number = selected_num.get();
        activation_value.1.set(v.clone());
	keyboard_state.update(|Keyboard{keys, ..}| {
	    for key in keys.iter_mut() {
		if key.selected || number==0 {
		    key.value.0 = v.parse::<u32>().unwrap();
		}
	    }
	});
    };

    let update_trigger_value =  move |ev| {
        let v = event_target_value(&ev);
	let number = selected_num.get();
        trigger_value.1.set(v.clone());
	keyboard_state.update(|Keyboard{keys, ..}| {
	    for key in keys.iter_mut() {
		if key.selected || number==0 {
		    key.value.1 = v.parse::<u32>().unwrap();
		}
	    }
	});
    };

    
    let update_reset_value =  move |ev| {
        let v = event_target_value(&ev);
	let number = selected_num.get();
        reset_value.1.set(v.clone());
	keyboard_state.update(|Keyboard{keys, ..}| {
	    for key in keys.iter_mut() {
		if key.selected || number==0{
		    key.value.2 = v.parse::<u32>().unwrap();
		}
	    }
	});
    };

    let update_lower_deadzone =  move |ev| {
        let v = event_target_value(&ev);
	let number = selected_num.get();
        lower_deadzone.1.set(v.clone());
	keyboard_state.update(|Keyboard{keys, ..}| {
	    for key in keys.iter_mut() {
		if key.selected || number==0{
		    key.value.3 = v.parse::<u32>().unwrap();
		}
	    }
	});
    };

    let update_mode = move |ev| {
	let v =  event_target_value(&ev);
	let number = selected_num.get();
	logging::log!("mode is {} selected: {}", v, number );
	uistate.update(|x| x.mode=v.parse::<u32>().unwrap());
	keyboard_state.update(|Keyboard{keys, ..}| {
	    for key in keys.iter_mut() {
		if key.selected || number==0 {
		    key.mode = v.parse::<u32>().unwrap();
		}
	    }
	});
	
    };
    
    view! {
	<div class="card">
	    <h4 class="card-header card-header-info">"Trigger Settings"</h4>
	    
	    <div class="card-body" style="display:flex;justify-content:space-around; height:240px;">


	    
	    <div style:width="40%">
	    <form style:width="100%"  
	    on:change=update_mode>
	    <h5>Mode</h5>
	    

	    <div class="form-check form-check-radio">
	    <label class="form-check-label">
            <input class="form-check-input" type="radio" name="mode" id="mode0" value="0" prop:checked=move || mode.get()==0/>
            "Traditional"
            <span class="circle">
            <span class="check"></span>
            </span>
	    </label>
	    </div>
	    <div class="form-check form-check-radio">
	    <label class="form-check-label">
            <input class="form-check-input" type="radio" name="mode" id="mode1" value="1" prop:checked=move || mode.get()==1/>
            "Rappid Trigger"
            <span class="circle">
            <span class="check"></span>
            </span>
	    </label>
	    </div>

	    </form>
	    
	    </div>

	    <div style:width="60%">

	{move || match mode.get().to_string().as_str() {
	    "0" => view! {
		<h5>"Activation Point"</h5>
		    <div class="form-row" style="justify-content:space-around; align-items:center;">
		    <span style:width="80%"><input type="range" min="1" max="100" class="slider" id="myRange0" prop:value=move||activation_value.0.get() on:input=update_activation_value/></span>
		    <input type="number" class="form-control" style:width="10%" prop:value=move||activation_value.0.get() on:input=update_activation_value/>
		    </div>
	    }.into_view(),
	    "1" => view! {
		<h5>"Dynamic Trigger Travel"</h5>
		    <div class="form-row" style="justify-content:space-around; align-items:center;">
		    <span style:width="80%"><input type="range" min="1" max="100" class="slider" id="myRange1" prop:value=move||trigger_value.0.get() on:input=update_trigger_value/></span>
		    <input type="number" class="form-control" style:width="10%" prop:value=move||trigger_value.0.get() on:input=update_trigger_value/>
		    </div>
		    
		    <h5>"Dynamic Reset Travel"</h5>
		    <div class="form-row" style="justify-content:space-around; align-items:center;">
		    <span style:width="80%"><input type="range" min="1" max="100" class="slider" id="myRange2" prop:value=move||reset_value.0.get() on:input=update_reset_value/></span>
		    <input type="number" class="form-control" style:width="10%" prop:value=move||reset_value.0.get() on:input=update_reset_value/>
		    </div>
		    <h5>"Lower DeadZone"</h5>
		    <div class="form-row" style="justify-content:space-around; align-items:center;">
		    <span style:width="80%"><input type="range" min="1" max="100" class="slider" id="myRange3" prop:value=move||lower_deadzone.0.get() on:input=update_lower_deadzone/></span>
		    <input type="number" class="form-control" style:width="10%" prop:value=move||lower_deadzone.0.get() on:input=update_lower_deadzone/>
		    
		    </div>
	    }.into_view(),
	    _ => view! {}.into_view(),

	    
	}}
		        

	   
	    
	    
	    </div>
	    

	    
	    </div>

	    

	</div>
	
    }
}




