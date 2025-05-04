// Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.

// Example 1:
// 
// Input: n = 3
// Output: ["((()))","(()())","(())()","()(())","()()()"]

// Example 2:
// 
// Input: n = 1
// Output: ["()"]

// Constraints:
// 1 <= n <= 8

struct Solution;
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        if !(1..=8).contains(&n) {
            return vec!["".to_string()];
        }
        let mut result = Vec::new();
        let mut current = String::new();

        fn backtrack(n: i32, current: &mut String, open_count: i32, close_count: i32, result: &mut Vec<String>) {
            if current.len() == 2 * n as usize {
                result.push(current.clone())
            }
            if open_count < n {
                current.push('(');
                backtrack(n, current, open_count + 1, close_count, result);
                current.pop();
            }
            if close_count < open_count {
                current.push(')');
                backtrack(n, current, open_count, close_count + 1, result);
                current.pop();
            }
        }
        
        backtrack(n, &mut current, 0, 0, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sort_vector(mut v: Vec<String>) -> Vec<String> {
        v.sort();
        v
    }
    #[test]
    fn test_n_0() {
        let result = Solution::generate_parenthesis(0);
        let expected = vec!["".to_string()];
        assert_eq!(sort_vector(result), sort_vector(expected));
    }
    #[test]
    fn test_n_1() {
        let result = Solution::generate_parenthesis(1);
        let expected = vec!["()".to_string()];
        assert_eq!(sort_vector(result), sort_vector(expected));
    }

    #[test]
    fn test_n_2() {
        let result = Solution::generate_parenthesis(2);
        let expected = vec!["(())". to_string(),
                            "()()".to_string()];
        assert_eq!(sort_vector(result), sort_vector(expected));
    }
    #[test]
    fn test_n_3() {
        let result = Solution::generate_parenthesis(3);
        let expected = vec![
            "((()))".to_string(),
            "(()())".to_string(),
            "(())()".to_string(),
            "()(())".to_string(),
            "()()()".to_string(),
        ];
        assert_eq!(sort_vector(result), sort_vector(expected));
    }
    #[test]
    fn test_n_4_len() {
        let result = Solution::generate_parenthesis(4);
        // Для n=4 должно быть 14 валидных комбинаций
        assert_eq!(result.len(), 14);
    }
}
