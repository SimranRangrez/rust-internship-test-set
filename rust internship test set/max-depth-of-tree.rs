// Assuming the binary tree is represented by the following struct
#[derive(Debug)]
struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

fn max_depth(root: &Option<Box<TreeNode>>) -> i32 {
    match root {
        Some(node) => {
            let left_depth = max_depth(&node.left);
            let right_depth = max_depth(&node.right);
            1 + left_depth.max(right_depth)
        }
        None => 0,
    }
}

fn main() {
    // Example binary tree
    let tree = Some(Box::new(TreeNode {
        value: 1,
        left: Some(Box::new(TreeNode {
            value: 2,
            left: None,
            right: None,
        })),
        right: Some(Box::new(TreeNode {
            value: 3,
            left: None,
            right: Some(Box::new(TreeNode {
                value: 4,
                left: None,
                right: None,
            })),
        })),
    }));

    let depth = max_depth(&tree);
    println!("The maximum depth of the binary tree is: {}", depth);
}