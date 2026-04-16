use niri_ipc::ColumnDisplay;

use crate::appearance::{
    BackgroundEffect, BackgroundEffectRule, BlockOutFrom, BorderRule, CornerRadius, ShadowRule,
    TabIndicatorRule,
};
use crate::layout::DefaultPresetSize;
use crate::utils::{MergeWith, RegexEq};
use crate::FloatOrInt;

#[derive(knuffel::Decode, Debug, Default, Clone, PartialEq)]
pub struct WindowRule {
    #[knuffel(children(name = "match"))]
    pub matches: Vec<Match>,
    #[knuffel(children(name = "exclude"))]
    pub excludes: Vec<Match>,

    // Rules applied at initial configure.
    #[knuffel(child)]
    pub default_column_width: Option<DefaultPresetSize>,
    #[knuffel(child)]
    pub default_window_height: Option<DefaultPresetSize>,
    #[knuffel(child, unwrap(argument))]
    pub open_on_output: Option<String>,
    #[knuffel(child, unwrap(argument))]
    pub open_on_workspace: Option<String>,
    #[knuffel(child, unwrap(argument))]
    pub open_maximized: Option<bool>,
    #[knuffel(child, unwrap(argument))]
    pub open_maximized_to_edges: Option<bool>,
    #[knuffel(child, unwrap(argument))]
    pub open_fullscreen: Option<bool>,
    #[knuffel(child, unwrap(argument))]
    pub open_floating: Option<bool>,
    #[knuffel(child, unwrap(argument))]
    pub open_focused: Option<bool>,

    // Rules applied dynamically.
    #[knuffel(child, unwrap(argument))]
    pub min_width: Option<u16>,
    #[knuffel(child, unwrap(argument))]
    pub min_height: Option<u16>,
    #[knuffel(child, unwrap(argument))]
    pub max_width: Option<u16>,
    #[knuffel(child, unwrap(argument))]
    pub max_height: Option<u16>,

    #[knuffel(child, default)]
    pub focus_ring: BorderRule,
    #[knuffel(child, default)]
    pub border: BorderRule,
    #[knuffel(child, default)]
    pub shadow: ShadowRule,
    #[knuffel(child, default)]
    pub tab_indicator: TabIndicatorRule,
    #[knuffel(child, unwrap(argument))]
    pub draw_border_with_background: Option<bool>,
    #[knuffel(child, unwrap(argument))]
    pub opacity: Option<f32>,
    #[knuffel(child)]
    pub geometry_corner_radius: Option<CornerRadius>,
    #[knuffel(child, unwrap(argument))]
    pub clip_to_geometry: Option<bool>,
    #[knuffel(child, unwrap(argument))]
    pub baba_is_float: Option<bool>,
    #[knuffel(child, unwrap(argument))]
    pub block_out_from: Option<BlockOutFrom>,
    #[knuffel(child, unwrap(argument))]
    pub variable_refresh_rate: Option<bool>,
    #[knuffel(child, unwrap(argument, str))]
    pub default_column_display: Option<ColumnDisplay>,
    #[knuffel(child)]
    pub default_floating_position: Option<FloatingPosition>,
    #[knuffel(child, unwrap(argument))]
    pub scroll_factor: Option<FloatOrInt<0, 100>>,
    #[knuffel(child, unwrap(argument))]
    pub tiled_state: Option<bool>,
    #[knuffel(child, default)]
    pub background_effect: BackgroundEffectRule,
    #[knuffel(child, default)]
    pub popups: PopupsRule,
}

/// Rules for popup surfaces.
#[derive(knuffel::Decode, Debug, Default, Clone, PartialEq)]
pub struct PopupsRule {
    #[knuffel(child, unwrap(argument))]
    pub opacity: Option<f32>,
    #[knuffel(child)]
    pub geometry_corner_radius: Option<CornerRadius>,
    #[knuffel(child, default)]
    pub background_effect: BackgroundEffectRule,
}

/// Resolved popup-specific rules.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct ResolvedPopupsRules {
    /// Extra opacity to draw popups with.
    pub opacity: Option<f32>,

    /// Corner radius to assume the popups have.
    pub geometry_corner_radius: Option<CornerRadius>,

    /// Background effect configuration for popups.
    pub background_effect: BackgroundEffect,
}

impl MergeWith<PopupsRule> for ResolvedPopupsRules {
    fn merge_with(&mut self, part: &PopupsRule) {
        if let Some(x) = part.opacity {
            self.opacity = Some(x);
        }
        if let Some(x) = part.geometry_corner_radius {
            self.geometry_corner_radius = Some(x);
        }
        self.background_effect.merge_with(&part.background_effect);
    }
}

#[derive(knuffel::Decode, Debug, Default, Clone, PartialEq)]
pub struct Match {
    #[knuffel(property, str)]
    pub app_id: Option<RegexEq>,
    #[knuffel(property, str)]
    pub title: Option<RegexEq>,
    #[knuffel(property)]
    pub is_active: Option<bool>,
    #[knuffel(property)]
    pub is_focused: Option<bool>,
    #[knuffel(property)]
    pub is_active_in_column: Option<bool>,
    #[knuffel(property)]
    pub is_floating: Option<bool>,
    #[knuffel(property)]
    pub is_window_cast_target: Option<bool>,
    #[knuffel(property)]
    pub is_urgent: Option<bool>,
    #[knuffel(property)]
    pub at_startup: Option<bool>,
}

#[derive(knuffel::Decode, Debug, Clone, Copy, PartialEq)]
pub struct FloatingPosition {
    #[knuffel(property)]
    pub x: FloatOrInt<-65535, 65535>,
    #[knuffel(property)]
    pub y: FloatOrInt<-65535, 65535>,
    #[knuffel(property, default)]
    pub relative_to: RelativeTo,
}

impl From<niri_ipc::FloatingPosition> for FloatingPosition {
    fn from(niri_ipc::FloatingPosition { x, y, relative_to }: niri_ipc::FloatingPosition) -> Self {
        Self {
            x: FloatOrInt(x),
            y: FloatOrInt(y),
            relative_to: relative_to.into(),
        }
    }
}

#[derive(knuffel::DecodeScalar, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum RelativeTo {
    #[default]
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Top,
    Bottom,
    Left,
    Right,
}

impl From<niri_ipc::RelativeTo> for RelativeTo {
    fn from(value: niri_ipc::RelativeTo) -> Self {
        match value {
            niri_ipc::RelativeTo::TopLeft => Self::TopLeft,
            niri_ipc::RelativeTo::TopRight => Self::TopRight,
            niri_ipc::RelativeTo::BottomLeft => Self::BottomLeft,
            niri_ipc::RelativeTo::BottomRight => Self::BottomRight,
            niri_ipc::RelativeTo::Top => Self::Top,
            niri_ipc::RelativeTo::Bottom => Self::Bottom,
            niri_ipc::RelativeTo::Left => Self::Left,
            niri_ipc::RelativeTo::Right => Self::Right,
        }
    }
}

impl From<RelativeTo> for niri_ipc::RelativeTo {
    fn from(value: RelativeTo) -> Self {
        match value {
            RelativeTo::TopLeft => Self::TopLeft,
            RelativeTo::TopRight => Self::TopRight,
            RelativeTo::BottomLeft => Self::BottomLeft,
            RelativeTo::BottomRight => Self::BottomRight,
            RelativeTo::Top => Self::Top,
            RelativeTo::Bottom => Self::Bottom,
            RelativeTo::Left => Self::Left,
            RelativeTo::Right => Self::Right,
        }
    }
}

impl From<FloatingPosition> for niri_ipc::FloatingPosition {
    fn from(FloatingPosition { x, y, relative_to }: FloatingPosition) -> Self {
        Self {
            x: x.0,
            y: y.0,
            relative_to: relative_to.into(),
        }
    }
}

impl From<crate::PresetSize> for niri_ipc::PresetSize {
    fn from(value: crate::PresetSize) -> Self {
        match value {
            crate::PresetSize::Proportion(v) => Self::Proportion(v),
            crate::PresetSize::Fixed(v) => Self::Fixed(v),
        }
    }
}

impl From<&WindowRule> for niri_ipc::SpawnRule {
    fn from(rule: &WindowRule) -> Self {
        Self {
            open_floating: rule.open_floating,
            open_maximized: rule.open_maximized,
            open_maximized_to_edges: rule.open_maximized_to_edges,
            open_fullscreen: rule.open_fullscreen,
            open_focused: rule.open_focused,
            open_on_output: rule.open_on_output.clone(),
            open_on_workspace: rule.open_on_workspace.clone(),
            default_column_display: rule.default_column_display,
            default_column_width: rule
                .default_column_width
                .as_ref()
                .map(|dps| dps.0.map(niri_ipc::PresetSize::from)),
            default_window_height: rule
                .default_window_height
                .as_ref()
                .map(|dps| dps.0.map(niri_ipc::PresetSize::from)),
            default_floating_position: rule
                .default_floating_position
                .map(niri_ipc::FloatingPosition::from),
        }
    }
}

/// Parse a KDL rule string (e.g. from `--rule`) into a `SpawnRule`.
///
/// The input uses the same syntax as window rule children in the config file,
/// with semicolons as separators:
///
/// ```text
/// open-floating true; default-floating-position x=100 y=100
/// ```
pub fn parse_spawn_rule(rule_str: &str) -> miette::Result<niri_ipc::SpawnRule> {
    // Wrap the rule string as children of a window-rule node.
    // Semicolons are valid KDL node terminators, but newlines also work.
    let kdl = format!("window-rule {{\n{}\n}}", rule_str.replace(';', "\n"));

    // Parse as a single-element document containing one WindowRule.
    #[derive(knuffel::Decode)]
    struct Wrapper {
        #[knuffel(child)]
        window_rule: WindowRule,
    }

    let wrapper: Wrapper = knuffel::parse("--rule", &kdl).map_err(|e| miette::miette!("{e}"))?;

    Ok(niri_ipc::SpawnRule::from(&wrapper.window_rule))
}
