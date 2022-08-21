use druid::{AppDelegate, DelegateCtx, Env, Event, WindowId};
use crate::ApplicationState;

pub(crate) struct ApplicationDelegator;

impl AppDelegate<ApplicationState> for ApplicationDelegator {
    fn event(&mut self, _: &mut DelegateCtx, _: WindowId, event: Event, data: &mut ApplicationState, _: &Env) -> Option<Event> {
        match event {
            Event::WindowConnected => {
                data.update_file_hash_from_command_args();
                Some(Event::WindowConnected)
            },
            other => Some(other),
        }
    }
}