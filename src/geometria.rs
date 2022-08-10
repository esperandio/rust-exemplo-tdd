pub mod formas_geometricas {
    pub struct Triangulo {
        a: f64,
        b: f64,
        c: f64,
        tipo: TrianguloTipo
    }

    #[derive(PartialEq)]
    #[derive(Debug)]
    #[derive(Clone, Copy)]
    pub enum TrianguloTipo {
        ESCALENO,
        ISOSCELES,
        EQUILATERO
    }

    impl Triangulo {
        pub fn new(a: f64, b: f64, c: f64) -> Triangulo {
            if ((a + b) <= c) || ((a + c) <= b) || ((b + c) <= a) {
                panic!("Se a soma entre os dois lados é igual ou menor ao terceiro, esse triângulo não pode existir.")
            }

            let tipo = Triangulo::get_triangulo_tipo(a, b, c);
    
            Triangulo { a, b, c, tipo}
        }

        pub fn equals(&self, a: f64, b: f64, c: f64) -> bool {
            self.a == a && self.b == b && self.c == c
        }

        fn get_triangulo_tipo(a: f64, b: f64, c: f64) -> TrianguloTipo {
            if Triangulo::escaleno(a, b, c) {
                return TrianguloTipo::ESCALENO
            } else if Triangulo::isosceles(a, b, c) {
                return TrianguloTipo::ISOSCELES
            } else if Triangulo::equilatero(a, b, c) {
                return TrianguloTipo::EQUILATERO;
            }

            panic!("Tipo inválido");
        }

        fn escaleno(a: f64, b: f64, c: f64) -> bool {
            a != b && b != c
        }

        fn isosceles(a: f64, b: f64, c: f64) -> bool {
            (a == b && a != c)
            || (a == c && a != b)
            || (b == c && b != a)
        }

        fn equilatero(a: f64, b: f64, c: f64) -> bool {
            a == b && b == c
        }

        pub fn tipo(&self) -> TrianguloTipo {
            self.tipo
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