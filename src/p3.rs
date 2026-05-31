use std::fmt::{self, Formatter};

trait Atacante {
    fn calcular_damage(&self) ->i8;
    fn obtener_nombre(&self) -> &str;
}

struct Guerrero {
    nombre: String,
    fuerza_base: i8,
}

impl  Guerrero{
    fn new(nombre: &str, fuerza_base: i8) -> Self {
        Guerrero{
            nombre: nombre.to_string(),
            fuerza_base,
        }
    }
}

impl Atacante for Guerrero {
    fn calcular_damage(&self) -> i8 {
        self.fuerza_base
    }
    fn obtener_nombre(&self) -> &str {
        &self.nombre
    }
}

impl fmt::Display for Guerrero {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Guerrero [{}] (Fuerza: {})", self.nombre, self.fuerza_base)
    }
}

struct Mago {
    cuerpo: Guerrero,
    mana: i8,
    multiplicador_magico: i8,
}

impl Mago {
    fn new(nombre: &str, fuerza_base: i8, mana: i8, multiplicador_magico: i8) -> Self {
        Mago {
            cuerpo: Guerrero::new(nombre, fuerza_base),
            mana,
            multiplicador_magico,
        }
    }
}

impl Atacante for Mago {
    fn calcular_damage(&self) ->i8 {
        self.cuerpo.calcular_damage() +self.mana * self.multiplicador_magico
    }
    fn obtener_nombre(&self) -> &str {
        self.cuerpo.obtener_nombre()
    }
}

impl fmt::Display for Mago {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Mago avanzado -> {} | Maná: {} | Multiplicador: x {}",
            self.cuerpo, self.mana, self.multiplicador_magico
        )
    }
}

fn main() {
    println!("------------ Simulación de Batalla ------------");
    let guerrero1 = Guerrero::new("Ricardo", 54);
    println!("{}", guerrero1);
    let mag1 = Mago::new("Galdaf", 4,12,2);
    println!("{}", mag1);
    println!("----------------------------------------");
    ejecutar_ataque(&guerrero1);
    ejecutar_ataque(&mag1);
}

fn ejecutar_ataque <T: Atacante>(personaje: &T) {
    println!(
        "¡{} ataca ferozmente causando {} de daño!",
        personaje.obtener_nombre(),
        personaje.calcular_damage(),
    )
}