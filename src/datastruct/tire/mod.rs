pub mod tire;

#[cfg(test)]
mod test_tire {
    use crate::datastruct::tire::tire::Trie;
    const WORD1: &str = "hello";
    const WORD2: &str = "world";
    const WORD3: &str = "apple";
    const WORD4: &str = "application";
    const PREFIX1: &str = "app";
    #[test] 
    fn test_insert() {
        let mut tire = Trie::new();
        tire.insert_word(WORD1);
        tire.insert_word(WORD2);
        tire.insert_word(WORD3);
        tire.insert_word(WORD4);
        tire.insert_word(PREFIX1);
        tire.insert_word(WORD1);
    }
    #[test]
    fn test_delete() {
        let mut tire = Trie::new();
        tire.insert_word(WORD1);
        assert_eq!(tire.delete_word(WORD1), true);
        assert_eq!(tire, Trie::new());
        tire.insert_word(WORD1);
        tire.insert_word(WORD2);
        tire.insert_word(WORD3);
        tire.insert_word(WORD4);
        tire.insert_word(PREFIX1);
        let tire_standard = tire.clone();
        assert_eq!(tire, tire_standard);
        tire.insert_word(PREFIX1);
        assert_ne!(tire, tire_standard);
        assert_eq!(tire.delete_word(PREFIX1), true);
        assert_eq!(tire, tire_standard);
        tire.insert_word(WORD4);
        assert_ne!(tire, tire_standard);
        assert_eq!(tire.delete_word(WORD4), true);
        assert_eq!(tire, tire_standard);
        assert_eq!(tire.delete_word(PREFIX1), true);
        assert_ne!(tire, tire_standard);
        assert_eq!(tire.delete_word(PREFIX1), false);
    }
    #[test]
    fn test_query() {
        let mut tire = Trie::new();
        assert_eq!(tire.query_word(WORD1), false);
        tire.insert_word(WORD3);
        assert_eq!(tire.query_word(WORD3), true);
        assert_eq!(tire.delete_word(WORD3), true);
        assert_eq!(tire.query_word(WORD3), false);
        tire.insert_word(WORD4);
        assert_eq!(tire.query_word(PREFIX1), false);
    }
    #[test]
    fn test_query_prefix() {
        let mut tire = Trie::new();
        tire.insert_word(WORD1);
        tire.insert_word(WORD2);
        tire.insert_word(WORD3);
        tire.insert_word(WORD4);
        tire.insert_word(PREFIX1);
        assert_eq!(tire.query_prefix(PREFIX1).sort(), vec![WORD4.to_string(), WORD3.to_string(), PREFIX1.to_string()].sort());
    }
}
