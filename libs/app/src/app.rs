use game::Splat;
use gfx::{Commands};
use platform_types::{command, sprite, unscaled, Button, Input, Speaker, SFX};
pub use platform_types::StateParams;

pub struct State {
    pub game_state: game::State,
    pub commands: Commands,
    pub input: Input,
    pub speaker: Speaker,
}

impl State {
    pub fn new((seed, logger, error_logger): StateParams) -> Self {
        unsafe {
            features::GLOBAL_LOGGER = logger;
            features::GLOBAL_ERROR_LOGGER = error_logger;
        }

        // We always want to log the seed, if there is a logger available, so use the function,
        // not the macro.
        features::log(&format!("{:?}", seed));

        let game_state = game::State::new(seed);

        Self {
            game_state,
            commands: Commands::default(),
            input: Input::default(),
            speaker: Speaker::default(),
        }
    }
}

#[cfg_attr(feature = "reload", unsafe(no_mangle))]
pub fn frame(state: &mut State) -> (&[platform_types::Command], &[SFX]) {
    state.commands.clear();
    state.speaker.clear();
    update_and_render(
        &mut state.commands,
        &mut state.game_state,
        state.input,
        &mut state.speaker,
    );

    state.input.previous_gamepad = state.input.gamepad;

    (state.commands.slice(), state.speaker.slice())
}

pub fn press(state: &mut State, button: Button) {
    if state.input.previous_gamepad.contains(button) {
        //This is meant to pass along the key repeat, if any.
        //Not sure if rewriting history is the best way to do this.
        state.input.previous_gamepad.remove(button);
    }

    state.input.gamepad.insert(button);
}

pub fn release(state: &mut State, button: Button) {
    state.input.gamepad.remove(button);
}

fn update(state: &mut game::State, input: Input, speaker: &mut Speaker) {
    if input.gamepad != <_>::default() {
        state.add_splat();
        speaker.request_sfx(SFX::CardPlace);
    }
}

#[inline]
fn render(commands: &mut Commands, state: &game::State) {
    for &Splat { kind, x, y } in &state.splats {
        commands.draw_card(kind, x, y);

        // Negating this boolean is a quick way to test the hot reloading enabled by the "reload" feature.
        if true {
            commands.sspr(
                sprite::XY {
                    x: sprite::X(0),
                    y: sprite::Y(64),
                },
                command::Rect::from_unscaled(unscaled::Rect {
                    x: x.saturating_sub(unscaled::W(16)),
                    y: y.saturating_sub(unscaled::H(16)),
                    w: unscaled::W(16),
                    h: unscaled::H(16),
                })
            );
        }
    }
}

#[inline]
fn update_and_render(
    commands: &mut Commands,
    state: &mut game::State,
    input: Input,
    speaker: &mut Speaker,
) {
    update(state, input, speaker);
    render(commands, state);
}
