# Extract the Preview JPEGs contained inside .ORF RAW files
# Place copy of exiftools next to script
# Place RAWs inside of RAW folder
# After running script, JPEGs will appear in JPEG folder

import os
from multiprocessing import Pool


def exiftoolcall(command1, command2):
  os.system(command1)  # extracts JPEG from ORF
  os.system(command2)  # copies EXIF data over to extracted JPEG


if __name__ == '__main__':  # required for multiprocessing pool, errors out without it
  if not os.path.exists("RAW"):
    os.mkdir("RAW")
    print("No RAW folder found, one has been made")
    exit()
  if not os.path.exists("JPEG"):
    os.mkdir("JPEG")
  pool = Pool()  # spin up worker processes
  fileslist = os.listdir("RAW")
  for file in fileslist:
    if os.path.isdir("RAW/" + file):  # isdir requires full path from current directory
      continue
    print("Extracting JPEG from", file)
    cmdstring1 = "exiftool.exe -b -PreviewImage RAW/" + file
    cmdstring1 += " > JPEG/" + file[:-3] + "jpg"
    cmdstring2 = "exiftool.exe -overwrite_original -TagsFromFile RAW/" + file
    cmdstring2 += " -exif:all JPEG/" + file[:-3] + "jpg"
    res = pool.apply_async(exiftoolcall, (cmdstring1, cmdstring2))  # send work to process pool
  pool.close()  # required before calling pool.join()
  pool.join()  # wait for all workers to finish before moving on
