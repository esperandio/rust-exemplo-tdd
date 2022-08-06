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
}