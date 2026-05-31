use std::fmt;

trait Calculable {
    fn calcular_pago(&self) -> f32;
    fn obtener_nombre(&self) -> &str;
}

struct Empleado{
    nombre: String,
    salario_base: f32,
}

impl Empleado {
    fn new(nombre: &str, salario_base: f32) -> Self{
        Empleado {
            nombre: nombre.to_string(),
            salario_base,
        }
    }
}

impl Calculable for Empleado {
    fn calcular_pago(&self) ->f32 {
        self.salario_base
    }
    fn obtener_nombre(&self) ->&str {
        &self.nombre
    }
}

impl fmt::Display for Empleado{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Empleado: {} | y con salario base: {}", self.nombre, self.salario_base)
    }
}

struct Gerente {
    empleado: Empleado,
    bono: f32,
    comision: f32,
}

impl Gerente {
    fn new(nombre: &str, salario_base: f32, bono: f32, comision: f32) -> Self {
        Gerente {
            empleado: Empleado::new(nombre, salario_base),
            bono,
            comision,
        }
    }
}

impl Calculable for Gerente {
    fn calcular_pago(&self) -> f32 {
        self.empleado.calcular_pago() + self.bono + self.comision
    }
    fn obtener_nombre(&self) -> &str {
        self.empleado.obtener_nombre()
    }
}

impl fmt::Display for Gerente {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} | Bono: {} | Comisión: {}", self.empleado, self.bono, self.comision)
    }
}

fn main() {
    println!("-------- Lista de Personal --------");
    let empl1 = Empleado::new("Ricci", 2000.0);
    println!("{}", empl1);
    let gerente1 = Gerente::new("Aani", 2000.0, 500.0, 400.0);
    println!("{}", gerente1);
    println!("Pago Total de {}: ${}", gerente1.obtener_nombre(), gerente1.calcular_pago())
}