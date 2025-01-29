use zellij_tile::prelude::*;

use std::collections::{BTreeMap, VecDeque};

#[derive(Default)]
struct State {
    sessions: Vec<SessionInfo>,
    current_session: usize,
    messages: VecDeque<String>,
    renders: usize,
}

impl State {
    fn next_session(&self) {
        if self.sessions.len() <= 1 {
            return;
        }
        let next_idx = (self.current_session + 1) % self.sessions.len();
        let next = &self.sessions[next_idx];
        switch_session(Some(next.name.as_str()));
    }

    fn prev_session(&self) {
        if self.sessions.len() <= 1 {
            return;
        }
        let next_idx = if self.current_session == 0 {
            self.sessions.len() - 1
        } else {
            (self.current_session - 1) % self.sessions.len()
        };
        let next = &self.sessions[next_idx];
        switch_session(Some(next.name.as_str()));
    }
}

register_plugin!(State);

// NOTE: you can start a development environment inside Zellij by running `zellij -l zellij.kdl` in
// this plugin's folder
//
// More info on plugins: https://zellij.dev/documentation/plugins

impl ZellijPlugin for State {
    fn load(&mut self, configuration: BTreeMap<String, String>) {
        request_permission(&[
            PermissionType::ReadApplicationState,
            PermissionType::ChangeApplicationState,
        ]);

        subscribe(&[EventType::SessionUpdate, EventType::Key]);
    }

    fn update(&mut self, event: Event) -> bool {
        // react to `Event`s that have been subscribed to (and the plugin has permissions for)
        // return true if this plugin's `render` function should be called for the plugin to render
        // itself
        self.messages.push_back("update running".to_string());

        match event {
            Event::Key(key) => self.messages.push_back("Key pressed".to_string()),
            Event::SessionUpdate(infos, _) => {
                self.messages.push_back("Event: SessionUpdate".to_string());
                self.sessions = infos;

                self.current_session = self
                    .sessions
                    .iter()
                    .enumerate()
                    .find(|(_, s)| s.is_current_session)
                    .unwrap()
                    .0;
            }
            _ => self.messages.push_back("unrecognized event".to_string()),
        }

        true
    }

    fn pipe(&mut self, pipe_message: PipeMessage) -> bool {
        // react to data piped to this plugin from the CLI, a keybinding or another plugin
        // read more about pipes: https://zellij.dev/documentation/plugin-pipes
        // return true if this plugin's `render` function should be called for the plugin to render
        // itself

        let Some(payload) = pipe_message.payload else {
            return false;
        };

        //switch_session(name);

        match payload.as_str() {
            "next" => self.next_session(),
            "prev" => self.prev_session(),
            _ => (),
        }
        
        hide_self();

        false
    }

    fn render(&mut self, rows: usize, cols: usize) {
        self.renders += 1;
        println!("Render: {}", self.renders);
        while let Some(msg) = self.messages.pop_front() {
            println!("{msg}");
        }
    }
}
