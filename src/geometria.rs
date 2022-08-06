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
            self.a == self.b && self.b == self.c
        }
    }
}