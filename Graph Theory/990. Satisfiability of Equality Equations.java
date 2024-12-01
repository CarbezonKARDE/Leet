class Solution {
    public boolean equationsPossible(String[] equations) {
        int[] group = new int[26];
        int currentGroup = 1;
        for(int i = 0 ; i < equations.length ; ++i){            
            if(equations[i].charAt(1) == '='){
                int first = equations[i].charAt(0) - 'a';
                int second = equations[i].charAt(3) - 'a';
                if( group[second] != 0 && group[first] != 0 ){                    
                    int min = Math.min(group[second] ,group[first] );
                    int max = Math.max(group[second] ,group[first] );
                    for(int j = 0 ; j < 26;++j){
                        if(group[j] == max){
                            group[j] = min;
                        }
                    }
                }else if(group[first] != 0){
                    group[second] = group[first];
                }else if(group[second] != 0){
                    group[first] = group[second];
                }else{
                    group[first] = currentGroup;
                    group[second]  = currentGroup;
                    currentGroup++;
                }
            }
        }
        for(int i = 0;i < equations.length;++i ){
             if(equations[i].charAt(1) == '='){
                 continue;
             }
            if(equations[i].charAt(0)  == equations[i].charAt(3)){
                return false;
            }
            if( group[equations[i].charAt(0) - 'a'] ==  group[equations[i].charAt(3) - 'a'] &&  group[equations[i].charAt(0) - 'a']  > 0 ){
                return false;
            }
        }
        return true;
    }
}
