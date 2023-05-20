use std::{
    borrow::Borrow,
    cmp,
    process::Command,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    time::{Duration, Instant},
};

use anyhow::Result;
use regex::Regex;
use tracing::{debug, info};

use crate::config::CornerConfig;

#[derive(Debug, PartialEq)]
pub enum CornerEvent {
    Enter,
    Leave,
}

#[derive(Debug)]
pub struct Corner {
    pub config: CornerConfig,
    pub channel: (
        Arc<Mutex<Sender<CornerEvent>>>,
        Arc<Mutex<Receiver<CornerEvent>>>,
    ),
}

impl Corner {
    pub fn new(config: CornerConfig) -> Corner {
        let (tx, rx) = channel();
        Corner {
            config,
            channel: (Arc::new(Mutex::new(tx)), Arc::new(Mutex::new(rx))),
        }
    }

    pub fn wait(&self) -> Result<()> {
        let timeout =
            Duration::from_millis(cmp::max(self.config.timeout_ms.into(), 5));
        let mut last_event = None;
        let mut command_done_at = None;
        loop {
            let event_result = self
                .channel
                .1
                .lock()
                .expect("cannot get corner receiver")
                .recv_timeout(timeout);
            match event_result {
                Ok(event) => {
                    debug!("Received event: {:?}", event);
                    if command_done_at.map_or(true, |value| {
                        Instant::now()
                            .duration_since(value)
                            .ge(&Duration::from_millis(250))
                    }) {
                        last_event = Some(event);
                    } else {
                        debug!(
                            "Ignored the event due to too fast after unlock."
                        );
                    }
                }
                Err(_error) => {
                    if let Some(event) = last_event {
                        if event == CornerEvent::Enter {
                            self.execute_command(&self.config.enter_command)?;
                        } else if event == CornerEvent::Leave {
                            self.execute_command(&self.config.exit_command)?;
                        }
                        command_done_at = Some(Instant::now());
                    }
                    last_event = None;
                }
            }
        }
    }

    pub fn on_enter_mouse(&self) -> Result<()> {
        self.channel
            .0
            .lock()
            .expect("Cannot get sender")
            .send(CornerEvent::Enter)?;
        Ok(())
    }

    pub fn on_leave_mouse(&self) -> Result<()> {
        self.channel
            .0
            .lock()
            .expect("Cannot get sender")
            .send(CornerEvent::Leave)?;
        Ok(())
    }

    pub fn is_match(&self, description: &str) -> bool {
        self.config
            .clone()
            .output
            .and_then(|value| value.description)
            .and_then(|value| Regex::new(value.as_str()).ok())
            .as_ref()
            .map(|regex| regex.is_match(description))
            .unwrap_or(true)
    }

    fn execute_command(&self, command: &Vec<String>) -> Result<()> {
        if let Some(binary) = command.first() {
            let args = command
                .iter()
                .enumerate()
                .filter(|(index, _)| index > 0.borrow())
                .map(|(_, value)| value)
                .collect::<Vec<_>>();
            info!("executing command: {} {:?}", binary, args);
            let output = Command::new(binary).args(args).output()?;
            info!("output received: {:?}", output);
        }

        Ok(())
    }
}
