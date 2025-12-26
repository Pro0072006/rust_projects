use std::io::{self, Write};

use crate::{
    game::{difficulty::Difficulty, player::Player, secret_number::SecretNumber},
    shop::apolo_shop::ApoloShop,
    util::apolo_util::{lucky_bonus, read_user},
    util::sleep::sleep_ms,
};

pub fn start_game() {
    let mut user_input: String;
    let mut player: Player = Player::new();
    let mut apolo_shop = ApoloShop::new();
    let mut secret_number: SecretNumber;

    println!(
        "\n¿Qué dificultad quieres elegir?
        [1] Fácil
        [2] Normal
        [3] Difícil",
    );

    user_input = read_user();
    secret_number = SecretNumber::new(match user_input.as_str() {
        "1" => Difficulty::Easy,
        "2" => Difficulty::Normal,
        "3" => Difficulty::Hard,
        _ => {
            println!("Opción inválida");
            return;
        }
    });

    loop {
        println!("--------------------------");
        if let Some(apolo_hint) = secret_number.hint() {
            println!("{}", apolo_hint);
        }

        println!(
            "\nVidas disponibles: {}\
             \nMonedas disponibles: {}\
             \n\nPresiona 's' para entrar a la tienda",
            player.lifes(),
            player.money()
        );

        print!("Elige un numero: ");
        io::stdout().flush().expect("WTF");
        user_input = read_user();

        if user_input.as_str() == "s" {
            println!("Vas a entrar a la tienda...");
            sleep_ms(1000);
            apolo_shop.enter_shop(&mut player, &mut secret_number);
            continue;
        }

        if user_input.parse::<u8>().unwrap() == secret_number.value() {
            println!("GANASTE");
            //logica para ganar
            return;
        }

        println!("\nNumero incorrecto, pierdes una vida");
        sleep_ms(800);

        lucky_bonus(&mut player);
        player.sub_lifes(1);

        if player.lifes() < 1 {
            println!("PERDISTE");
            //logica para perder
            return;
        }
    }
}
