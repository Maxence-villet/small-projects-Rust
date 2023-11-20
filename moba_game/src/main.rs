#[derive(Debug)]
enum HeroClass {
    Tank,
    Mage,
    _Assassin,
    _Marksman,
    _Support,
}


struct Hero {
    name: String,
    class : HeroClass,
    health: i32,
    attack_damage: i32,
    ability_damage: i32,
}

impl Hero {
    fn attack(&mut self, target: &mut Hero) {
        target.health -= self.attack_damage;
    }

    fn use_ability(&mut self, target: &mut Hero) {
        target.health -= self.ability_damage;
    }
}


fn main() {

    let mut garen = Hero {
        name: String::from("Garen"),
        class: HeroClass::Tank,
        health: 1000,
        attack_damage: 50,
        ability_damage: 100,
    };
    let mut lux =  Hero {
        name: String::from("Lux"),
        class: HeroClass::Mage,
        health: 500,
        attack_damage: 30,
        ability_damage: 150,
    };



    println!("Hero: {}, Class: {:?}, Health: {}, Attack Damage: {}, Ability Damage: {}", garen.name, garen.class, garen.health, garen.attack_damage, garen.ability_damage);
    println!("Hero: {}, Class: {:?}, Health: {}, Attack Damage: {}, Ability Damage: {}", lux.name, lux.class, lux.health, lux.attack_damage, lux.ability_damage);



    println!("Garen attacks Lux:");
    garen.attack(&mut lux);
    println!("lux's health: {}", lux.health);

    println!("Lux uses an ability on garen:");
    lux.use_ability(&mut garen);
    println!("garen's health: {}", garen.health);

}
