pub struct BatterBasicStats {
    player_id: u32,
    hits: u16,
    at_bats: u16,
    walks: u16,
    hit_by_pitch: u8,
    sacrifice_flies: u8,
    singles: u16,
    doubles: u16,
    triples: u8,
    home_runs: u8,

    batting_average: f32,
    on_base_percentage: f32,
    slugging_percentage: f32,
    on_base_plus_slugging: f32,
}

pub struct BatterAdvancedStats {
    player_id: u32,
    wrc_plus: i16,
    xwoba: f32,
    barrel_pct: f32,
    hard_hit_pct: f32,
    expected_batting_avg: f32,
}

pub struct PitcherBasicStats {
    player_id: u32,
    wins: u8,
    losses: u8,
    strikeouts: u16,
    saves: u8,
    holds: u8,
    innings_pitched: f32,
    earned_runs: u16,
    hits_allowed: u16,
    walks_allowed: u16,
    homes_runs_allowed: u8,
    era: f32,
}

pub struct PitcherAdvancedStats {
    player_id: u32,
    whip: f32,
    fip: f32,
    batting_average_against: f32,
}
