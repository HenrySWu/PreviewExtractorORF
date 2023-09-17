# PreviewExtractorORF
Extracts the embedded preview JPEGs from .ORF files.

## How To Use
Download the PreviewExtractorORF.exe.<br />
Download exiftool.exe from exiftool.org.<br />
Place both files in the same directory.<br />
Place .ORF files into a folder named RAW.<br />
Run the exe.

## Why
The Olympus M1 Mark II suffers from a peculiar problem where setting it to record in RAW+JPEG dramatically slows down the write speed. This problem exists even if two cards are used in tandem, even if the JPEG output is set to both low resolution and low quality. Shooting only in RAW has dramatically faster write speeds, and therefore much higher sustained burst rates. Most camera RAW files contain embedded JPEGs, which is what allows for fast previews. This code was written to extract the 3200*2400 embedded JPEGs from ORF RAWs in a fast manner, leveraging multicore processors.

## Files
Python and Go implementations are presented. The exe file is compiled from the Go source code. No flags used.
>go build PreviewExtractorORF.go

Multiprocess is typically used to allow for parallel computing despite Python's global interpreter lock. In this case, async/await or ThreadPool can run this operation in parallel because the multiple exiftool instances that run independently are doing the heavy lifting. ProcessPool code is used here to demonstrate how to typically utilize a multicore processor using Python.

Go's coroutine system works innately for parallelism.

Rust files are the Cargo.toml and the files in the src folder. The main.rs file uses tokio::spawn, which spawns multiple independent threads to execute tasks. The use_spawn_blocking.rs file uses tokio::tasks:spawn_blocking to utilize non async functions.
