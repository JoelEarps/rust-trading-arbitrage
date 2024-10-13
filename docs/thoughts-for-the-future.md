_Note:_ There will be a second branch working on some of these ideas.

1. Configurable algorithm to be used based on number of base currencies - i.e. one that doesn't search through every just through the first positive e.g. Djikstras algorithm, searches for best. As number of tickers increases, Bellman ford could potentially start to take a long time, so if input is large you could choose one algorithm that decreases the time spent calculating and return a result factor (i.e. service deprecation), whereas for smaller inputs maybe you could improve QoS and use your more accurate/ full algorithm.
2. Live Updates, multi threaded.
3. Integration Tests - provide configurable API call address for testing.
