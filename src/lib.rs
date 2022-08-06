mod geometria {
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
    }
}
#[cfg(test)]
mod tests {
    use crate::geometria::Triangulo;

    #[test]
    fn test_sucesso_criacao_triangulo() {
        let a = 16;
        let b = 20;
        let c = 30;

        let triangulo = Triangulo::new(a, b, c);

        assert_eq!(triangulo.equals(a, b, c), true);
    }

    #[test]
    #[should_panic]
    fn test_erro_criacao_triangulo_quando_desigualdade_triangular() {
        Triangulo::new(20, 15, 5);
    }
}