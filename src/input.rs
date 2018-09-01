pub enum Input {
    None,
    Move(::direction::Direction),
    Turn(::direction::Direction),
    Quit,
}
