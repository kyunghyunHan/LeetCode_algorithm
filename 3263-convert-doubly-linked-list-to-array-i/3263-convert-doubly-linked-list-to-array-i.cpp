/**
 * Definition for doubly-linked list.
 * class Node {
 *     int val;
 *     Node* prev;
 *     Node* next;
 *     Node() : val(0), next(nullptr), prev(nullptr) {}
 *     Node(int x) : val(x), next(nullptr), prev(nullptr) {}
 *     Node(int x, Node *prev, Node *next) : val(x), next(next), prev(prev) {}
 * };
 */
class Solution {
public:
vector<int> toArray(Node *head){
        Node *tmp = head;
        vector<int> ret;
        while(tmp != NULL) {
            ret.push_back(tmp->val);
            tmp = tmp->next;

            if(tmp == head) break;
        }
        return ret;
    }
};