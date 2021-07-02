/*
 * author: qiubinglin
 */
#include <vector>
#include <iostream>
#include <algorithm>

template <typename T>
class Heap
{
public:
    Heap(){}
    explicit Heap(const std::vector<T> &arr)
    {
        heapArr = arr;
        buildHeap();
    }
    ~Heap(){}
    int getHeapSize()
    {
        return heapArr.size();
    }
    void insert(T t)
    {
        heapArr.emplace_back(t);
        int i = heapArr.size()/2 - 1;
        while(i >= 0)
        {
            maxHeapify(i);
            i = (i+1)/2 - 1;
        }
    }
    T top()
    {
        if(!heapArr.empty()) return heapArr[0];
        return T();
    }
    void pop()
    {
        if(!heapArr.empty())
        {
            std::swap(heapArr[0], heapArr[heapArr.size()-1]);
            heapArr.pop_back();
            maxHeapify(0);
        }
    }
    std::vector<T> getArr()
    {
        return heapArr;
    }
    void display()
    {
        std::cout << "display: \n";
        for(int i = 0; i < heapArr.size(); ++i)
        {
            std::cout << heapArr[i] << " ";
        }
        std::cout << "\n";
    }

private:
    void maxHeapify(int index)
    {
        int maxIdx = index;
        int l = getLeftChildIndex(maxIdx);
        int r = getRightChildIndex(maxIdx);
        if(l >= 0 && heapArr[l] > heapArr[maxIdx])
        {
            maxIdx = l;
        }
        if(r >= 0 && heapArr[r] > heapArr[maxIdx])
        {
            maxIdx = r;
        }
        
        if(maxIdx != index)
        {
            std::swap(heapArr[maxIdx], heapArr[index]);
            maxHeapify(maxIdx);
        }
    }
    void buildHeap()
    {
        if(heapArr.size() <= 1) return;

        int pos = heapArr.size()/2 - 1;
        for(int i = pos; i >= 0; --i)
        {
            maxHeapify(i);
        }
    }
    int getLeftChildIndex(int parent)
    {
        int res = parent * 2 + 1;
        if(res >= heapArr.size())
        {
            return -1;
        }
        return res;
    }
    int getRightChildIndex(int parent)
    {
        int res = parent * 2 + 2;
        if(res >= heapArr.size())
        {
            return -1;
        }
        return res;
    }

    std::vector<T> heapArr;
};

template<typename T>
void heapSort(std::vector<T> &arr)
{
    Heap<T> hp(arr);
    for(int i = 0; i < arr.size(); ++i)
    {
        arr[i] = hp.top();
        hp.pop();
    }
}

void heapTest()
{
    std::vector<std::vector<int>> arrs = {
        {},
        {0},
        {2},
        {3, 5},
        {5, 3},
        {5, 5},
        {0, 0, 0, 0},
        {1, 1, 1, 1},
        {2, 2, 3, 5},
        {0, 2, 2, 3, 5},
        {2, 5, 3, 0, 2, 3, 0, 3},
        {6, 1, 2, 7, 9, 3, 4, 5, 10, 8},
        {103, 9, 1, 7, 11, 15, 25, 201, 209, 107, 5},
        {-45, -2, -5},
    };

    for(auto &arr : arrs)
    {
        Heap<int> h(arr);
        std::vector<int> tmpArr = h.getArr();
        std::cout << "heap" << std::endl;
        for(int i = 0; i < tmpArr.size(); ++i)
        {
            std::cout << tmpArr[i] << " ";
        }
        std::cout << '\n';

        heapSort(tmpArr);
        std::cout << "sorted" << std::endl;
        for(int i = 0; i < tmpArr.size(); ++i)
        {
            std::cout << tmpArr[i] << " ";
        }
        std::cout << '\n';

        std::cout << "top:" << h.top() << '\n';
        h.pop();
        std::cout << "after pop, top:" << h.top() << '\n';
        h.insert(100);
        std::cout << "after insert, top:" << h.top() << '\n';
    }
}