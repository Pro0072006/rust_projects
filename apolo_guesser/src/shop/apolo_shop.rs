use crate::game::{player::Player, secret_number::SecretNumber};
use crate::util::apolo_util::read_user;

pub struct ApoloShop {
    lifes: u8,
    hints: u8,
}

impl ApoloShop {
    pub fn new() -> Self {
        Self { lifes: 3, hints: 1 }
    }

    pub fn enter_shop(&mut self, player: &mut Player, secret: &mut SecretNumber) {
        let mut user_out = false;
        let mut user_input: String;
        while !user_out {
            println!("!!!!!!Bienvenido a APOLO STORE!!!!!");
            println!(
                "Que vas a comprar:
                1. Vidas | precio: 3 | disponible: {}
                2. Pista | precio: 5 | disponible: {}",
                self.lifes, self.hints
            );

            user_input = read_user();
            match user_input.as_str() {
                "1" => {
                    if !self.buy_life(player) {
                        println!("No tienes el dinero suficiente o no hay vidas disponibles");
                        break;
                    };

                    println!(
                        "Felicidades compraste una vida ahora tienes: {} vida/s | {} monedas",
                        player.lifes(),
                        player.money()
                    )
                }
                "2" => {
                    if !self.buy_hint(player, secret) {
                        println!("No tienes el dinero suficiente o no hay pistas disponibles");
                        break;
                    };

                    println!(
                        "Felicidades compraste una pista, la pista se revelara cuando salgas de la tienda: {} monedas",
                        player.money()
                    )
                }
                _ => println!("Opcion no valida"),
            }

            println!(
                "Quieres seguir en la tienda??
                1. Si
                2. No"
            );
            user_input = read_user();
            match user_input.as_str() {
                "2" => {
                    user_out = true;
                }
                _ => (),
            }
        }
    }

    fn buy_life(&mut self, player: &mut Player) -> bool {
        if self.lifes < 1 {
            return false;
        }
        if !player.try_spend(3) {
            return false;
        }

        self.lifes -= 1;
        player.add_lifes(1);
        true
    }

    fn buy_hint(&mut self, player: &mut Player, secret: &mut SecretNumber) -> bool {
        if self.hints < 1 {
            return false;
        }
        if !player.try_spend(5) {
            return false;
        }

        self.hints -= 1;
        secret.generate_hint();
        true
    }
}
