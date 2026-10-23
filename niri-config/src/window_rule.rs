use knuffel::errors::DecodeError;
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
    #[knuffel(child, unwrap(argument))]
    pub on_xdg_activate: Option<OnXdgActivate>,

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
    pub variable_refresh_rate: Option<VrrRule>,
    #[knuffel(child, unwrap(argument, str))]
    pub default_column_display: Option<ColumnDisplay>,
    #[knuffel(child)]
    pub default_floating_position: Option<FloatingPosition>,
    #[knuffel(child, unwrap(argument))]
    pub scroll_factor: Option<FloatOrInt<0, 100>>,
    #[knuffel(child, unwrap(argument))]
    pub pinch_sensitivity: Option<FloatOrInt<0, 100>>,
    #[knuffel(child, unwrap(argument))]
    pub tiled_state: Option<bool>,
    #[knuffel(child, default)]
    pub background_effect: BackgroundEffectRule,
    #[knuffel(child, default)]
    pub popups: PopupsRule,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VrrRule {
    /// No effect on VRR.
    Off,
    /// Enable VRR when the window is visible.
    On,
    /// Enable VRR when the window is the current fullscreen window.
    FullscreenOnly,
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

#[derive(knuffel::DecodeScalar, Debug, Clone, Copy, PartialEq, Eq)]
pub enum OnXdgActivate {
    Ignore,
    SetUrgent,
    Focus,
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
            open_on_workspace_id: None,
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
///
/// On top of the window rule children, `open-on-workspace-id <id>` is accepted. It is
/// taken out before parsing since a workspace id only means something at runtime, so
/// it has no place in config window rules.
pub fn parse_spawn_rule(rule_str: &str) -> miette::Result<niri_ipc::SpawnRule> {
    let mut open_on_workspace_id = None;
    let mut children = Vec::new();
    for line in rule_str.split([';', '\n']) {
        match line.trim().strip_prefix("open-on-workspace-id") {
            Some(id) => {
                let id = id.trim().parse::<u64>().map_err(|e| {
                    miette::miette!("invalid open-on-workspace-id {:?}: {e}", id.trim())
                })?;
                open_on_workspace_id = Some(id);
            }
            None => children.push(line),
        }
    }

    // Wrap the rule string as children of a window-rule node.
    let kdl = format!("window-rule {{\n{}\n}}", children.join("\n"));

    // Parse as a single-element document containing one WindowRule.
    #[derive(knuffel::Decode)]
    struct Wrapper {
        #[knuffel(child)]
        window_rule: WindowRule,
    }

    let wrapper: Wrapper = knuffel::parse("--rule", &kdl).map_err(|e| miette::miette!("{e}"))?;

    Ok(niri_ipc::SpawnRule {
        open_on_workspace_id,
        ..niri_ipc::SpawnRule::from(&wrapper.window_rule)
    })
}

impl<S: knuffel::traits::ErrorSpan> knuffel::DecodeScalar<S> for VrrRule {
    fn raw_decode(
        val: &knuffel::span::Spanned<knuffel::ast::Literal, S>,
        _ctx: &mut knuffel::decode::Context<S>,
    ) -> Result<VrrRule, DecodeError<S>> {
        const ERROR: &str = "expected `true`, `\"fullscreen-only\"`, or `false`";
        match &**val {
            knuffel::ast::Literal::Bool(b) => Ok(if *b { VrrRule::On } else { VrrRule::Off }),
            knuffel::ast::Literal::String(s) => match &s[..] {
                // Also accept bool strings to reduce confusion.
                "true" => Ok(VrrRule::On),
                "false" => Ok(VrrRule::Off),
                "fullscreen-only" => Ok(VrrRule::FullscreenOnly),
                _ => Err(DecodeError::conversion(val, ERROR)),
            },
            _ => Err(DecodeError::conversion(val, ERROR)),
        }
    }

    fn type_check(
        type_name: &Option<knuffel::span::Spanned<knuffel::ast::TypeName, S>>,
        ctx: &mut knuffel::decode::Context<S>,
    ) {
        if let Some(typ) = type_name {
            ctx.emit_error(DecodeError::TypeName {
                span: typ.span().clone(),
                found: Some((**typ).clone()),
                expected: knuffel::errors::ExpectedType::no_type(),
                rust_type: stringify!(VrrRule),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_spawn_rule_workspace_id() {
        let rule = parse_spawn_rule("open-floating true; open-on-workspace-id 7").unwrap();
        assert_eq!(rule.open_on_workspace_id, Some(7));
        assert_eq!(rule.open_floating, Some(true));

        assert!(parse_spawn_rule("open-on-workspace-id nope").is_err());

        // The name-based key must not be taken for the id one.
        let rule = parse_spawn_rule(r#"open-on-workspace "dev""#).unwrap();
        assert_eq!(rule.open_on_workspace.as_deref(), Some("dev"));
        assert_eq!(rule.open_on_workspace_id, None);
    }
}
