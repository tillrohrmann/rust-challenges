use aoc_common::{GenericResult, GenericError};

#[derive(Debug, PartialEq)]
pub struct TreeNode {
    children: Vec<TreeNode>,
    meta_data: Vec<usize>,
}

impl TreeNode {
    pub fn node(children: Vec<TreeNode>, meta_data: Vec<usize>) -> TreeNode {
        TreeNode {
            children,
            meta_data,
        }
    }

    pub fn leaf(meta_data: Vec<usize>) -> TreeNode {
        TreeNode {
            children: vec![],
            meta_data,
        }
    }

    pub fn parse_tree(input: &Vec<usize>) -> GenericResult<TreeNode> {
        let mut input_iter = input.iter();

        TreeNode::internal_parse(&mut input_iter)
    }

    fn internal_parse<'a, I>(input: &mut I) -> GenericResult<TreeNode> where I: Iterator<Item = &'a usize> {
        if let Some(&num_children) = input.next() {
            if let Some(&num_meta_data) = input.next() {
                let children = (0..num_children).map(|_| TreeNode::internal_parse(input)).collect::<GenericResult<Vec<TreeNode>>>()?;
                let meta_data: Vec<usize> = input.take(num_meta_data).map(|e| e.clone()).collect();

                Ok(TreeNode::node(children, meta_data))
            } else {
                Err(GenericError::new("Invalid meta data specification.").into())
            }
        } else {
            Err(GenericError::new("Invalid input.").into())
        }
    }

    pub fn sum_meta_data(&self) -> usize {
        self.children.iter().map(|child| child.sum_meta_data()).sum::<usize>() + self.meta_data.iter().sum::<usize>()
    }

    pub fn calculate_indexed_value(&self) -> usize {
        if self.children.is_empty() {
            self.meta_data.iter().sum::<usize>()
        } else {
            self.meta_data.iter()
                .map(|&index| self.children
                    .get(index - 1)
                    .map(|child| child.calculate_indexed_value())
                    .unwrap_or(0))
                .sum::<usize>()
        }
    }
}

pub fn read_input_from_file(path: &str) -> GenericResult<Vec<usize>> {
    let content = std::fs::read_to_string(path)?;

    content
        .trim()
        .split_whitespace()
        .map(|number| number
            .parse::<usize>()
            .map_err(|e| e.into()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_parsing() {
        let input = vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2];
        let expected_tree = create_test_tree();
        let tree = TreeNode::parse_tree(&input).unwrap();
        assert_eq!(tree, expected_tree);
    }

    fn create_test_tree() -> TreeNode {
        let expected_tree = TreeNode::node(
            vec![
                TreeNode::leaf(vec![10, 11, 12]),
                TreeNode::node(
                    vec![TreeNode::leaf(vec![99])],
                    vec![2])],
            vec![1, 1, 2]);
        expected_tree
    }

    #[test]
    fn test_meta_data_sum() {
        assert_eq!(create_test_tree().sum_meta_data(), 138);
    }

    #[test]
    fn test_indexed_value() {
        assert_eq!(create_test_tree().calculate_indexed_value(), 66);
    }
}
