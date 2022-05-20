use crate::theme::Theme;

pub trait SwitchTo {
    /// Switches to the given variant.
    fn switch_to(&self, theme: &Theme);
}
