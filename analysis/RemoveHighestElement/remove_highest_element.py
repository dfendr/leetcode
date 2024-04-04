def removeHighest(arr:list, n):
    return_arr = []
    if len(arr) == 0:
        return return_arr
    highest = arr[n-1]
    for i, j in enumerate(arr):
        if j != highest:
            return_arr.insert(i, -1)
    return return_arr

print(removeHighest([1,1,2,3,3], 5))


print(removeHighest([1,1,1,1,1], 5))
