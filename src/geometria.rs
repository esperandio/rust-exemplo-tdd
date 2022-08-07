pub mod formas_geometricas {
    pub struct Triangulo {
        a: f64,
        b: f64,
        c: f64
    }

    impl Triangulo {
        pub fn new(a: f64, b: f64, c: f64) -> Triangulo {
            if ((a + b) <= c) || ((a + c) <= b) || ((b + c) <= a) {
                panic!("Se a soma entre os dois lados é igual ou menor ao terceiro, esse triângulo não pode existir.")
            }
    
            Triangulo { a, b, c }
        }

        pub fn equals(&self, a: f64, b: f64, c: f64) -> bool {
            self.a == a && self.b == b && self.c == c
        }

        pub fn escaleno(&self) -> bool {
            self.a != self.b && self.b != self.c
        }

        pub fn isosceles(&self) -> bool {
            (self.a == self.b && self.a != self.c)
            || (self.a == self.c && self.a != self.b)
            || (self.b == self.c && self.b != self.a)
        }

        pub fn equilatero(&self) -> bool {
            self.a == self.b && self.b == self.c
        }

        pub fn perimetro(&self) -> f64 {
            self.a + self.b + self.c
        }

        pub fn area(&self) -> f64 {
            // Fórmula de Heron 🧙‍♂️:
            // p = (a + b + c) / 2
            // A = √p(p-a)(p-b)(p-c)

            let p = (self.a + self.b + self.c) / 2_f64;
            
            (p * (p - self.a) * (p - self.b) * (p - self.c)).sqrt()
        }
    }
}