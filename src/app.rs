use leptos::*;
use wasm_bindgen::prelude::*;
use leptos_router::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}



use crate::{component::*, keyboard::Keyboard};


#[component]
pub fn App() -> impl IntoView {
    let keyboard_state = create_rw_signal(Keyboard::new());
    provide_context(keyboard_state);
	    
    let navbar_switch = create_signal(false);

    // let handle = window_event_listener(ev::keypress, |ev| {
    //     // ev is typed as KeyboardEvent automatically,
    //     // so .code() can be called
    //     let code = ev.time_stamp();
    //     logging::log!("code = {code:?}");
    // });
    // on_cleanup(move || handle.remove());



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
