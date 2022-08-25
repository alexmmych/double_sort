# double_sort
This is a short and concise sorting algorithm written in Rust as a learning project.

It works by grabbing numbers from a vector in pairs of two, putting them into a structure called Nodes,
comparing both numbers in it and placing the smallest one to the left and the biggest to the right. 
A binary heap is used to place the Nodes in ascending order, ordering by their leftmost number and then after that
the program loops by checking neighbouring numbers and swapping them if needed. All of this is still on the heap so
the order of the Nodes will still be consistent. The vector is always sorted after n/2 - 1 times (Rounded up for odd amounts)


# Diagram
![double_sort](https://user-images.githubusercontent.com/54857786/184100234-f2c7a726-47ee-4dad-8292-15f830bccf72.svg)
