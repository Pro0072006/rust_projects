const MAX_LIFES: u8 = 10;
const MAX_MONEY: u8 = 20;

pub struct Player {
    name: String,
    lifes: u8,
    money: u8,
}

impl Player {
    pub fn new(name: String) -> Self {
        Self {
            name,
            lifes: 10,
            money: 10,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn money(&self) -> u8 {
        self.money
    }

    pub fn lifes(&self) -> u8 {
        self.lifes
    }

    pub fn add_money(&mut self, amount: u8) {
        self.money = self.money.saturating_add(amount).min(MAX_MONEY);
    }

    pub fn try_spend(&mut self, cost: u8) -> bool {
        if self.money >= cost {
            self.money -= cost;
            true
        } else {
            false
        }
    }

    pub fn add_lifes(&mut self, amount: u8) {
        self.lifes = self.lifes.saturating_add(amount).min(MAX_LIFES);
    }

    pub fn sub_lifes(&mut self, amount: u8) {
        self.lifes = self.lifes.saturating_sub(amount);
    }
}
