pub mod geometria;

#[cfg(test)]
mod tests {
    use crate::geometria::formas_geometricas::Triangulo;

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

    #[test]
    fn test_triangulo_escaleno() {
        let triangulo = Triangulo::new(30, 20, 40);
        assert_eq!(triangulo.escaleno(), true);
    }

    #[test]
    fn test_triangulo_isosceles() {
        let triangulo = Triangulo::new(20, 20, 10);
        assert_eq!(triangulo.isosceles(), true);
    }
}