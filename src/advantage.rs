use rand::prelude::SliceRandom;


#[derive(Clone)]
pub enum Advantage {
    PlayerAdvantage(PlayerAdvantage),
    EnemyAdvantage(EnemyAdvantage)
}

#[derive(Clone)]
pub enum PlayerAdvantage {
    DoubleJump,
    DoubleInitialHp,
}

#[derive(Clone)]
pub enum EnemyAdvantage {
    DoubleBite,
    DoubleSpeed
}

static ADVANTAGES: &[Advantage] = &[
    Advantage::PlayerAdvantage(PlayerAdvantage::DoubleJump),
    Advantage::PlayerAdvantage(PlayerAdvantage::DoubleInitialHp),
    Advantage::EnemyAdvantage(EnemyAdvantage::DoubleBite),
    Advantage::EnemyAdvantage(EnemyAdvantage::DoubleSpeed),
];

impl Advantage {
    pub fn random() -> Self {
        ADVANTAGES.choose(&mut rand::thread_rng()).unwrap().clone()
    }
}