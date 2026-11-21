//! Spring motion for the Material 3 Expressive look.
//!
//! egui only ships fixed-duration easing. Expressive motion is spring physics: values can
//! overshoot and carry velocity when the target changes mid-flight. [`animate`] keeps one
//! spring per widget id in the context's temporary data and asks for repaints only until the
//! spring has settled, so an idle window stays idle.
use egui::{Context, Id};

/// Damping ratio and stiffness of a unit-mass spring.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Spring {
	pub damping: f32,
	pub stiffness: f32,
}

#[allow(
	dead_code,
	reason = "the full token set; controls adopt the slower and effect springs as they migrate"
)]
impl Spring {
	/// Moves things on screen (position, size, shape). Slightly under-damped, so it overshoots.
	pub const SPATIAL_FAST: Spring = Spring::new(0.6, 800.0);
	pub const SPATIAL: Spring = Spring::new(0.8, 380.0);
	pub const SPATIAL_SLOW: Spring = Spring::new(0.8, 200.0);
	/// Fades and tints. Critically damped: no overshoot, so colours never leave their range.
	pub const EFFECTS_FAST: Spring = Spring::new(1.0, 3800.0);
	pub const EFFECTS: Spring = Spring::new(1.0, 1600.0);
	pub const EFFECTS_SLOW: Spring = Spring::new(1.0, 800.0);

	pub const fn new(damping: f32, stiffness: f32) -> Spring {
		Spring { damping, stiffness }
	}
}

#[derive(Clone, Copy, Debug)]
struct State {
	value: f32,
	velocity: f32,
}

/// Longest step fed to the integrator; a stalled frame must not blow the spring up.
const MAX_FRAME: f32 = 1.0 / 30.0;
/// Integrator sub-step. Stiff effect springs need a small step to stay stable.
const SUB_STEP: f32 = 1.0 / 480.0;
const SETTLE_DISTANCE: f32 = 0.001;
const SETTLE_SPEED: f32 = 0.01;

/// Advance `state` toward `target` by `dt` seconds. Returns true once it has settled on it.
fn step(state: &mut State, target: f32, spring: Spring, dt: f32) -> bool {
	let damping_coefficient = 2.0 * spring.damping * spring.stiffness.sqrt();
	let mut remaining = dt.clamp(0.0, MAX_FRAME);
	while remaining > 0.0 {
		let h = remaining.min(SUB_STEP);
		let acceleration =
			spring.stiffness * (target - state.value) - damping_coefficient * state.velocity;
		// Semi-implicit Euler: stable for the stiffness range used here.
		state.velocity += acceleration * h;
		state.value += state.velocity * h;
		remaining -= h;
	}
	let settled =
		(target - state.value).abs() < SETTLE_DISTANCE && state.velocity.abs() < SETTLE_SPEED;
	if settled {
		state.value = target;
		state.velocity = 0.0;
	}
	settled
}

/// The spring value for `id` this frame, easing toward `target`.
///
/// The first call for an id starts at `target` without animating, so a widget that appears
/// already switched on does not slide in. A zero `animation_time` in the active style (the
/// app's reduced-motion setting) snaps straight to the target.
pub fn animate(ctx: &Context, id: Id, target: f32, spring: Spring) -> f32 {
	if ctx.style_of(ctx.theme()).animation_time <= 0.0 {
		return target;
	}
	let mut state = ctx
		.data(|data| data.get_temp::<State>(id))
		.unwrap_or(State {
			value: target,
			velocity: 0.0,
		});
	let dt = ctx.input(|input| input.stable_dt);
	let settled = step(&mut state, target, spring, dt);
	// Kept even when settled: the resting value is where the next change starts from.
	ctx.data_mut(|data| data.insert_temp(id, state));
	if !settled {
		ctx.request_repaint();
	}
	state.value
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn every_spring_settles_on_its_target_and_stops() {
		for spring in [
			Spring::SPATIAL_FAST,
			Spring::SPATIAL,
			Spring::SPATIAL_SLOW,
			Spring::EFFECTS_FAST,
			Spring::EFFECTS,
			Spring::EFFECTS_SLOW,
		] {
			let mut state = State {
				value: 0.0,
				velocity: 0.0,
			};
			let frames = (0..600).take_while(|_| !step(&mut state, 1.0, spring, 1.0 / 60.0));
			assert!(frames.count() < 600, "{spring:?} never settled");
			assert_eq!(state.value, 1.0);
		}
	}
}
