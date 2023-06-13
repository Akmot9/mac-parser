# mac-parser
A minimal crate, providing a debug formater and information about the MAC itself. I wrote it, because this code can be shared between two crates of mine.
## no-std
This crate only requires the ```core``` crate(i.e. no allocations).
## Performance
Since the parsing is just a simple copy operation, speeds of up to 1.05G MACs/s were achieved on my Intel 12th Gen i5-1240p.
