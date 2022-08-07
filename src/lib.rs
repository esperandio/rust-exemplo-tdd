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
    fn test_tipo_triangulo() {
        assert_eq!(Triangulo::new(10, 10, 10).equilatero(), true);
        assert_eq!(Triangulo::new(20, 20, 10).isosceles(), true);
        assert_eq!(Triangulo::new(30, 20, 40).escaleno(), true);
    }

    #[test]
    fn test_perimetro_triangulo() {
        assert_eq!(Triangulo::new(19, 19, 19).perimetro(), 57);
        assert_eq!(Triangulo::new(20, 20, 14).perimetro(), 54);
        assert_eq!(Triangulo::new(12, 15, 19).perimetro(), 46);
    }

    #[test]
    fn test_area_triangulo_equilatero() {
        assert_eq!(Triangulo::new(20, 20, 20).area().round(), 173.0);
    }
}