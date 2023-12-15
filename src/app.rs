use std::time::Duration;

use leptos::*;
use tauri_sys::window;
use wasm_bindgen::prelude::*;
use leptos_router::*;
use web_sys::{Hid, HidDeviceRequestOptions, HidDevice, EventTarget};


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}


#[wasm_bindgen]
pub fn ripple_effect() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = std::rc::Rc::new(window.document().expect("should have a Document"));



    let doc = document.clone();
    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        let target = event.target().unwrap().dyn_into::<web_sys::Element>().unwrap();
//	logging::log!("{:?}",target.class_name());
        if target.class_list().contains("btn") || target.class_list().contains("ripple-effect") {
	    let ripple = doc.clone().create_element("div").unwrap();
	    ripple.set_class_name("ripple");
	    let ripple_container = doc.clone().create_element("div").unwrap();
	    ripple_container.set_class_name("ripple-container");
	    ripple_container.append_child(&ripple).unwrap();
	    
            let style = format!("top: {}px; left: {}px", event.offset_y(), event.offset_x());
            ripple.set_attribute("style", &style).unwrap();
//	    logging::log!("cnt: {:?} name: {:?}", target.child_element_count(), target.class_name());
            target.append_child(&ripple_container).unwrap();
	    set_timeout(move || {target.remove_child(&ripple_container).unwrap();}, Duration::from_secs(1));
        }
    }) as Box<dyn FnMut(_)>);

    document.clone().add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
    closure.forget();

//     let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
//         let target = event.target().unwrap().dyn_into::<web_sys::Element>().unwrap();
// 	logging::log!("{:?}", target.class_name());
//         if target.class_name().as_str() == "ripple" {
// //	    target.remove();
//         }
//     }) as Box<dyn FnMut(_)>);
//     document.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
//     closure.forget();
    
    Ok(())
}

use crate::{component::*, keyboard::{Keyboard, KeyCode}};


#[derive(Debug, Clone)]
pub struct UiState {
    pub hid_device: Option<HidDevice>,
    pub mode: u32,
    pub key_monitor: u32,
}

#[derive(Debug, Clone)]
pub struct ADC_Data {
    pub array: [u32; 64],
    pub cnt: usize,
}

#[component]
pub fn App() -> impl IntoView {
    let keyboard_state = create_rw_signal(Keyboard::new());
    let uistate = create_rw_signal(UiState{
	hid_device: None,
	mode: 0,
	key_monitor: 0,
    });
    provide_context(uistate);
    provide_context(keyboard_state);
    let (adc_datas, set_adc_datas) = create_signal(ADC_Data{array: [0u32; 64], cnt: 0});
    provide_context(adc_datas);
    provide_context(set_adc_datas);
    let chart_data = create_rw_signal(Vec::<u32>::new());
    provide_context(chart_data);
    
    let navbar_switch = create_signal(false);

    ripple_effect().unwrap();


    view! {
	
	<Router>
	   
	<div class="wraper" class:nav-open=navbar_switch.0>

	    <Sidebar/>

	  <div class="main-panel">

	    <Navbar navbar_switch/>







	    
	    <div class="content">
	      <div class="container-fluid">

	        <div class="row">

	    <Keyboard_View/>

	    <Routes>
	    <Route path="/performance" view=DashBoard/>
	    <Route path="/keymap" view=Profiles/>
	    <Route path="/rgb" view=Rgb/>
	    <Route path="/debug" view=Analog_Chart/>
	    </Routes>



	        </div>
	    
	    
	      </div>
	    </div>





	    <footer class="footer">
              <div class="container-fluid">
            <nav class="float-left">
            <ul>
              <li>
                <a href="">
                  About Us
                </a>
              </li>
              <li>
                <a href="https://www.creative-tim.com/license">
                  Licenses
                </a>
              </li>
            </ul>
            </nav>

	    
              <div class="copyright float-right" id="date">"© 2023
                , made with "<i class="material-icons">favorite</i> by
                <a href="https://www.creative-tim.com" target="_blank">Creative Tim</a>" for a better web."
              </div>
            </div>
            </footer>



	    
	  </div>
	    
	   

	    </div>
	    


	    </Router>
    }
    
}
