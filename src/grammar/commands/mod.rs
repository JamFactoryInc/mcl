use crate::grammar::identifier::McIdentifier;
use crate::grammar::nbt::Nbt;
use crate::grammar::transform::{Coordinate, Rotation};
use crate::grammar::types::*;

pub enum CommandFragment {
    Keyword { command: &'static str },
    McIdentifier (McIdentifier),
    Decimal (Decimal),
    UDecimal (UDecimal),
    Range (Range),
    URange (URange),
    UDecimalRange (UDecimalRange),
    NBT (Box<Nbt>),
    UInt (usize),
    Int (usize),
    StackSize (usize),
    Rotation (Rotation),
    Coordinate (Coordinate),
}

pub enum Command {
    Advancement,
    Attribute,
    Ban,
    BanIp,
    Banlist,
    Bossbar,
    Clear,
    Clone,
    Damage,
    Data,
    Datapack,
    Debug,
    DefaultGamemode,
    Deop,
    Difficulty,
    Effect,
    Enchant,
    Execute,
    Experience,
    Fill,
    Fillbiome,
    Forceload,
    Function,
    Gamemode,
    Gamerule,
    Give,
    Help,
    Item,
    Jfr,
    Kick,
    Kill,
    List,
    Locate,
    Loot,
    Me,
    Msg,
    Op,
    Pardon,
    PardonIp,
    Particle,
    Perf,
    Place,
    Playsound,
    Publish,
    Recipe,
    Reload,
    Return,
    Ride,
    SaveAll,
    SaveOff,
    SaveOn,
    Say,
    Schedule,
    Scoreboard,
    Seed,
    Setblock,
    Setidletimeout,
    Setworldspawn,
    Spawnpoint,
    Spectate,
    Spreadplayers,
    Stop,
    Stopsound,
    Summon,
    Tag,
    Team,
    Teammsg,
    Teleport,
    Tell,
    Tellraw,
    Time,
    Title,
    Tm,
    Tp,
    Trigger,
    W,
    Weather,
    Whitelist,
    Worldborder,
    Xp,
}