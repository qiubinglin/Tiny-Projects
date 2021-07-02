/*
 * author: qiubinglin
 */
template <typename T>
struct Node
{
    ~Node()
    {
        if(next)
        {
            delete next;
        }
    }
    T value;
    Node<T> *next = nullptr;
};

template <typename T>
class LinkedStack
{
public:
    explicit LinkedStack(){}
    ~LinkedStack()
    {
        delete stTop;
    }
    void push(const T &t)
    {
        Node<T> *newTop = new Node<T>;
        newTop->value = t;
        newTop->next = stTop;
        stTop = newTop;
        ++stSize;
    }
    void pop()
    {
        Node<T> *tmp = stTop;
        stTop = stTop->next;
        --stSize;
        tmp->next = nullptr;
        delete tmp;
    }
    T top()
    {
        return stTop->value;
    }
    int size()
    {
        return stSize;
    }
    bool empty()
    {
        return (bool)stTop;
    }
private:
    Node<T> *stTop = nullptr;
    int stSize = 0;
};

#include <iostream>
void linkedStackTest()
{
    LinkedStack<int> st;
    std::cout << "stack is empty: " << st.empty() << std::endl;
    for(int i = 0; i < 10; ++i)
    {
        st.push(i);
    }
    std::cout << "size is 10: " << st.size() << std::endl;
    std::cout << "top is 9: " << st.top() << std::endl;
    st.pop();
    st.pop();
    std::cout << "after pop twice, top is 7: " << st.top() << std::endl;
    st.push(11);
    std::cout << "after push, top is 11: " << st.top() << std::endl;
}