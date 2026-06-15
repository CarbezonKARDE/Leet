class Solution {
    public ListNode deleteMiddle(ListNode head) {
        if(head.next==null){
            return null;
        }
        ListNode fast=head;
        ListNode slow=new ListNode(-1);
        slow.next=head;
        while(fast!=null && fast.next!=null ){
            fast=fast.next.next;
            slow=slow.next;
        }
        slow.next=slow.next.next;
        return head;
    }
}
