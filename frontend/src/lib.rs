use common::{ApiResponse, ImproveRequest, TranslateRequest};
use gloo_net::http::Request;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    // EN -> PL translation states
    let en_to_pl_text = use_state(|| String::new());
    let en_to_pl_result = use_state(|| String::new());
    let en_to_pl_loading = use_state(|| false);

    // PL -> EN translation states
    let pl_to_en_text = use_state(|| String::new());
    let pl_to_en_result = use_state(|| String::new());
    let pl_to_en_loading = use_state(|| false);

    // Improve text states
    let improve_text = use_state(|| String::new());
    let improve_result = use_state(|| String::new());
    let improve_loading = use_state(|| false);

    // EN -> PL input handler
    let on_en_to_pl_input = {
        let en_to_pl_text = en_to_pl_text.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlTextAreaElement>() {
                en_to_pl_text.set(input.value());
            }
        })
    };

    // PL -> EN input handler
    let on_pl_to_en_input = {
        let pl_to_en_text = pl_to_en_text.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlTextAreaElement>() {
                pl_to_en_text.set(input.value());
            }
        })
    };

    // Improve input handler
    let on_improve_input = {
        let improve_text = improve_text.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlTextAreaElement>() {
                improve_text.set(input.value());
            }
        })
    };

    // EN -> PL translate click handler
    let on_en_to_pl_click = {
        let en_to_pl_text = en_to_pl_text.clone();
        let en_to_pl_result = en_to_pl_result.clone();
        let en_to_pl_loading = en_to_pl_loading.clone();
        Callback::from(move |_| {
            let text = (*en_to_pl_text).clone();
            let result = en_to_pl_result.clone();
            let loading = en_to_pl_loading.clone();
            
            if text.trim().is_empty() {
                return;
            }

            loading.set(true);
            wasm_bindgen_futures::spawn_local(async move {
                let request = TranslateRequest {
                    text,
                    target_lang: Some("PL".to_string()),
                };

                match Request::post("http://127.0.0.1:3000/translate")
                    .json(&request)
                    .unwrap()
                    .send()
                    .await
                {
                    Ok(response) => {
                        match response.json::<ApiResponse>().await {
                            Ok(api_response) => {
                                if api_response.success {
                                    result.set(api_response.result);
                                } else {
                                    result.set(format!(
                                        "Error: {}",
                                        api_response.error.unwrap_or_else(|| "Unknown error".to_string())
                                    ));
                                }
                            }
                            Err(e) => {
                                result.set(format!("Parse error: {}", e));
                            }
                        }
                    }
                    Err(e) => {
                        result.set(format!("Request error: {}", e));
                    }
                }
                loading.set(false);
            });
        })
    };

    // PL -> EN translate click handler
    let on_pl_to_en_click = {
        let pl_to_en_text = pl_to_en_text.clone();
        let pl_to_en_result = pl_to_en_result.clone();
        let pl_to_en_loading = pl_to_en_loading.clone();
        Callback::from(move |_| {
            let text = (*pl_to_en_text).clone();
            let result = pl_to_en_result.clone();
            let loading = pl_to_en_loading.clone();
            
            if text.trim().is_empty() {
                return;
            }

            loading.set(true);
            wasm_bindgen_futures::spawn_local(async move {
                let request = TranslateRequest {
                    text,
                    target_lang: Some("EN".to_string()),
                };

                match Request::post("http://127.0.0.1:3000/translate")
                    .json(&request)
                    .unwrap()
                    .send()
                    .await
                {
                    Ok(response) => {
                        match response.json::<ApiResponse>().await {
                            Ok(api_response) => {
                                if api_response.success {
                                    result.set(api_response.result);
                                } else {
                                    result.set(format!(
                                        "Error: {}",
                                        api_response.error.unwrap_or_else(|| "Unknown error".to_string())
                                    ));
                                }
                            }
                            Err(e) => {
                                result.set(format!("Parse error: {}", e));
                            }
                        }
                    }
                    Err(e) => {
                        result.set(format!("Request error: {}", e));
                    }
                }
                loading.set(false);
            });
        })
    };

    // Improve click handler
    let on_improve_click = {
        let improve_text = improve_text.clone();
        let improve_result = improve_result.clone();
        let improve_loading = improve_loading.clone();
        Callback::from(move |_| {
            let text = (*improve_text).clone();
            let result = improve_result.clone();
            let loading = improve_loading.clone();
            
            if text.trim().is_empty() {
                return;
            }

            loading.set(true);
            wasm_bindgen_futures::spawn_local(async move {
                let request = ImproveRequest { 
                    text,
                    target_lang: None,
                    writing_style: None,
                    tone: None,
                };

                match Request::post("http://127.0.0.1:3000/improve")
                    .json(&request)
                    .unwrap()
                    .send()
                    .await
                {
                    Ok(response) => {
                        match response.json::<ApiResponse>().await {
                            Ok(api_response) => {
                                if api_response.success {
                                    result.set(api_response.result);
                                } else {
                                    result.set(format!(
                                        "Error: {}",
                                        api_response.error.unwrap_or_else(|| "Unknown error".to_string())
                                    ));
                                }
                            }
                            Err(e) => {
                                result.set(format!("Parse error: {}", e));
                            }
                        }
                    }
                    Err(e) => {
                        result.set(format!("Request error: {}", e));
                    }
                }
                loading.set(false);
            });
        })
    };

    html! {
        <div class="container">
        <div class="section">
                <h2 class="section-title">{"Improve Text"}</h2>
                <textarea
                    class="textarea"
                    rows="6"
                    cols="100"
                    style="width: 90%; min-width: 800px; font-size: 16px; padding: 15px;"
                    placeholder="Enter text to improve..."
                    value={(*improve_text).clone()}
                    oninput={on_improve_input}
                />
                <button
                    class={classes!("btn", "btn-success", improve_loading.then_some("loading"))}
                    onclick={on_improve_click}
                    disabled={*improve_loading}
                >
                    {if *improve_loading { "Improving..." } else { "Improve Text" }}
                </button>
                
                {if !improve_result.is_empty() {
                    let is_error = improve_result.starts_with("Error:") || improve_result.starts_with("Parse error:") || improve_result.starts_with("Request error:");
                    html! {
                        <div class={classes!("result", is_error.then_some("error"))}>
                            <h4 class="result-title">{"Improved Text:"}</h4>
                            <p class="result-text">{(*improve_result).clone()}</p>
                        </div>
                    }
                } else {
                    html! {}
                }}
            </div>
            
            <div class="section">
                <h2 class="section-title">{"Translate EN → PL"}</h2>
                <textarea
                    class="textarea"
                    rows="6"
                    cols="100"
                    style="width: 90%; min-width: 00px; font-size: 16px; padding: 15px;"
                    placeholder="Enter English text to translate to Polish..."
                    value={(*en_to_pl_text).clone()}
                    oninput={on_en_to_pl_input}
                />
                <button
                    class={classes!("btn", "btn-primary", en_to_pl_loading.then_some("loading"))}
                    onclick={on_en_to_pl_click}
                    disabled={*en_to_pl_loading}
                >
                    {if *en_to_pl_loading { "Translating..." } else { "Translate to Polish" }}
                </button>
                
                {if !en_to_pl_result.is_empty() {
                    let is_error = en_to_pl_result.starts_with("Error:") || en_to_pl_result.starts_with("Parse error:") || en_to_pl_result.starts_with("Request error:");
                    html! {
                        <div class={classes!("result", is_error.then_some("error"))}>
                            <h4 class="result-title">{"Polish Translation:"}</h4>
                            <p class="result-text">{(*en_to_pl_result).clone()}</p>
                        </div>
                    }
                } else {
                    html! {}
                }}
            </div>

            <div class="section">
                <h2 class="section-title">{"Translate PL → EN"}</h2>
                <textarea
                    class="textarea"
                    rows="6"
                    cols="100"
                    style="width: 90%; min-width: 400px; font-size: 16px; padding: 15px;"
                    placeholder="Enter Polish text to translate to English..."
                    value={(*pl_to_en_text).clone()}
                    oninput={on_pl_to_en_input}
                />
                <button
                    class={classes!("btn", "btn-secondary", pl_to_en_loading.then_some("loading"))}
                    onclick={on_pl_to_en_click}
                    disabled={*pl_to_en_loading}
                >
                    {if *pl_to_en_loading { "Translating..." } else { "Translate to English" }}
                </button>
                
                {if !pl_to_en_result.is_empty() {
                    let is_error = pl_to_en_result.starts_with("Error:") || pl_to_en_result.starts_with("Parse error:") || pl_to_en_result.starts_with("Request error:");
                    html! {
                        <div class={classes!("result", is_error.then_some("error"))}>
                            <h4 class="result-title">{"English Translation:"}</h4>
                            <p class="result-text">{(*pl_to_en_result).clone()}</p>
                        </div>
                    }
                } else {
                    html! {}
                }}
            </div>

        </div>
    }
} 