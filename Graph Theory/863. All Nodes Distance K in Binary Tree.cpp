class Solution {
public:
    vector<int> distanceK(TreeNode* root, TreeNode* target, int k) {
        vector<int> res;
        unordered_map<int,int> bfs_tree;
        queue<pair<TreeNode*,int> > q;
        int cur_position = 1;
        int find_position = -1;
        int find_value = target->val;
        if(root == NULL){return vector<int>();}
        q.push(make_pair(root,1));
        while(!q.empty()){
            TreeNode * cur_node = q.front().first;
            int father_position = q.front().second;
            if(cur_node->val == find_value){find_position = father_position;}
            q.pop();
            bfs_tree.insert(make_pair(cur_node->val, father_position));
            if(cur_node->left != NULL) q.push(make_pair(cur_node->left, father_position*2));
            if(cur_node->right != NULL) q.push(make_pair(cur_node->right, father_position*2+1));
        }
        unordered_map<int,int>::iterator it;
        for(it = bfs_tree.begin() ; it != bfs_tree.end() ; it++){
            int this_position = it->second;
            int tmp_find_position = find_position;
            int distance = 0;
            while(this_position != tmp_find_position){
                if(this_position > tmp_find_position){
                    this_position = this_position/2;
                    distance += 1;
                }
                else{
                    tmp_find_position = tmp_find_position/2;
                    distance += 1;
                }
            }
            if(distance == k){
                res.push_back(it->first);
            }
        }
        return res;
    }
};
