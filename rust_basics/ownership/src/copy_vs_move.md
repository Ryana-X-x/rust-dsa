# Copy vs Move

- Scaler values with fixed sizes will automatically get copied in the stack, copying here is cheap
- Dynamically sized data won't get copied, but moved, copying would be too expensive

## Scalar types
- Integers 
- FLoating-point numbers
- Booleans 
- Characters