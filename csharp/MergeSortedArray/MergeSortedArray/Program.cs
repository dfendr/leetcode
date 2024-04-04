// See https://aka.ms/new-console-template for more information


var sol = new Solution();
var index1 = new int[] {1, 2, 3, 0, 0, 0};
sol.Merge(index1, 3, new int[] {2, 5, 6}, 3);
foreach (var n in index1)
{
    Console.WriteLine(n);
}


public class Solution
{
    public void Merge(int[] nums1, int m, int[] nums2, int n)
    {

        // Early exit if nums2 is empty
        if (n == 0)
            return;

        // if nums1 is "empty", copy all entries from nums2
        if (m == 0)
        {
            for (int i = 0; i < n; i++)
                nums1[i] = nums2[i];
            return;
        }

        // set indices to track position in each list
        int index = 0;
        int index2 = 0;

        while (index < n + m)
        {

            // at each position in nums1, check if nums1[i] or nums2[i2] smaller
            // if n1 larger, move nums2 over to spot, shifting n1 numbers over
            if (nums1[index] >= nums2[index2])
            {
                // Shift values right
                for (int i = n + m - 1; i > index; i--)
                {
                    // i starts at max index
                    // move i - 1 to i, overwriting the one before it
                    // this will leave a space at the index where nums2[index2] will be inserted
                    nums1[i] = nums1[i - 1];
                }
                nums1[index] = nums2[index2];
                index2++;
            }

            else // nums2[index2] larger
            {
                if (nums1[index] == 0)
                {
                    nums1[index] = nums2[index2];
                    index2++;
                }
                index++;
            }
        }
    }
}
