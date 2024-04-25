pub mod calc {

    /// # Essa função faz a soma dos valores e adiciona 1
    /// # exemplo
    /// ``` rust
    /// use calc_near_x::calc;
    /// 
    /// assert_eq!(42, calc::sum_plus_one(41,0));
    /// assert_eq!(4, calc::sum_plus_one(1,2));
    /// assert_eq!(1, calc::sum_plus_one(0,0));
    /// ```
    pub fn sum_plus_one(x: u8, y: u8) -> u8 {
        x + y + 1
    }

    /// # Essa função faz a subtracao dos valores e subtrai 1
    /// 
    /// - Se o primeiro parâmetro for menor ou igual ao segundo, vai retornar 0
    ///
    /// # exemplo
    /// ``` rust
    /// use calc_near_x::calc;
    /// 
    /// assert_eq!(40, calc::sub_less_one(41,0));
    /// assert_eq!(0, calc::sub_less_one(6,6));
    /// assert_eq!(0, calc::sub_less_one(5,50));
    /// ```
    pub fn sub_less_one(x: u8, y: u8) -> u8 {
        if x <= y {
            return 0;
        }

        x - y - 1
    }
}

#[cfg(test)]
    mod test {

        use super::calc;

        #[test]
        fn test_sum(){
            let result = calc::sum_plus_one(5, 6);
            let expected = 12;
            assert_eq!(result, expected);
        }

        #[test]
        fn test_sub_failed(){
            let result = calc::sub_less_one(5, 6);
            let expected = 0;
            assert_eq!(result, expected);
        }

        #[test]
        fn test_sub_(){
            let result = calc::sub_less_one(6, 6);
            let expected = 0;
            assert_eq!(result, expected);
        }
    }