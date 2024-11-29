/// TODO
#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AnimationStyle {
    #[default]
    /// TODO
    Overlapping,
    /// TODO
    ToEnd,
}
