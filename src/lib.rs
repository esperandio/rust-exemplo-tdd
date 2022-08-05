struct Triangulo {
    a: i32,
    b: i32,
    c: i32
}

impl Triangulo {
    fn new(a: i32, b: i32, c: i32) -> Triangulo {
        Triangulo { a, b, c }
    }
}

#[cfg(test)]
mod tests {
    use crate::Triangulo;

    #[test]
    fn test_criar_triangulo() {
        let triangulo = Triangulo::new(16, 20, 30);
        
        assert_eq!(triangulo.a, 16);
        assert_eq!(triangulo.b, 20);
        assert_eq!(triangulo.c, 30);
    }
}