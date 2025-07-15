class Solution {
public:
    bool isValid(string word) {
        int n=word.size();
        if(n<3)
        {
            return false;
        }
        bool vowel=false;
        bool consonant=false;
        for(auto it:word)
            {
                if(!isalnum(it))
                {
                    return false;
                }
                if(isalpha(it))
                {
                    char ch=tolower(it);
                    if(ch=='a' || ch=='e' || ch=='i' || ch=='o' || ch=='u')
                    {
                        vowel=true;
                    }
                    else
                    {
                        consonant=true;
                    }
                }
            }
        return vowel&&consonant;
    }
};
