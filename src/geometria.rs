pub mod formas_geometricas {
    pub struct Triangulo {
        a: i32,
        b: i32,
        c: i32
    }

    impl Triangulo {
        pub fn new(a: i32, b: i32, c: i32) -> Triangulo {
            if ((a + b) <= c) || ((a + c) <= b) || ((b + c) <= a) {
                panic!("Se a soma entre os dois lados é igual ou menor ao terceiro, esse triângulo não pode existir.")
            }
    
            Triangulo { a, b, c }
        }

        pub fn equals(&self, a: i32, b: i32, c: i32) -> bool {
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

        pub fn perimetro(&self) -> i32 {
            self.a + self.b + self.c
        }

        pub fn area(&self) -> f64 {
            // Fórmula de Heron 🧙‍♂️:
            // p = (a + b + c) / 2
            // A = √p(p-a)(p-b)(p-c)

            let p = (self.a + self.b + self.c) / 2;
            
            ((p * (p - self.a) * (p - self.b) * (p - self.c)) as f64).sqrt()
        }
    }
}