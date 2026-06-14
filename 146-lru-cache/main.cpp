#include <bits/stdc++.h>
using namespace std;

struct Node {
    Node *prev;
    Node *next;
    int key;
    int val;

    Node(int k, int v) : prev(nullptr), next(nullptr), key(k), val(v) {}
};

class LRUCache {
  private:
    int capacity;
    Node *head;
    Node *tail;
    unordered_map<int, Node *> table;

    void removeNode(Node *node) {
        node->prev->next = node->next;
        node->next->prev = node->prev;
    }

    void insertToTail(Node *node) {
        node->prev = tail->prev;
        node->next = tail;
        tail->prev->next = node;
        tail->prev = node;
    }

    void moveToTail(Node *node) {
        removeNode(node);
        insertToTail(node);
    }

  public:
    LRUCache(int capacity) {
        this->capacity = capacity;
        head = new Node(0, 0);
        tail = new Node(0, 0);
        head->next = tail;
        tail->prev = head;
    }

    ~LRUCache() {
        Node *current = head;
        while (current != nullptr) {
            Node *next = current->next;
            delete current;
            current = next;
        }
    }

    int get(int key) {
        auto it = table.find(key);
        if (it == table.end()) {
            return -1;
        }

        Node *node = it->second;
        moveToTail(node);
        return node->val;
    }

    void put(int key, int value) {
        auto it = table.find(key);
        if (it != table.end()) {
            Node *node = it->second;
            node->val = value;
            moveToTail(node);
            return;
        }

        Node *node = new Node(key, value);
        table[key] = node;
        insertToTail(node);

        if ((int)table.size() > capacity) {
            Node *lru = head->next;
            removeNode(lru);
            table.erase(lru->key);
            delete lru;
        }
    }
};

int main() {
    LRUCache c(2);
    c.put(1, 1);
    c.put(2, 2);
    c.get(1);
    c.put(3, 3);
    c.get(2);
    c.put(4, 4);
    c.get(1);
    c.get(3);
    c.get(4);
}
