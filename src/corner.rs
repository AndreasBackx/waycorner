use std::{
    borrow::Borrow,
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
pub struct CornerMotionEvent {
    pub time: Instant,
    pub surface_x: f64,
    pub surface_y: f64,
}

#[derive(Debug, PartialEq)]
pub enum CornerEvent {
    Enter,
    Leave,
    Motion(CornerMotionEvent),
}

#[derive(Debug, PartialEq)]
pub struct FlickInfo {
    last_motion_event: Option<CornerMotionEvent>,
    action_was_done: bool,
}

impl Default for FlickInfo {
    fn default() -> Self {
        FlickInfo {
            last_motion_event: None,
            action_was_done: false,
        }
    }
}

#[allow(clippy::type_complexity)]
#[derive(Debug)]
pub struct Corner {
    pub config: CornerConfig,
    pub channel: (
        Arc<Mutex<Sender<CornerEvent>>>,
        Arc<Mutex<Receiver<CornerEvent>>>,
    ),
    flick_info: Arc<Mutex<FlickInfo>>,
}

impl Corner {
    pub fn new(config: CornerConfig) -> Corner {
        let (tx, rx) = channel();
        Corner {
            config,
            channel: (Arc::new(Mutex::new(tx)), Arc::new(Mutex::new(rx))),
            flick_info: Default::default(),
        }
    }

    pub fn wait(&self) -> Result<()> {
        if self.config.timeout_ms != 0 {
            // FIXME current implementation incompatible with flick_command
            self.loop_with_timeout()
        } else {
            self.loop_without_timeout()
        }
    }

    pub fn send_event(&self, event: CornerEvent) -> Result<()> {
        self.channel
            .0
            .lock()
            .expect("Cannot get sender")
            .send(event)?;
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

    fn execute_command(&self, command: &[String]) -> Result<()> {
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

    fn execute_event(&self, event: CornerEvent) -> Result<()> {
        match event {
            CornerEvent::Enter => self.execute_command(&self.config.enter_command),
            CornerEvent::Leave => {
                *self.flick_info.lock().unwrap() = Default::default();
                self.execute_command(&self.config.exit_command)
            }
            CornerEvent::Motion { 0: motion_event } => {
                let mut flick_info = self.flick_info.lock().expect("Thread error");
                if !flick_info.action_was_done {
                    if let Some(last_motion_event) = flick_info.last_motion_event.as_ref() {
                        let delta = (motion_event.time - last_motion_event.time).as_millis() as f64;
                        let dx = motion_event.surface_x - last_motion_event.surface_x;
                        let dy = motion_event.surface_y - last_motion_event.surface_y;
                        let distance = (dx * dx + dy * dy).sqrt();
                        let speed = distance / delta;
                        dbg!(&speed);
                        if speed >= self.config.flick_activation_speed {
                            self.execute_command(&self.config.flick_command)?;
                            (*flick_info).action_was_done = true;
                        }
                    }
                    (*flick_info).last_motion_event = Some(motion_event);
                }
                Ok(())
            }
        }
    }

    fn loop_without_timeout(&self) -> Result<()> {
        loop {
            if let Ok(event) = self
                .channel
                .1
                .lock()
                .expect("cannot get corner receiver")
                .recv()
            {
                self.execute_event(event)?;
            }
        }
    }

    fn loop_with_timeout(&self) -> Result<()> {
        let timeout = Duration::from_millis(self.config.timeout_ms.into());
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
                        debug!("Ignored the event due to too fast after unlock.");
                    }
                }
                Err(_error) => {
                    if let Some(event) = last_event {
                        self.execute_event(event)?;
                        command_done_at = Some(Instant::now());
                    }
                    last_event = None;
                }
            }
        }
    }
}
