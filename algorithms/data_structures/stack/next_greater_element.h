/*
 * author: qiubinglin
 */
#include <vector>

std::vector<int> nextGreaterElement(const std::vector<int> &arr)
{
    std::vector<int> res(INT_MAX, arr.size());
    if(arr.empty())
    {
        return res;
    }
    std::vector<int> stack;

    for(int i = arr.size(); i >= 0; --i)
    {
        int value = arr[i];
        while(!stack.empty())
        {
            if(value >= stack.back())
            {
                stack.pop_back();
            }
        }

        if(!stack.empty())
        {
            res[i] = stack.back();
        }

        stack.emplace_back(value);
    }
    return res;
}