## File structure

Files are mods, following the S in SOLID I separated out the key functionality into files and therefore software components. The reason for a folder structure for graph algorithms is so that the arbitrage algorithm can be implemented depending on the algorithm you would want to use e.g. if the number of nodes becomes higher you choose to provide a degradation of service.

## Algorithm Choice

Upon reading the question and challenges presented, it became apparent to me this is a shortest/ weighted route problem (something I looked at before during my PhD for an investigation). Initially there were a few algorithms that came to mind.

1. Djiktra.
2. Bellman Ford.
3. DFS - Depth First Search.

Upon review there was only one clear option for use in this context - Bellman Ford. Bellman can handle negative weights, whereas the others cannot, and this will be the core concept behind detecting arbitrage loops (docs/arbitrage-loop-algorithm-explained.md).

# Data pre processing functionality

There were two potential solutions when looking to implement this algorithm - either use an enum and define all base currencies with an index or use some preprocessing to clean the data set and create usable data from the endpoint. Whilst the enum would have been far easier, this makes the application less scalable as every time a new base currency is added to the endpoint then the application code would also have to be changed. With the current implementation unless the data contract between the endpoint and the arbitrage application changes

## Review of options.

Positives:

1. Makes code more clear.
2. Easier to assign values to the enum and then check.

Negatives:

1. The application cannot scale itself and is none adaptable, the programmer would have to update the enum everytime a new currency would like to be added. This is prone to human error and can be a tedious task.
2. For real world applications - this would make the codebase huge and increase space complexity.
