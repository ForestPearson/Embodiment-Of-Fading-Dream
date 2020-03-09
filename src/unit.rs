use crate::*;

#[derive(Clone)]
pub struct Unit {
    id: i32,           //for texture drawing
    currentframe: i32, //to get image, refer to images[(ID * 4) + currentframe]
    name: String,
    pub x: i32,
    pub y: i32,

    pub player_owned: bool,
    alive: bool,
    counter: bool,

    health: i32,
    maxhealth: i32,
    pub moverange: i32,
    pub attackrange: i32,
    armor: i32,
    maxdamage: i32,
    mindamage: i32,
    basehit: i32,
    //whatever else a unit needs
}

impl Unit {
    //
    //Constructors for various unit types
    //
    #[allow(dead_code)]
    pub fn new(player_owned: bool) -> Self {
        Unit {
            id: 0,
            currentframe: 0,
            name: "Default".to_string(),
            x: 0,
            y: 0,

            player_owned,
            alive: true,
            counter: true,

            maxhealth: 0,
            health: 0,
            moverange: 0,
            attackrange: 0,
            armor: 0,
            maxdamage: 0,
            mindamage: 0,
            basehit: 0,
        }
    }
    #[allow(dead_code)]
    #[allow(clippy::too_many_arguments)] //Very large function, requires the exception if we aren't reducing it.
    pub fn new_custom(
        id: i32,
        name: &str,
        player_owned: bool,
        alive: bool,
        counter: bool,
        maxhealth: i32,
        health: i32,
        moverange: i32,
        attackrange: i32,
        armor: i32,
        maxdamage: i32,
        mindamage: i32,
        basehit: i32,
    ) -> Self {
        Unit {
            id,
            currentframe: 0,
            name: name.to_string(),
            x: 0,
            y: 0,

            player_owned,
            alive,
            counter,

            maxhealth,
            health,
            moverange,
            attackrange,
            armor,
            maxdamage,
            mindamage,
            basehit,
        }
    }

    //
    //Whatever functions units need
    //

    //this function is both a setter and a getter. It checks if a unit is dead,
    //if they aren't it checks if they should be, and sets them as dead accordingly
    pub fn is_alive(&mut self) -> bool {
        if self.alive && self.health > 0 {
            return true;
        }
        if !self.alive {
            return false;
        }
        if self.health <= 0 {
            self.alive = false;
            false
        } else {
            true
        }
    }

    //generates hit damage, returns -1 to indicate attack missing
    pub fn get_damage(&self) -> i32 {
        let mut rnjesus = rand::thread_rng();
        if rnjesus.gen_range(1, 100) <= self.basehit {
            rnjesus.gen_range(self.mindamage, self.maxdamage)
        } else {
            -1
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, images: &[Texture2D]) {
        //&Vec<Texture2D>)
        d.draw_texture(
            &images[((self.id * 4) + self.currentframe) as usize],
            self.x,
            self.y,
            Color::WHITE,
        );
    }

    pub fn ismoused(&self, mouse: Vector2, tile_size: f32, scale: f32) -> bool {
        mouse.x > self.x as f32
            && mouse.y > self.y as f32
            && mouse.x < self.x as f32 + (tile_size * scale)
            && mouse.y < self.y as f32 + (tile_size * scale)
    }
}

//
//Functions related to units that don't make sense as methods
//

//This function handles combat encounters
//
//Unit is the attacking unit, unit2 is the defending unit
//returns 0 for no units killed, 1 for defending unit killed, and 2 for attacking unit killed
pub fn combat(unit: &mut Unit, unit2: &mut Unit, range: i32) -> i32 {
    assert_eq!(unit.is_alive(), true, "Attacking unit isn't alive");
    assert_eq!(unit2.is_alive(), true, "Defending unit isn't alive");

    let attack = unit.get_damage();
    println!("The {} prepares to attack the {}!", unit.name, unit2.name);
    if attack == -1 {
        println!("The attack misses!");
    } else {
        let damage = attack - unit2.armor;
        if damage > 0 {
            unit2.health -= damage;
            println!(
                "The {} hits the {} for {} damage, leaving them with {} health!",
                unit.name, unit2.name, damage, unit2.health
            );
        } else {
            println!(
                "The {}'s attack can't get through the {}'s armor!",
                unit.name, unit2.name
            );
        }
    }
    if unit2.is_alive() {
        //Checks the defending unit is in range and can counter
        if range <= unit2.attackrange && unit2.counter {
            println!("The {} prepares to counter attack!", unit2.name);
            let counter_attack = unit2.get_damage();
            if counter_attack == -1 {
                println!("The counter attack misses!");
                println!("The combat ends.");
                return 0;
            } else {
                let counter_damage = counter_attack - unit.armor;
                if counter_damage > 0 {
                    unit.health -= counter_damage;
                    println!(
                        "The {} counterattacks the {} for {} damage, leaving them with {} health!",
                        unit2.name, unit.name, counter_damage, unit.health
                    );
                } else {
                    println!(
                        "The {}'s attack can't get through the {}'s armor!",
                        unit2.name, unit.name
                    );
                }
            }
            if unit.is_alive() {
                println!("The combat ends.");
                0
            } else {
                println!("The attacking {} is killed!", unit.name);
                2
            }
        } else {
            println!("The defending {} is unable to strike back.", unit2.name);
            println!("The combat ends.");
            0
        }
    } else {
        println!("The defending {} is killed!", unit2.name);
        println!("The combat ends.");
        1
    }
}
pub fn print_possible_combat_results(unit: &mut Unit, unit2: &mut Unit, range: i32) -> bool {
    assert_eq!(unit.is_alive(), true, "Attacking unit isn't alive");
    assert_eq!(unit2.is_alive(), true, "Defending unit isn't alive");

    //Defending unit can't counter attack for some reason
    if !unit2.counter || range > unit2.attackrange {
        let mut attack_max_damage = unit.maxdamage - unit2.armor;
        let mut attack_min_damage = unit.mindamage - unit2.armor;
        //correct negative damage for display
        if attack_max_damage < 0  {
            attack_max_damage = 0;
        }
        if attack_min_damage < 0  {
            attack_min_damage = 0;
        }
        
        let def_max_health = unit2.health - attack_min_damage;
        let def_min_health = unit2.health - attack_max_damage;
        println!("Attacker: {}\nCurrent Health: {}\nPossible Damage: {} - {}\nPossible Remaining Health: {}\n\nDefender: {}\nCurrent Health: {}\nPossible Damage: N/A\nPossible Remaining Health: {} - {}",
        unit.name,
        unit.health,
        attack_min_damage,
        attack_max_damage,
        unit.health,
        unit2.name,
        unit2.health,
        def_min_health,
        def_max_health,
        );
    } else {
        let mut attack_max_damage = unit.maxdamage - unit2.armor;
        let mut attack_min_damage = unit.mindamage - unit2.armor;
        let mut def_max_damage = unit2.maxdamage - unit.armor;
        let mut def_min_damage = unit2.mindamage - unit.armor;
        //correct negative damage for display
        if attack_max_damage < 0  {
            attack_max_damage = 0;
        }
        if attack_min_damage < 0  {
            attack_min_damage = 0;
        }
        if def_max_damage < 0  {
            def_max_damage = 0;
        }
        if def_min_damage < 0  {
            def_min_damage = 0;
        }
        let attack_max_health = unit.health - def_min_damage;
        let attack_min_health = unit.health - def_max_damage;
        let def_max_health = unit2.health - attack_min_damage;
        let def_min_health = unit2.health - attack_max_damage;
        println!("Attacker: {}\nCurrent Health: {}\nPossible Damage: {} - {}\nPossible Remaining Health: {} - {}\n\nDefender: {}\nCurrent Health: {}\nPossible Damage: {} - {}\nPossible Remaining Health: {} - {}\n\n",
        unit.name,
        unit.health,
        attack_min_damage,
        attack_max_damage,
        attack_min_health,
        attack_max_health,
        unit2.name,
        unit2.health,
        def_min_damage,
        def_max_damage,
        def_min_health,
        def_max_health,
        );
    }
    return true;
}
