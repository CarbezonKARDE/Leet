class Solution {
public:
    vector<ListNode*> splitListToParts(ListNode* head, int k) {
        vector<ListNode*> result(k, nullptr);
        int len = 0;
        ListNode* current = head;
        while (current != nullptr) {
            len++;
            current = current->next;
        }
        int size = len / k;
        int extra = len % k;
        current = head;
        for (int i = 0; i < k; i++) {
            if (current == nullptr) {
                result[i] = nullptr;
            } else {
                result[i] = current;
                int partSize = size + (extra > 0 ? 1 : 0);
                extra--;
                                for (int j = 1; j < partSize; j++) {
                    if (current != nullptr) {
                        current = current->next;
                    }
                }
                if (current != nullptr) {
                    ListNode* nextPart = current->next;
                    current->next = nullptr;
                    current = nextPart;
                }
            }
        }
        return result;
    }
};
