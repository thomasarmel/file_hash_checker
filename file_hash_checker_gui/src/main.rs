// Hide console on Win32
#![windows_subsystem = "windows"]

mod application_state;
mod application_delegator;

use druid::widget::{Align, Label};
use druid::{AppLauncher, Env, LocalizedString, Widget, WindowDesc, WidgetExt, Color};

use application_state::ApplicationState;
use application_delegator::ApplicationDelegator;

const WINDOW_TITLE: LocalizedString<ApplicationState> = LocalizedString::new("File Hash Checker");

fn main() {
    let main_window = WindowDesc::new(build_root_widget)
        .title(WINDOW_TITLE)
        .resizable(false)
        .window_size((200.0, 200.0)); // Effective size could be larger

    let initial_application_state = ApplicationState {
        file_hash: Ok("Calculating file hash...".into()),
    };

    AppLauncher::with_window(main_window)
        .delegate(ApplicationDelegator)
        .launch(initial_application_state)
        .expect("Failed to launch GUI application");
}

fn build_root_widget() -> impl Widget<ApplicationState> {
    let label = Label::new(|data: &ApplicationState, _env: &Env| format!("{}", match data.file_hash {
        Ok(ref hash) => hash,
        Err(ref error) => *error,
    }))
        .with_text_color(Color::BLACK)
        .with_text_size(20.0);

    Align::centered(label).background(Color::rgb(242.0, 242.0, 242.0))
}