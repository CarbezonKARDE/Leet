class Solution {
public:
    vector<int> diffWaysToCompute(string exp) {
        vector<int>ans;
        int n=exp.size();
        for(int i=0;i<n;i++){
            char op=exp[i];
            if(op=='+'|| op=='-'|| op=='*'){
                vector<int>s1= diffWaysToCompute(exp.substr(0,i));
                vector<int>s2= diffWaysToCompute(exp.substr(i+1));
                for(int a:s1){
                    for (int b:s2){
                        if(op=='+') ans.push_back(a+b);
                        else if(op=='-') ans.push_back(a-b);
                        else if(op=='*') ans.push_back(a*b);
                    }
                }
            }
        }
        if(ans.empty()) ans.push_back(stoi(exp));
        return ans;
    }
};
