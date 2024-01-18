#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::{
    html::{data, div, input_data::keyboard_types::KeyboardEvent, style, text, textarea},
    prelude::*,
};
use dioxus_desktop::{
    tao::{
        dpi::Size,
        window::{Fullscreen, Window},
    },
    LogicalSize, WindowBuilder,
};

const RESET_CSS: &str = include_str!("reset.css");

fn main() {
    // launch the dioxus app in a webview
    dioxus_desktop::launch_cfg(
        App,
        // Note that we have to disable the viewport goofiness of the browser.
        // Dioxus_mobile should do this for us
        dioxus_desktop::Config::default()
            .with_window(WindowBuilder::default().with_maximized(true))
            .with_custom_head(format!(
                "
            <style>
                body {{
                    overflow: hidden;
                }}
                textarea {{
                    border: none;
                    outline: none;
                    font-size: 2em;
                    padding: 0.1em;
                    width: 100%;
                    height:auto;
                    background-color:#eee;
                }}
                {}
            </style>
        ",
                RESET_CSS
            )),
    );
}

#[derive(Clone, Debug)]
pub enum RowData {
    Paragraph { txt: String },
}

#[derive(Clone, Debug)]
pub struct Row {
    focus: bool,
    data: RowData,
}

impl Row {
    fn to_string(&self) -> String {
        match &self.data {
            RowData::Paragraph { txt } => txt.clone(),
        }
    }
    fn style(&self) -> String {
        let mut ret: String = match &self.data {
            RowData::Paragraph { txt } => {
                if txt.starts_with("#") {
                    "font-size: 1.2em; font-weight: bold"
                } else {
                    "font-size: 1em"
                }
            }
            _ => "font-size: 1em",
        }
        .into();
        if self.focus {
            ret += "; font-style:italic;";
        }
        ret
    }
}

// define a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    let rows = use_ref(cx, || {
        vec![Row {
            focus: true,
            data: RowData::Paragraph {
                txt: "".to_string(),
            },
        }]
    });
    cx.render(rsx! {
        div {
            div {
                style: "width: 25%; height: 100vh; display: inline-block; vertical-align:top;",
                background_color: "#222"
            },
            div {
                style: "width: 75%; height: 100vh; display: inline-block; vertical-align:top;font-size:2em",
                background_color: "#eee",
                for (i, row) in rows.read().iter().enumerate() {
                    textarea {
                        class: "row",
                        style: "{row.style()}",
                        onfocus: move |e| {
                            rows.write().get_mut(i).unwrap().focus = true;
                        },
                        onfocusout: move |e| {
                            rows.write().get_mut(i).unwrap().focus = false;
                        },
                        oninput: move |e| {
                            if e.data.value.ends_with("\n") {
                                if let Some(r)  = rows.write().get_mut(i) {
                                    *r = Row {
                                        focus: false,
                                        data: RowData::Paragraph {
                                            txt: e.data.value.to_string().strip_suffix("\n").unwrap_or_default().into()
                                        }
                                    };
                                }
                                if i == rows.read().len() - 1 {
                                    rows.write().push(
                                        Row {
                                            focus: true,
                                            data: RowData::Paragraph{ txt: "".into() }
                                        }
                                    );
                                } else {
                                    rows.write().get_mut(i+1).unwrap().focus = true;
                                }
                            } else if e.data.value.starts_with("#") {
                                if let Some(r)  = rows.write().get_mut(i) {
                                    *r = Row {
                                        focus: true,
                                        data: RowData::Paragraph {
                                            txt: e.data.value.to_string()
                                        }
                                    };
                                }
                            } else {
                                if let Some(r)  = rows.write().get_mut(i) {
                                    *r = Row {
                                        focus: true,
                                        data: RowData::Paragraph {
                                            txt: e.data.value.to_string()
                                        }
                                    };
                                }
                            }
                        },
                        onmounted: move |d| {
                            //if row.focus {
                                d.set_focus(true);
                            //}
                        },
                        value: "{row.to_string()}"
                    }
                }
            }
        }
    })
}
