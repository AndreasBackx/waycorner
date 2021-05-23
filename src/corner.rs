use std::{
    borrow::Borrow,
    process::Command,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    time::Duration,
};

use anyhow::Result;
use regex::Regex;

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
        let mut last_event = None;
        loop {
            let event_result = self
                .channel
                .1
                .lock()
                .expect("cannot get corner receiver")
                .recv_timeout(Duration::from_millis(self.config.timeout_ms.into()));
            match event_result {
                Ok(event) => {
                    debug!("Received event: {:?}", event);
                    last_event = Some(event);
                }
                Err(_error) => {
                    if last_event.map_or(false, |value| value == CornerEvent::Enter) {
                        self.execute_command()?;
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

    fn execute_command(&self) -> Result<()> {
        if let Some(binary) = self.config.command.first() {
            let args = self
                .config
                .command
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
