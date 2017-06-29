use {Timer, Color};
use serde_json::{to_writer, Result};
use std::io::Write;
use std::fmt::Write as FmtWrite;
use analysis::{state_helper, delta};
use time_formatter::{Delta, TimeFormatter, Accuracy};

#[derive(Default)]
pub struct Component {
    settings: Settings,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Settings {
    pub comparison_override: Option<String>,
    pub drop_decimals: bool,
    pub accuracy: Accuracy,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            comparison_override: None,
            drop_decimals: true,
            accuracy: Accuracy::Tenths,
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct State {
    pub text: String,
    pub time: String,
    pub color: Color,
}

impl Component {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_settings(settings: Settings) -> Self {
        Self {
            settings,
            ..Default::default()
        }
    }

    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    pub fn settings_mut(&mut self) -> &mut Settings {
        &mut self.settings
    }

    pub fn state(&self, timer: &Timer) -> State {
        let mut state = State::default();
        state.update(self, timer);
        state
    }
}

impl State {
    pub fn update(&mut self, component: &Component, timer: &Timer) {
        let comparison = component
            .settings
            .comparison_override
            .as_ref()
            .and_then(|c| timer.run().comparisons().find(|&rc| c == rc))
            .unwrap_or_else(|| timer.current_comparison());

        let (delta, use_live_delta) = delta::calculate(timer, comparison);

        let mut index = timer.current_split_index();
        if !use_live_delta {
            index -= 1;
        }
        let color = if index >= 0 {
            state_helper::split_color(
                timer,
                delta,
                index as usize,
                true,
                false,
                comparison,
                timer.current_timing_method(),
            )
        } else {
            Color::Default
        };

        self.text.clear();
        self.text.push_str(comparison);

        self.time.clear();
        write!(
            self.time,
            "{}",
            Delta::custom(
                component.settings.drop_decimals,
                component.settings.accuracy,
            ).format(delta)
        ).unwrap();

        self.color = color;
    }

    pub fn write_json<W>(&self, writer: W) -> Result<()>
    where
        W: Write,
    {
        to_writer(writer, self)
    }
}
