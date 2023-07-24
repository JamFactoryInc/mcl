
pub enum Cmds {
    Align(),
    Anchored,
    As,
    At,
    Facing,
    If,
    In,
    On,
    Positioned,
    Rotated,
    Run,
    Store,
    Summon,
    Unless,
}

pub mod r#if {
    pub enum Cmds {
        Biome,
        Block,
        Blocks,
        Data,
        Dimension,
        Entity,
        Loaded,
        Predicate,
        Score,
    }

    pub mod score {
        pub enum Cmds {
            Matches
        }
    }
}
