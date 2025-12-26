use std::io;

use crate::util::sleep::sleep_ms;
use rand::Rng;

use crate::game::player::Player;
pub fn read_user() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("???????????");
    buffer.trim().to_lowercase()
}

pub fn lucky_bonus(player: &mut Player) {
    let mut rng = rand::thread_rng();

    let gen_number = rng.gen_range(player.luck()..100);

    match gen_number {
        90.. => {
            player.add_money(20);
            player.reset_lucky();
            println!("Felicidades caiste en el megabonusss, ganaste 20 moneditas mas")
        }
        75.. => {
            player.add_money(10);
            player.reset_lucky();
            println!("Felicidades caiste en el bonus, ganaste 10 moneditas mas")
        }
        _ => {
            player.add_lucky(10);
            println!("No caiste en el bonus sin embargo tu suerte aumento un 10%")
        }
    }

    sleep_ms(2000);
}
