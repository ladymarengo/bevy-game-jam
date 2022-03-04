use rand::prelude::SliceRandom;


#[derive(Clone)]
pub enum Advantage {
    Player(PlayerAdvantage),
    Enemy(EnemyAdvantage)
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
    Advantage::Player(PlayerAdvantage::DoubleJump),
    Advantage::Player(PlayerAdvantage::DoubleInitialHp),
    Advantage::Enemy(EnemyAdvantage::DoubleBite),
    Advantage::Enemy(EnemyAdvantage::DoubleSpeed),
];

impl Advantage {
    pub fn random() -> Self {
        ADVANTAGES.choose(&mut rand::thread_rng()).unwrap().clone()
    }
}