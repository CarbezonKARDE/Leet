class Solution {
public:
    bool hasSameDigits(string s) {
       while(s.size()>2){
            int idx=0;
            string newS=s;
            while(idx<s.size()-1){
                char n=(s[idx]-'0'+s[idx+1]-'0')%10;
                newS[idx]=n+'0';
                idx++;
            }
            newS.pop_back();
            s=newS;
        }
        return (s[0]==s[1])?true:false;
    }
};
