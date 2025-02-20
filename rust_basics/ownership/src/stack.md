# Memory
- Component in a computer to store data and instructions for the processor to execute. 
- Random Access Memory(RAM) is volatile, when power turned off all contents are lost.
- Two types of regions in RAM used by a program at runtime: Stack memory and heap memory. 

--------------------

## Stack
- Last in, first out/
- All data stored on the stack must have a known, fixed size(like integers, float, char, bool, etc)
- Pushing to the stack is faster than allocating on the heap, because the location for new data is always at the top of the stack.
- Types of unkown size will get allocated to the heap and a pointer to the value is pushed to the stack, becuase a pointer is fixed size(usize).