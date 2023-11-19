use std::ops::Not;
use leptos::*;
use leptos_router::*;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use tauri_sys::{event, tauri};
use serde::Deserialize;
use futures::StreamExt;


use crate::keyboard::{Keyboard, KEYBOARD_CHARS, KEYMAP, MessageArgs};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}


#[derive(Clone, serde::Deserialize)]
struct Payload {
  message: String,
}

async fn listen_on_generic_event(event_rw: RwSignal<Vec<i64>>) {
    let mut events = event::listen::<Payload>("adc_data")
        .await
        .unwrap();

    while let Some(event) = events.next().await {
        event_rw.update(|all_events| {
	    all_events.push(event.payload.message.parse::<i64>().unwrap());
	    if all_events.len()>128 { (0..all_events.len()-128).for_each(|_| {all_events.remove(0);})}
	});
    }
}


pub const WIDTH: u32 = 64;
pub const HEIGHT: u32 = 64;




#[component]
pub fn Analog_Chart() -> impl IntoView {
    let event_vec = create_rw_signal::<Vec<i64>>(vec![]);
    create_local_resource(move || event_vec, listen_on_generic_event);
    let adc_vec_memo = create_memo(move |_| event_vec.get());
    
    use plotters::prelude::*;
    use plotters_canvas::CanvasBackend;

    let canvas_ref = create_node_ref::<html::Canvas>();
    create_effect(move |_| {
	let adc_datas = adc_vec_memo.get();
	let canvas = canvas_ref.get().unwrap();
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
    
    view! {
	{move || match keyboard_state.get().keys[index].mode {
	    0 => {
		view! {
		    <p class="text-xs">"[Default]"</p>
		//	<p style:font-size="10px" style:height="8px" style:margin="0 0">{move || keyboard_state.get().keys[index].value.0}</p>
		//	<p style:font-size="10px" style:height="8px" style:margin="auto">" "</p>
		}.into_view()
	    },
	    1 => {
		view! {<p class="text-xs">"[RT]"</p>
		    //   <p style:font-size="10px" style:height="8px" style:margin="0 0">{move || keyboard_state.get().keys[index].value.1}</p>
		    //   <p style:font-size="10px" style:height="8px" style:margin="0 0">{move || keyboard_state.get().keys[index].value.2}</p>
		}.into_view()
	    }
	    _ => {view! {}.into_view()},
	    
	}}
    }
}


#[component]
fn KeyboardButton(
    index: usize,
) -> impl IntoView {
    let keyboard_state = use_context::<RwSignal<Keyboard>>().unwrap();
    let location = use_location().pathname;
    let on_click = move |_| {
	keyboard_state.update(|Keyboard{keys, ..}| {
	    for (idx, key) in keys.iter_mut().enumerate() {
		if idx == index {
		    key.selected = key.selected.not();
		} else {
		    key.selected = false;
		}
	    }
	});

	if location.get().as_str() == "/debug" {
	    logging::log!("set adc num");
	    spawn_local(async move {
		let args = to_value(&MessageArgs{payload: KEYMAP[index].to_string().as_str()}).unwrap();
		let msg = invoke("set_adc_num", args).await.as_string().unwrap();
		logging::log!("{}", msg);
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
	// on:click=move |_| {keyboard_state.update(|Keyboard{keys, ..}| keys[index].selected = keys[index].selected.not())}
	    on:click=on_click
	class:active=move || keyboard_state.get().keys[index].selected>
	    <p style:font-weight="bold" style:height="24px" style:margin="0 0">{move || KEYBOARD_CHARS[keyboard_state.get().keys[index].bind_key as usize]}</p>
	    
	    <div><Keycap_value index/></div>
	    
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
    // let title = move || match path_name.get().as_str(). {
    // 	"/performance" => "Performance",
    // 	"/keymap" => "KeyMap",
    // 	"/rgb" => "R G B",
    // 	_ => "HOME"
    // };
    let title = move || path_name.get().to_ascii_uppercase().as_str()[1..].to_string();
    let upload = move |_| {
	spawn_local(async move {
	    let payload = keyboard_state.get().keys;
	    let args = to_value(&MessageArgs {payload: &serde_json::to_string_pretty(&payload).unwrap()}).unwrap();
	    let msg = invoke("upload_settings", args).await.as_string().unwrap();
	    logging::log!("{}", msg);
	});
    };

	
    view! {

	<nav class="navbar navbar-expand-lg navbar-transparent navbar-absolute fixed-top " id="navigation-example">
            <div class="container-fluid">
            <div class="navbar-wrapper">
            <a class="navbar-brand" >{title}<div class="ripple-container"></div></a>
            </div>

	    <button type="submit" class="btn btn-primary ml-auto" on:click=upload>"Save to Keyboard"</button>
	    
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

    let activation_value = create_signal("50".to_string());
    let trigger_value = create_signal("5".to_string());
    let reset_value = create_signal("5".to_string());
    let lower_deadzone = create_signal("35".to_string());

    
    let mode = create_signal("0".to_string());
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
	mode.1.set(v.clone());
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
            <input class="form-check-input" type="radio" name="mode" id="mode0" value="0" checked/>
            "Traditional"
            <span class="circle">
            <span class="check"></span>
            </span>
	    </label>
	    </div>
	    <div class="form-check form-check-radio">
	    <label class="form-check-label">
            <input class="form-check-input" type="radio" name="mode" id="mode1" value="1"/>
            "Rappid Trigger"
            <span class="circle">
            <span class="check"></span>
            </span>
	    </label>
	    </div>

	    </form>
	    
	    </div>

	    <div style:width="60%">

	{move || match mode.0.get().as_str() {
	    "0" => view! {
		<h5>"Activation Point"</h5>
		    <div class="form-row" style="justify-content:space-around; align-items:center;">
		    <span style:width="80%"><input type="range" min="1" max="100" class="slider" id="myRange0" prop:value=activation_value.0 on:input=update_activation_value/></span>
		    <input type="number" class="form-control" style:width="10%" prop:value=activation_value.0 on:input=update_activation_value/>
		    </div>
	    }.into_view(),
	    "1" => view! {
		<h5>"Dynamic Trigger Travel"</h5>
		    <div class="form-row" style="justify-content:space-around; align-items:center;">
		    <span style:width="80%"><input type="range" min="1" max="100" class="slider" id="myRange1" prop:value=trigger_value.0 on:input=update_trigger_value/></span>
		    <input type="number" class="form-control" style:width="10%" prop:value=trigger_value.0 on:input=update_trigger_value/>
		    </div>
		    
		    <h5>"Dynamic Reset Travel"</h5>
		    <div class="form-row" style="justify-content:space-around; align-items:center;">
		    <span style:width="80%"><input type="range" min="1" max="100" class="slider" id="myRange2" prop:value=reset_value.0 on:input=update_reset_value/></span>
		    <input type="number" class="form-control" style:width="10%" prop:value=reset_value.0 on:input=update_reset_value/>
		    </div>
		    <h5>"Lower DeadZone"</h5>
		    <div class="form-row" style="justify-content:space-around; align-items:center;">
		    <span style:width="80%"><input type="range" min="1" max="100" class="slider" id="myRange3" prop:value=lower_deadzone.0 on:input=update_lower_deadzone/></span>
		    <input type="number" class="form-control" style:width="10%" prop:value=lower_deadzone.0 on:input=update_lower_deadzone/>
		    
		    </div>
	    }.into_view(),
	    _ => view! {}.into_view(),

	    
	}}
		        

	   
	    
	    
	    </div>
	    

	    
	    </div>

	    

	</div>
	
    }
}




