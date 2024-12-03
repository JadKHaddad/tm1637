/// Style for rotating circle.
#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RotatingStyle {
    #[default]
    Clockwise,
    CounterClockwise,
}
