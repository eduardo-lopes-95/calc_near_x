pub mod calculation {
    /// # Essa função faz uma soma entre dois valores
    /// 
    /// # exemplo
    /// ```rust
    /// use calc_near_x::calculation;
    /// 
    /// assert_eq!(4, calculation::add(2, 2));
    /// ```
    pub fn add(left: u8, right: u8) -> u8 {
        left + right
    }

    /// # Essa função faz uma soma entre dois valores
    /// 
    /// # exemplo
    /// ```rust
    /// use calc_near_x::calculation;
    /// 
    /// assert_eq!(2, calculation::sub(4, 2));
    /// ```
    pub fn sub(left: u8, right: u8) -> u8 {
        if right <= 0 {
            return 0;
        }
        left - right
    }
}

#[cfg(test)]
mod tests {
    use super::calculation;

    #[test]
    fn test_add() {
        let result = calculation::add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_sub() {
        let result = calculation::sub(4, 2);
        assert_eq!(result, 2);
    }
}