use leptos::*;
use wasm_bindgen::prelude::*;
use leptos_router::*;
use web_sys::{Hid, HidDeviceRequestOptions, HidDevice};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}



use crate::{component::*, keyboard::Keyboard};


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
	    <Route path="/user" view=|| view! {profiles view}/>
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
