    class MyCircularDeque {
    private class Node {
        int val;
        Node prev, next;
        Node(int value) {
            this.val = value;
        }
    }
    private int count;
    private int n;
    private Node front;
    private Node rear;
    public MyCircularDeque(int k) {
        this.count = 0;
        this.n = k;
        this.front = null;
        this.rear = null;
    }
    public boolean insertFront(int value) {
        if (isFull()) {
            return false;
        }
        Node newNode = new Node(value);
        if (front == null && rear == null) {
            front = rear = newNode;
        } else {
            newNode.next = front;
            front.prev = newNode;
            front = newNode;
        }
        count++;
        return true;
    }
    public boolean insertLast(int value) {
        if (isFull()) {
            return false;
        }
        Node newNode = new Node(value);
        if (front == null && rear == null) {
            front = rear = newNode;
        } else {
            rear.next = newNode;
            newNode.prev = rear;
            rear = newNode;
        }
        count++;
        return true;
    }
    public boolean deleteFront() {
        if (isEmpty()) {
            return false;
        }
        if (front == rear) {
            front = rear = null;
        } else {
            front = front.next;
            front.prev = null;
        }
        count--;
        return true;
    }
    public boolean deleteLast() {
        if (isEmpty()) {
            return false;
        }
        if (front == rear) {
            front = rear = null;
        } else {
            rear = rear.prev;
            rear.next = null;
        }
        count--;
        return true;
    }
    public int getFront() {
        if (isEmpty()) {
            return -1;
        }
        return front.val;
    }
    public int getRear() {
        if (isEmpty()) {
            return -1;
        }
        return rear.val;
    }
    public boolean isEmpty() {
        return count == 0;
    }
    public boolean isFull() {
        return count == n;
    }
}
