#!/usr/bin/env python

# please ensure python means python3 on your system
# the file can be any binary file that contains a JPG image
# note that it's hungry and doesn't chunk the read so careful with large files

# usage: extract-jpg file_name

import sys

file_name = "src/wall.jpg"

def extract_jpg_image():
  jpg_byte_start = b'\xff\xd8'
  jpg_byte_end = b'\xff\xd9'
  jpg_image = bytearray()

  with open(file_name, 'rb') as f:
    req_data = f.read()

    start = req_data.find(jpg_byte_start)

    if start == -1:
      print('Could not find JPG start of image marker!')
      return

    end = req_data.find(jpg_byte_end, start) + len(jpg_byte_end)
    jpg_image += req_data[start:end]
    a = req_data[start:end]
    print(a)

    print(f'Size: {end - start} bytes')

  with open(f'{file_name}-extracted-img', 'wb') as f:
    f.write(jpg_image)

if __name__ == "__main__":
  extract_jpg_image()