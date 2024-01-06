from PIL import Image
import sys

if len(sys.argv) != 2:
    print("Usage: python convert.py <image_path>")
    sys.exit(1)

image_path = sys.argv[1]

img = Image.open(image_path)
img_gray = img.convert("L")
pixels = []

width, height = img_gray.size
for y in range(0, height):
    for x in range(0, width):
        pixel = img_gray.getpixel((x, y))
        pixels.append(1 if pixel > 128 else 0)

while len(pixels) % 8 != 0:
    pixels.append(0)

bin_pixels = []

for i in range(0, len(pixels), 8):
    b = "".join(map(str, pixels[i : i + 8]))
    bin_pixels.append(int(b[::-1], 2))
print("let pixels = {};".format(str(bin_pixels)))
