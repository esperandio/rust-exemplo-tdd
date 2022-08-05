struct Triangulo {
    a: i32,
    b: i32,
    c: i32
}

impl Triangulo {
    fn new(a: i32, b: i32, c: i32) -> Triangulo {
        if ((a + b) <= c) || ((a + c) <= b) || ((b + c) <= a) {
            panic!("Se a soma entre os dois lados é igual ou menor ao terceiro, esse triângulo não pode existir.")
        }

        Triangulo { a, b, c }
    }
}

#[cfg(test)]
mod tests {
    use crate::Triangulo;

    #[test]
    fn test_sucesso_criacao_triangulo() {
        let triangulo = Triangulo::new(16, 20, 30);
        
        assert_eq!(triangulo.a, 16);
        assert_eq!(triangulo.b, 20);
        assert_eq!(triangulo.c, 30);
    }

    #[test]
    #[should_panic]
    fn test_erro_criacao_triangulo_quando_desigualdade_triangular() {
        Triangulo::new(20, 15, 5);
    }
}