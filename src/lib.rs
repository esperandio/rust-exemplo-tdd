pub mod geometria;

#[cfg(test)]
mod tests {
    use crate::geometria::formas_geometricas::{Triangulo, TrianguloTipo};

    #[test]
    fn test_sucesso_criacao_triangulo() {
        let a = 16.0;
        let b = 20.0;
        let c = 30.0;

        let triangulo = Triangulo::new(a, b, c);

        assert_eq!(triangulo.equals(a, b, c), true);
    }

    #[test]
    #[should_panic]
    fn test_erro_criacao_triangulo_quando_desigualdade_triangular() {
        Triangulo::new(20.0, 15.0, 5.0);
    }

    #[test]
    fn test_tipo_triangulo() {
        assert_eq!(Triangulo::new(10.0, 10.0, 10.0).tipo(), TrianguloTipo::EQUILATERO);
        assert_eq!(Triangulo::new(20.0, 20.0, 10.0).tipo(), TrianguloTipo::ISOSCELES);
        assert_eq!(Triangulo::new(30.0, 20.0, 40.0).tipo(), TrianguloTipo::ESCALENO);
    }

    #[test]
    fn test_perimetro_triangulo() {
        assert_eq!(Triangulo::new(19.0, 19.0, 19.0).perimetro(), 57.0);
        assert_eq!(Triangulo::new(20.0, 20.0, 14.0).perimetro(), 54.0);
        assert_eq!(Triangulo::new(12.0, 15.0, 19.0).perimetro(), 46.0);
    }

    #[test]
    fn test_area_triangulo() {
        assert_eq!(Triangulo::new(20.0, 20.0, 20.0).area().round(), 173.0);
        assert_eq!(Triangulo::new(20.0, 20.0, 24.0).area().round(), 192.0);
        assert_eq!(Triangulo::new(7.0, 7.0, 5.0).area().round(), 16.0);
        assert_eq!(Triangulo::new(12.0, 19.0, 9.0).area().round(), 42.0);
    }
}