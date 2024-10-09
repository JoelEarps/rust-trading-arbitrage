## File structure

Files are essentially mods, following the S in SOLID I separated out the key functionality into files and therefore software components. I din't think there was a need for a folder structure as we would have one file per folder which would just over complicate. In the future if logic was able to be grouped into sub categories

## Potential Algorithms Reviewed

1. Bellman Ford - Common in
2. Floyd Warshall - Negative Edges
3. Djikstra - Negative Edges
4. DFS -

## Reasons for using Vecs

Options considered:

1. Binary Tree -
2. Hashmap -
3. Vectors of custom structs

// Actually
The algorithm - you need to check for every possible start point, or allow the user to input them
Go through each one, complete a complete loop all the way back to the start, if at any point the value is higher than the original start price, you save it as an increase, when you get to the end, return the max value and the trades you can do?
