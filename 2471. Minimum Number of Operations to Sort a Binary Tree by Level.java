class Solution {
    final int SHIFT = 20;
    final int MASK = 0xFFFFF;
    public int minimumOperations(TreeNode root) {
        Queue<TreeNode> queue = new LinkedList();
        queue.add(root);
        int swaps = 0;
        while (!queue.isEmpty()) {
            int levelSize = queue.size();
            long[] nodes = new long[levelSize];
            for (int i = 0; i < levelSize; i++) {
                TreeNode node = queue.poll();
                nodes[i] = ((long) node.val << SHIFT) + i;
                if (node.left != null) queue.add(node.left);
                if (node.right != null) queue.add(node.right);
            }
            Arrays.sort(nodes);
            for (int i = 0; i < levelSize; i++) {
                int origPos = (int) (nodes[i] & MASK);
                if (origPos != i) {
                    long temp = nodes[i];
                    nodes[i--] = nodes[origPos];
                    nodes[origPos] = temp;
                    swaps++;
                }
            }
        }
        return swaps;
    }
}
