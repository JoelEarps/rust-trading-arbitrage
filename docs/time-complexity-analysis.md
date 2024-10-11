# Time Complexity and Algorithm Analysis

Bellman time complexity is O(VE) and Dijkstra algo has O(Elog(V)) in case of max-heap is used.
Bellman does relaxation for V-1 times and Dijkstra algo only 1 time.
Bellman can handle negative weights but Dijkstra algo can't.
Bellman visits a vertex more than once but Dijkstra algo only once.

So as number of tickers increases, Bellman ford would take a long time, so if input is large you could choose one algorithm that decreases the time spent calculating and return a result factor (i.e. service deprecation), whereas for smaller inputs maybe you could improve QoS and use your performant algorithm.

Algorithms used:

https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.retain
OLog(Capacity)

Time Complexity for Bellman Ford is O(V\*E) - where V is the number of Vertexes and E is the number of Edges
