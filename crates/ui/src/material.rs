//! Material 3 colour scheme generated from a seed colour.
//!
//! Material's own generator works in HCT, which needs a table-driven solver. This module
//! uses the same idea with OKLCH: keep the seed's hue, fix a chroma per palette, and pick
//! lightness by CIE L* "tone" so tone 90 or 10 means what it does in the Material spec.
//! The colours are close to HCT's, not identical; swap in a real HCT implementation if
//! exact parity matters.
use crate::design::{DARK_DANGER, Palette};
use egui::Color32;

/// Hue (radians) and chroma of a colour in OKLab.
fn seed_hue_chroma([r, g, b]: [u8; 3]) -> (f32, f32) {
	let lin = |c: u8| {
		let c = f32::from(c) / 255.0;
		if c <= 0.04045 {
			c / 12.92
		} else {
			((c + 0.055) / 1.055).powf(2.4)
		}
	};
	let (r, g, b) = (lin(r), lin(g), lin(b));
	let l = (0.412_221_47 * r + 0.536_332_55 * g + 0.051_445_995 * b).cbrt();
	let m = (0.211_903_5 * r + 0.680_699_5 * g + 0.107_396_96 * b).cbrt();
	let s = (0.088_302_46 * r + 0.281_718_85 * g + 0.629_978_7 * b).cbrt();
	let a = 1.977_998_5 * l - 2.428_592_2 * m + 0.450_593_7 * s;
	let b = 0.025_904_037 * l + 0.782_771_77 * m - 0.808_675_77 * s;
	(b.atan2(a), a.hypot(b))
}

/// CIE L* (0-100) to relative luminance.
fn tone_to_luminance(tone: f32) -> f32 {
	if tone > 8.0 {
		((tone + 16.0) / 116.0).powi(3)
	} else {
		tone / 903.3
	}
}

/// Linear sRGB for an OKLab colour, or None when it falls outside the sRGB gamut.
fn oklab_to_linear(lightness: f32, a: f32, b: f32) -> Option<[f32; 3]> {
	let l = (lightness + 0.396_337_78 * a + 0.215_803_76 * b).powi(3);
	let m = (lightness - 0.105_561_346 * a - 0.063_854_17 * b).powi(3);
	let s = (lightness - 0.089_484_18 * a - 1.291_485_5 * b).powi(3);
	let rgb = [
		4.076_741_7 * l - 3.307_711_6 * m + 0.230_969_94 * s,
		-1.268_438 * l + 2.609_757_4 * m - 0.341_319_4 * s,
		-0.004_196_086 * l - 0.703_418_6 * m + 1.707_614_7 * s,
	];
	rgb.iter()
		.all(|c| (-0.001..=1.001).contains(c))
		.then_some(rgb)
}

/// The colour at `tone` for a hue and chroma, with chroma reduced until it fits in sRGB.
fn tonal(hue: f32, chroma: f32, tone: f32) -> Color32 {
	// For a neutral, OKLab lightness is the cube root of luminance.
	let lightness = tone_to_luminance(tone).cbrt();
	let mut chroma = chroma;
	let linear = loop {
		let (a, b) = (chroma * hue.cos(), chroma * hue.sin());
		if let Some(rgb) = oklab_to_linear(lightness, a, b) {
			break rgb;
		}
		if chroma < 0.001 {
			break [lightness.powi(3); 3];
		}
		chroma *= 0.92;
	};
	let encode = |c: f32| {
		let c = c.clamp(0.0, 1.0);
		let c = if c <= 0.003_130_8 {
			c * 12.92
		} else {
			1.055 * c.powf(1.0 / 2.4) - 0.055
		};
		(c * 255.0).round() as u8
	};
	Color32::from_rgb(encode(linear[0]), encode(linear[1]), encode(linear[2]))
}

/// A Material 3 scheme as Marshmellow's surface roles.
///
/// Surfaces step up through Material's surface-container tones; `selected` is the secondary
/// container, which Material uses for the active navigation item. `accent` is the primary
/// role (tone 80 in dark, 40 in light) with its on-primary text.
pub(crate) fn palette(dark: bool, seed: [u8; 3]) -> Palette {
	let (hue, seed_chroma) = seed_hue_chroma(seed);
	let primary = |tone| tonal(hue, seed_chroma.max(0.12), tone);
	let secondary = |tone| tonal(hue, 0.04, tone);
	let tertiary = |tone| tonal(hue + 1.05, 0.06, tone);
	let neutral = |tone| tonal(hue, 0.008, tone);
	let neutral_variant = |tone| tonal(hue, 0.016, tone);
	let (base, sidebar, chat, raised, hover, selected, border);
	let (text_strong, text, muted, link, accent, accent_text);
	let (mention_text, positive, warning, danger);
	if dark {
		base = neutral(4.0);
		sidebar = neutral(8.0);
		chat = neutral(10.0);
		raised = neutral(17.0);
		hover = neutral(20.0);
		selected = secondary(30.0);
		border = neutral_variant(30.0);
		text_strong = neutral(93.0);
		text = neutral_variant(82.0);
		muted = neutral_variant(68.0);
		link = primary(80.0);
		accent = primary(80.0);
		accent_text = primary(20.0);
		mention_text = primary(90.0);
		positive = tertiary(75.0);
		warning = Color32::from_rgb(0xe8, 0xa3, 0x3d);
		// Kept equal to the other dark presets so solid badges pick the same deeper red.
		danger = DARK_DANGER;
	} else {
		base = neutral(90.0);
		sidebar = neutral(94.0);
		chat = neutral(98.0);
		raised = neutral(92.0);
		hover = neutral(91.0);
		selected = secondary(86.0);
		border = neutral_variant(80.0);
		text_strong = neutral(8.0);
		text = neutral_variant(25.0);
		muted = neutral_variant(38.0);
		link = primary(40.0);
		accent = primary(40.0);
		accent_text = primary(100.0);
		mention_text = primary(30.0);
		positive = tertiary(38.0);
		warning = Color32::from_rgb(0xa8, 0x6a, 0x00);
		danger = Color32::from_rgb(0xb3, 0x26, 0x1e);
	}
	Palette {
		base,
		sidebar,
		chat,
		raised,
		hover,
		selected,
		border,
		text_strong,
		text,
		muted,
		link,
		accent,
		accent_text,
		positive,
		warning,
		danger,
		mention_bg: Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 76),
		mention_text,
		backdrop: None,
		canvas: chat,
		surface: sidebar,
	}
}
