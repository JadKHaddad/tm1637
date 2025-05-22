/// Direction for rotating circles.
#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RotatingDirection {
    /// Clockwise rotation.
    #[default]
    Clockwise,
    /// Counter-clockwise rotation.
    CounterClockwise,
}
