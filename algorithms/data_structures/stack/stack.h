/*
 * author: qiubinglin
 */
#include <iostream>

template <typename T>
class Stack
{
public:
    explicit Stack(int N = 10)
        :m_limit(N)
    {
        m_stack = new T[m_limit];
    }
    ~Stack()
    {
        delete[] m_stack;
    }
    void push(const T &t)
    {
        if(m_top < m_limit-1)
        {
            m_stack[++m_top] = t;
        }
    }

    void pop()
    {
        if(m_top >= 0)
        {
            --m_top;
        }
    }

    T top()
    {
        return m_stack[m_top];
    }

    bool empty()
    {
        return (m_top > -1);
    }

    size_t size()
    {
        return m_top+1;
    }

private:
    T *m_stack = nullptr;
    int m_limit;
    int m_top = -1;
};

bool stackTest()
{
    Stack<int> st;
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

    return true;
}