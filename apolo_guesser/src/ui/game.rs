use std::io::{self, Write};

use crate::{
    game::{difficulty::Difficulty, player::Player, secret_number::SecretNumber},
    shop::apolo_shop::ApoloShop,
    util::apolo_util::read_user,
};

pub fn start_game() {
    let mut user_input: String;
    let mut player: Player;
    let mut secret_number: SecretNumber;
    let mut apolo_shop = ApoloShop::new();

    println!("Cual es tu nombre??");
    user_input = read_user();
    player = Player::new(user_input);

    println!(
        "\n{} ¿Qué dificultad quieres elegir?
        \n[1] Fácil
        \n[2] Normal
        \n[3] Difícil",
        player.name()
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
        if let Some(apolo_hint) = secret_number.hint() {
            println!("{}", apolo_hint);
        }

        println!(
            "\nVidas disponibles: {}\
             \nMonedas disponibles: {}",
            player.lifes(),
            player.money()
        );

        print!("Elige un numero: ");
        io::stdout().flush().expect("WTF");
        user_input = read_user();

        if user_input.parse::<u8>().unwrap() == secret_number.value() {
            println!("GANASTE");
            //logica para ganar
            return;
        }

        player.sub_lifes(1);

        if player.lifes() < 1 {
            println!("PERDISTE");
            //logica para perder
            return;
        }

        if player.lifes() == 6 {
            apolo_shop.enter_shop(&mut player, &mut secret_number);
        }
    }
}
