enum InheritanceFactor {
    Speed,
    Sprint,
    Stamina,
    Guts,
    QuickTemper,
}

enum RaceStrategyAptitude {
    PaceLeader(u8),
    FrontRunner(u8),
    HoldUpRunner(u8),
    StretchRunner(u8),
}
// For Japanese:
// PaceLeader:逃げ
// FrontRunner:先行
// HoldUpRunner:差し
// StretchRunner:追い込み

enum Sex {
    Male,
    Female,
}
enum HorceStatus {
    Speed(u8),
    Sprint(u8),
    Stamina(u8),
    Guts(u8),
    Health(u8),
    Temperament(u8),
    Weight(u16),
    Sex(Sex),
}
// For Japanese:
// Speed:追走力
// Sprint:瞬発力
// Stamina:持久力
// Guts:勝負根性
// Health:回復力
// Temperament:気性

struct Pedigree;

struct Horse {
    sire: String,
    dam: String,
    date_of_birth: String,
    status: HorceStatus,
}

fn main() {}

#[cfg(test)]
mod tests {}
