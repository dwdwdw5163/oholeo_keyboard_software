use std::{ops::Not, str::FromStr};
use leptos::*;
use leptos_router::*;
use plotters::{coord::ranged1d::NoDefaultFormatting, prelude::Ranged};
use serde_wasm_bindgen::to_value;
use strum::{VariantNames, IntoEnumIterator};
use wasm_bindgen::prelude::*;
use tauri_sys::{event, tauri};
use serde::Deserialize;
use futures::StreamExt;
use web_sys::{HidDevice, DragEvent};


use crate::{keyboard::*, app::{UiState, ADC_Data}};

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
pub fn Rgb() -> impl IntoView {
    use hex;
    let keyboard_state = use_context::<RwSignal<Keyboard>>().unwrap();
    let selected_num = create_memo(move |_| keyboard_state.get().keys.iter().filter(|key| key.selected).count());

    
    let on_color_change = move |e: ev::Event| {
	e.prevent_default();
	let v = event_target_value(&e);
	logging::log!("color: {:?}", v);
	let color = hex::decode(&v[1..]).unwrap_or(vec![255,255,255]);
	let number = selected_num.get();
	logging::log!("color: {:?}", color);
	keyboard_state.update(|Keyboard{keys, ..}| {
	    for key in keys.iter_mut() {
		if key.selected || number==0 {
		    key.rgb_raw = key.rgb_raw & 0xff000000;
		    key.rgb_raw = key.rgb_raw | ((color[0] as u32)<<16);
		    key.rgb_raw = key.rgb_raw | ((color[1] as u32)<<8);
		    key.rgb_raw = key.rgb_raw | ((color[2] as u32)<<0);
		}
	    }
	});
    };
    
    let on_gm_change = move |e: ev::Event| {
	e.prevent_default();
	let v = event_target_value(&e);
	logging::log!("gm: {:?}", v);
	let global_mode = RGB_GLOBAL_MODE::from_str(&v).unwrap_or(RGB_GLOBAL_MODE::RGB_GLOBAL_MODE_INDIVIDUAL) as u32;
	logging::log!("gm: {}", global_mode);

	keyboard_state.update(|Keyboard{keys, ..}| {
	    for key in keys.iter_mut() {
		    key.rgb_raw = key.rgb_raw & 0x0fffffff;
		    key.rgb_raw = key.rgb_raw | (global_mode<<28)
	    }
	});
    };

    let on_mode_change = move |e: ev::Event| {
	e.prevent_default();
	let v = event_target_value(&e);
	logging::log!("mode: {:?}", v);
	let mode = RGB_MODE::from_str(&v).unwrap_or(RGB_MODE::STATIC) as u32;
	logging::log!("mode: {}", mode);
	let number = selected_num.get();
	keyboard_state.update(|Keyboard{keys, ..}| {
	    for key in keys.iter_mut() {
		if key.selected || number==0 {
		    key.rgb_raw = key.rgb_raw & 0xf0ffffff;
		    key.rgb_raw = key.rgb_raw | (mode<<24)
		}
	    }
	});
    };
    
    

    view! {
	<div class="card">
	    <div class="card-header card-header-info">
	      <h5>RGB Settings</h5>
	    

	    </div>
	    <div class="card-body">

	    <form>
	    <div class="form-group">
	    <label for="color">Color</label>
	    <input id="color" type="color" on:change=on_color_change/>
	    </div>
	    
	    <div class="form-group">
	    <label for="global-mode">Global Mode</label>
	    <select id="global-mode" class="form-control" on:change=on_gm_change>
	{RGB_GLOBAL_MODE::iter().map(|gm| view! {<option>{gm.to_string()}</option>} ).collect_view()}
	    </select>
	    </div>

	    <div class="form-group">
	    <label for="mode">Mode</label>
	    <select id="mode" class="form-control" on:change=on_mode_change>
	{RGB_MODE::iter().map(|m| view! {<option>{m.to_string()}</option>} ).collect_view()}
	    </select>
	    </div>

	 
	    

	    </form>

	    </div>
	    <div class="card-footer">



	    </div>
	</div>

    }
}

#[component]
pub fn Profiles() -> impl IntoView {
    use strum::IntoEnumIterator;


    let on_dragstart = move |event: ev::DragEvent| {
	let data_transfer = event.data_transfer().unwrap();
	data_transfer.set_data("keycode", "test").unwrap();
    };
    let on_dragend = move |event: ev::DragEvent| {
	let data_transfer = event.data_transfer().unwrap();
	data_transfer.clear_data().unwrap();
    };
    
    view! {
	<div class="card">
	    <div class="card-header card-header-info">
	    
	    <ul class="nav nav-tabs">
	    <li class="nav-item">
            <a class="nav-link ripple-effect" href="">Basic</a>
	    </li>
	    <li class="nav-item">
            <a class="nav-link ripple-effect" href="">System</a>
	    </li>
	    <li class="nav-item">
            <a class="nav-link ripple-effect" href="">Joystick</a>
	    </li>
	    </ul>

	    </div>

	    <div class="card-body">
	
	    <div class="overflow-scroll" style:height="300px">{
		KeyCode::iter().enumerate().map(|(idx, code)| view! {
		    <div class="keycap-border" style="min-width:64px; height:64px">
			<div class="keybutton m-px p-3"
			on:dragstart=move|event: ev::DragEvent|{
			    let data_transfer = event.data_transfer().unwrap();
			    data_transfer.set_data("keycode", idx.to_string().as_str()).unwrap();
			}
			on:dragend=on_dragend
			draggable="true">
		        {code.to_string()}
			</div>
		    </div>
		    
		}).collect_view()
	    }</div>
	
	    
	    
	    </div>

	    <div class="card-footer">


	    </div>
	    
	</div>
    }
}


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
    let ui_state = use_context::<RwSignal<UiState>>().unwrap();
    let pathname = use_location().pathname;
    let adc_datas = use_context::<ReadSignal<ADC_Data>>().unwrap();
    let adc_data = create_memo(move |_| adc_datas.get().array[index]);
    // let (adc_data, set_adc_data) = create_signal(0);
    // let adc_data_cnt = create_memo(move |_| adc_datas.get().cnt);
    // create_effect(move |_| {
	
    // });
    let bind_keys = create_memo(move |_| {
	use std::convert::TryFrom;
	let keycode: i16 = match ui_state.get().layer {
	    0 => keyboard_state.get().keys[index].bind_key.0,
	    1 => keyboard_state.get().keys[index].bind_key.1,
	    _ => KeyCode::RESERVED as i16,

	};
	let keycode = KeyCode::try_from(keycode).unwrap_or(KeyCode::ERROR_UNDEFINED);
	keycode.to_string()
    });

    let rgb_mode = create_memo(move |_| {
	use strum::VariantNames;
	let rgb_raw = keyboard_state.get().keys[index].rgb_raw;
	let mode = (rgb_raw >> 24) & 0x0f;
	RGB_MODE::VARIANTS[mode as usize].to_string()
    });

    let rgb_color = create_memo(move |_| {
	let color = keyboard_state.get().keys[index].rgb_raw & 0x00ffffff;
	format!("#{:06X}", color)
    });
    
    view! {

	{
	    move || {
		if pathname.get().as_str() == "/performance" {
		    match keyboard_state.get().keys[index].mode {
			0 => {
			    view! {
				<p class="m-0" style:font-weight="bold" style:font-size="20px">{move || KEYBOARD_CHARS[keyboard_state.get().keys[index].index as usize]}</p>
				<p class="mb-0" style:font-size="12px">{move || keyboard_state.get().keys[index].value.0}</p>
				//	<p style:font-size="10px" style:height="8px" style:margin="0 0">{move || keyboard_state.get().keys[index].value.0}</p>
				//	<p style:font-size="10px" style:height="8px" style:margin="auto">" "</p>
			    }.into_view()
			},
			1 => {
			    view! {
				<p class="m-0" style:font-weight="bold" style:font-size="20px">{move || KEYBOARD_CHARS[keyboard_state.get().keys[index].index as usize]}</p>
				<p style:font-size="10px">{move || keyboard_state.get().keys[index].value.1}{" "}
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
			<p class="m-0" style:font-weight="bold" style:font-size="20px">{move || KEYBOARD_CHARS[keyboard_state.get().keys[index].index as usize]}</p>
			<p>{move || adc_data.get()}</p>
		    }.into_view()
		} else if pathname.get().as_str() == "/keymap" {
		    view! {
			<p class="m-0 p-1.5" style:font-weight="bold" style:font-size="12px">{bind_keys}</p>
		    }.into_view()
		} else if pathname.get().as_str() == "/rgb" { 
		    view! {
			<p style:font-weight="bold" style:font-size="12px">{move||rgb_mode.get()}</p>
			    <p style:background-color=move||rgb_color.get() style:color=move||rgb_color.get()>"COLOR"</p>
		    }.into_view()
		} else {
		    view! {
			<p class="m-0" style:font-weight="bold" style:font-size="20px">{move || KEYBOARD_CHARS[keyboard_state.get().keys[index].index as usize]}</p>
			<p class="m-0"></p>
		    }.into_view()
		}
	    }
	}
    }
}


#[component]
fn KeyboardButton(
    index: usize,
) -> impl IntoView {
    let keyboard_state = use_context::<RwSignal<Keyboard>>().unwrap();
    let ui_state = use_context::<RwSignal<UiState>>().unwrap();

    let select = move || keyboard_state.get().keys[index].selected;
    
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

    let on_dragenter = move |event: ev::DragEvent| {
	event.prevent_default();
	keyboard_state.update(|Keyboard{keys, ..}| {
	    keys[index].selected = true;
	});
	logging::log!("dragenter index {}",index);
    };

    let on_dragleave = move |event: ev::DragEvent| {
	event.prevent_default();
	keyboard_state.update(|Keyboard{keys, ..}| {
	    keys[index].selected = false;
	});
	logging::log!("dragleave index {}",index);
   };
    
    let on_dragover = move |event: ev::DragEvent| {
	event.prevent_default();
//	logging::log!("dragover index {}", index);
    };

    let on_drop = move |event: ev::DragEvent| {
	let data_transfer = event.data_transfer().unwrap();
	let index = data_transfer
	    .get_data("keycode").unwrap()
	    .parse::<usize>().unwrap();
	let keycode = KeyCode::iter().enumerate().find(|(idx, _code)| index==*idx).unwrap_or((0,KeyCode::RESERVED)).1 as i16;
	logging::log!("drop index {}, data {:?}, keycode {}",
		      index,
		      KeyCode::VARIANTS[index],
		      keycode);
	keyboard_state.update(|Keyboard{keys, ..}| {
	    for key in keys.iter_mut() {
		if key.selected {
		    key.selected = false;
		    match ui_state.get().layer {
			0 => key.bind_key.0 = keycode,
			1 => key.bind_key.1 = keycode,
			_ => {},
		    }
		    
		}
	    }
	});
    };
    
    view! {
	<div
	    class="keycap-border"

	    style:width=move || format!("{}px", keyboard_state.get().keys[index].size.0)
	    style:height=move || format!("{}px", keyboard_state.get().keys[index].size.1)>


	    <div class="keybutton"
	    droppable="true"
	    on:dragover=on_dragover
	    on:drop=on_drop
	    on:dragenter=on_dragenter
	    on:dragleave=on_dragleave
	    style:position="relative"
	    style:width="100%"
	    style:top="0px"
	    on:click=on_click
	    
	    class:active=move || keyboard_state.get().keys[index].selected>

	    <div class="pointer-events-none">
	    <Keycap_value index/>
	    </div>
	    // <p style:font-weight="bold" style:height="24px" style:margin="0 0">{move || KEYBOARD_CHARS[keyboard_state.get().keys[index].bind_key as usize]}</p>
	    
	    // <div><Keycap_value index/></div>
	    
	    </div>
	    </div>
    }
}


 #[component]
 pub fn Keyboard_View(

 ) -> impl IntoView {
     let pathname = use_location().pathname;
     let ui_state = use_context::<RwSignal<UiState>>().unwrap();
     
     let switch_layer = move |_| {
	 ui_state.update(|state| {
	     if state.layer == 0 {
		 state.layer = 1;
	     } else {
		 state.layer = 0;
	     }
	 });
     };

     let layer_name = create_memo(move |_| {
	 if ui_state.get().layer == 0 {
	     "Default".to_string()
	 } else {
	     "Fn Layer".to_string()
	 }
     });
     
     view! {

	 <div class="card">

	     <Show when=move || pathname.get().as_str()=="/keymap">
	     <div class="card-header card-header-info">
	     <button class="btn btn-primary ml-auto" on:click=switch_layer>{layer_name}</button>
	     </div>
	     </Show>
	     

	    <div class="card-body text-center" style:overflow="scroll" style:padding="0 0">
	 <div style:min-width=move || format!("{}px", 15*WIDTH)>

	 <div>{(0..=13)
	  .map(|idx|
	       view! {<KeyboardButton index=idx/>}
	  ).collect_view()}</div>

	 <div>{(14..=27)
	  .map(|idx|
	       view! {<KeyboardButton index=idx/>}
	  ).collect_view()}</div>

	 <div>{(28..=40)
	  .map(|idx|
	       view! {<KeyboardButton index=idx/>}
	  ).collect_view()}</div>

	 <div>{(41..=54)
	  .map(|idx|
	       view! {<KeyboardButton index=idx/>}
	  ).collect_view()}</div>

	 <div>{(55..=63)
	  .map(|idx|
	       view! {<KeyboardButton index=idx/>}
	  ).collect_view()}</div>

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

    let title = move || path_name.get().to_ascii_uppercase().as_str()[1..].to_string();
    let upload = move |_| {
	spawn_local(async move {
	    if let Some(device) = uistate.get().hid_device {
		let mut send_buf = [0u8; 64];
		match path_name.get().as_str() {
		    "/performance" => {
			send_performance_report(&mut send_buf, &keyboard_state.get(), &device).await;
		    },
		    "/rgb" => {
			send_rgb_report(&mut send_buf, &keyboard_state.get(), &device).await;
		    },
		    "/keymap" => {
			send_keymap_report(&mut send_buf, &keyboard_state.get(), &device).await;
		    },
		    _ => {},
		}
		logging::log!("Send Report");
		set_dialog_switch.set(true);
	    } else {
		init_hid_device(uistate, keyboard_state, set_adc_datas, adc_vec).await;
	    }
	});
    };

	
    view! {

	<nav class="navbar navbar-expand-lg navbar-transparent navbar-absolute fixed-top " id="navigation-example">
            <div class="container-fluid">
            <div class="navbar-wrapper">
            <a class="navbar-brand" >{title}<div class="ripple-container"></div></a>
            </div>

	    <button type="submit" class="btn ml-auto overflow-hidden relative" class:btn-success=move||uistate.get().hid_device.is_some() on:click=upload>{move|| {
		if uistate.get().hid_device.is_some() {
		    "Save to Keyboard"
		} else {
		    "Connect"
		}

	    }}
	</button>

	    <div class="modal fade" class:show=move||dialog_switch.get() tabindex="-1" role="dialog"
	    style:display=move||{if dialog_switch.get() {"block"}else{"none"}}
	    >
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
    
    let update_activation_value =  move |ev| {
        let v = event_target_value(&ev);
	let value = v.parse::<u32>().unwrap();
	if value>0 && value<=100 {
	    let number = selected_num.get();
            activation_value.1.set(v.clone());
	    keyboard_state.update(|Keyboard{keys, ..}| {
		for key in keys.iter_mut() {
		    if key.selected || number==0 {
			key.value.0 = v.parse::<u32>().unwrap();
		    }
		}
	    });
	}
    };

    let update_trigger_value =  move |ev| {
        let v = event_target_value(&ev);
	let value = v.parse::<u32>().unwrap();
	if value>0 && value<=100 {
	    let number = selected_num.get();
            trigger_value.1.set(v.clone());
	    keyboard_state.update(|Keyboard{keys, ..}| {
		for key in keys.iter_mut() {
		    if key.selected || number==0 {
			key.value.1 = v.parse::<u32>().unwrap();
		    }
		}
	    });
	}
    };

    
    let update_reset_value =  move |ev| {
        let v = event_target_value(&ev);	
	let value = v.parse::<u32>().unwrap();
	if value>0 && value<=100 {
	    let number = selected_num.get();
            reset_value.1.set(v.clone());
	    keyboard_state.update(|Keyboard{keys, ..}| {
		for key in keys.iter_mut() {
		    if key.selected || number==0{
			key.value.2 = v.parse::<u32>().unwrap();
		    }
		}
	    });
	}
    };

    let update_lower_deadzone =  move |ev| {
        let v = event_target_value(&ev);
	let value = v.parse::<u32>().unwrap();
	if value>0 && value<=100 {
	    let number = selected_num.get();
            lower_deadzone.1.set(v.clone());
	    keyboard_state.update(|Keyboard{keys, ..}| {
		for key in keys.iter_mut() {
		    if key.selected || number==0{
			key.value.3 = v.parse::<u32>().unwrap();
		    }
		}
	    });
	}
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
		    <input type="number" class="form-control" min="1" max="100" style:width="10%" prop:value=move||activation_value.0.get() on:change=update_activation_value/>
		    </div>
	    }.into_view(),
	    "1" => view! {
		<h5>"Dynamic Trigger Travel"</h5>
		    <div class="form-row" style="justify-content:space-around; align-items:center;">
		    <span style:width="80%"><input type="range" min="1" max="100" class="slider" id="myRange1" prop:value=move||trigger_value.0.get() on:input=update_trigger_value/></span>
		    <input type="number" class="form-control" min="1" max="100" style:width="10%" prop:value=move||trigger_value.0.get() on:change=update_trigger_value/>
		    </div>
		    
		    <h5>"Dynamic Reset Travel"</h5>
		    <div class="form-row" style="justify-content:space-around; align-items:center;">
		    <span style:width="80%"><input type="range" min="1" max="100" class="slider" id="myRange2" prop:value=move||reset_value.0.get() on:input=update_reset_value/></span>
		    <input type="number" class="form-control" min="1" max="100" style:width="10%" prop:value=move||reset_value.0.get() on:change=update_reset_value/>
		    </div>
		    <h5>"Lower DeadZone"</h5>
		    <div class="form-row" style="justify-content:space-around; align-items:center;">
		    <span style:width="80%"><input type="range" min="1" max="100" class="slider" id="myRange3" prop:value=move||lower_deadzone.0.get() on:input=update_lower_deadzone/></span>
		    <input type="number" class="form-control" min="1" max="100" style:width="10%" prop:value=move||lower_deadzone.0.get() on:change=update_lower_deadzone/>
		    
		    </div>
	    }.into_view(),
	    _ => view! {}.into_view(),

	    
	}}
		        

	   
	    
	    
	    </div>
	    

	    
	    </div>

	    

	</div>
	
    }
}




