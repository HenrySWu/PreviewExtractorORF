# PreviewExtractorORF
Extracts the embedded preview JPEGs from .ORF files.
How to use:
  Download the PreviewExtractorORF.exe.
  Download exiftool.exe from exiftool.org.
  Place both files in the same directory.
  Place RAW .ORF files into a folder named RAW and run the exe.

The Olympus M1 Mark II suffers from a peculiar problem where setting it to record in RAW+JPEG dramatically slows down the write speed.
This problem exists even if two cards are used in tandem, even if the JPEG output is set to both low resolution and low quality.
Shooting only in RAW has dramatically faster write speeds, and therefore much higher sustained burst rates.
Most camera RAW files contain embedded JPEGs, which is what allows for fast previews.
This code was written to extract the 3200*2400 embedded JPEGs from ORF RAWs in a fast manner, leveraging multicore processors.

Python and Go implementations are presented. The exe file is compiled from the Go source code. No flags used, simply "go build PreviewExtractorORF.go".

Python's global interpreter lock does not interfere with using a multicore processor in this case. This is because the ThreadPool can launch multiple instances of exiftool, which run independently. ThreadPool and ProcessPool provide similar performance here.
