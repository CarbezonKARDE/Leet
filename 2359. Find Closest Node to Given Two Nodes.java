class Solution {
    static final int VISITED1 = 1;
    static final int VISITED2 = 2;
    static final int VISITED_BOTH = VISITED1 | VISITED2;
    public int closestMeetingNode(int[] edges, int node1, int node2) {
        int n = edges.length;
        byte[] visited = new byte[n];
        int result = Integer.MAX_VALUE;
        while (result == Integer.MAX_VALUE && node1 >= 0 && node2 >= 0) {
            if ((visited[node1] & VISITED1) != 0) 
                node1 = -1;
            else if ((visited[node1] |= VISITED1) == VISITED_BOTH) 
                result = node1;
            else
                node1 = edges[node1];
            if ((visited[node2] & VISITED2) != 0) {
                node2 = -1;
                break;
            }
            if ((visited[node2] |= VISITED2) == VISITED_BOTH) {
                result = Math.min(result, node2);
                break;
            }
            node2 = edges[node2];
        }
        if (result != Integer.MAX_VALUE)  return result;
        if (node1 >= 0) {
            while (node1 >= 0) {
                if ((visited[node1] & VISITED1) != 0)  return -1;
                if ((visited[node1] |= VISITED1) == VISITED_BOTH)  return node1;
                node1 = edges[node1];
            }
            return -1;
        }
        while (node2 >= 0) {
            if ((visited[node2] & VISITED2) != 0)  return -1;
            if ((visited[node2] |= VISITED2) == VISITED_BOTH)  return node2;
            node2 = edges[node2];
        }
        return -1;
    }
}
