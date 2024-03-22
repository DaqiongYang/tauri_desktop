use tauri::{
    plugin::{Plugin, Result as PluginResult},
    AppHandle, Invoke, PageLoadPayload, Runtime, Window,
};

mod handle;
use common;

pub struct MysqlPlugin<R: Runtime> {
    invoke_handler: Box<dyn Fn(Invoke<R>) + Send + Sync>,
    // plugin state, configuration fields
}

// the plugin custom command handlers if you choose to extend the API.
#[tauri::command]
// this will be accessible with `invoke('plugin:awesome|initialize')`.
// where `awesome` is the plugin name.
fn test_link() {
    // 测试连接
    println!("mysql -  initialize")
}

#[tauri::command]
// this will be accessible with `invoke('plugin:awesome|do_something')`.
fn do_something() {
    println!("mysql -  do_something")
}

impl<R: Runtime> MysqlPlugin<R> {
    // you can add configuration fields here,
    // see https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
    pub fn new() -> Self {
        Self {
            invoke_handler: Box::new(tauri::generate_handler![test_link, do_something]),
        }
    }
}

impl<R: Runtime> Plugin<R> for MysqlPlugin<R> {
    /// The plugin name. Must be defined and used on the `invoke` calls.
    fn name(&self) -> &'static str {
        "mysql"
    }

    /// The JS script to evaluate on initialization.
    /// Useful when your plugin is accessible through `window`
    /// or needs to perform a JS task on app initialization
    /// e.g. "window.awesomePlugin = { ... the plugin interface }"
    fn initialization_script(&self) -> Option<String> {
        None
    }

    /// initialize plugin with the config provided on `tauri.conf.json > plugins > $yourPluginName` or the default value.
    #[allow(warnings, unused)]
    fn initialize(&mut self, app: &AppHandle<R>, config: serde_json::Value) -> PluginResult<()> {
        // println!("window: {:?}, payload: {:?} ", app, config);
        Ok(())
    }

    /// Callback invoked when the Window is created.
    #[allow(warnings, unused)]
    fn created(&mut self, window: Window<R>) {
        // println!("window: {:?}", window)
    }

    /// Callback invoked when the webview performs navigation.
    #[allow(warnings, unused)]
    fn on_page_load(&mut self, window: Window<R>, payload: PageLoadPayload) {
        // println!("window: {:?}, payload: {:?} ", window, payload)
    }

    /// Extend the invoke handler.
    fn extend_api(&mut self, message: Invoke<R>) {
        (self.invoke_handler)(message)
    }
}
