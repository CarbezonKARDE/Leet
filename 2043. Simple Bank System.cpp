class Bank {
public:
vector<long long> bal;
  bool isValidAc(int account){
     return account>=1 && account<=bal.size();
  }
    Bank(vector<long long>& balance) {
       bal=balance;
    }
    bool transfer(int account1, int account2, long long money) {
        if(isValidAc(account1) && isValidAc(account2)){
            if(bal[account1-1]>=money){
                bal[account2-1]=bal[account2-1]+money;
                bal[account1-1]=bal[account1-1]-money;
                 return true;
            }
           
        }
        return false;
    }
    bool deposit(int account, long long money) {
        if(isValidAc(account)){
            bal[account-1]+=money;
            return true;
        }
        return false;
    }
    bool withdraw(int account, long long money) {
          if(isValidAc(account) && bal[account-1]>=money){
            bal[account-1]-=money;
            return true;
    }
    return false;
    }
};
