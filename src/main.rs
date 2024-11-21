struct Monster {
    health: i32,
}

struct Wizard {}
struct Ranger {}

trait FightClose {
    fn attack_with_sword(&self, opponent: &mut Monster) {
        opponent.health -= 10;
        println!(
            "Sword attack! Your opponent has {} health left.",
            opponent.health
        );
    }

    fn attack_with_hand(&self, opponent: &mut Monster) {
        opponent.health -= 2;
        println!(
            "Hand attack! Your opponent has {} health left.",
            opponent.health
        );
    }
}

trait FightFromDistance {
    fn attack_with_bow(&self, opponent: &mut Monster, distance: u32) {
        if distance < 10 {
            opponent.health -= 10;
            println!(
                "Bow attack! Your opponent has {} health left.",
                opponent.health
            );
        }
    }

    fn attack_with_rock(&self, opponent: &mut Monster, distance: u32) {
        if distance < 3 {
            opponent.health -= 4;
            println!(
                "Rock attack! Your opponent has {} health left.",
                opponent.health
            );
        }
    }
}

impl FightClose for Wizard {}
impl FightClose for Ranger {}
impl FightFromDistance for Ranger {}

fn main() {
    let radagst = Wizard {};
    let aragon = Ranger {};
    let mut uruk_hai = Monster { health: 40 };
    radagst.attack_with_sword(&mut uruk_hai);
    aragon.attack_with_bow(&mut uruk_hai, 8);
}
