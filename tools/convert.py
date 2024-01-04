from PIL import Image
import sys


def image_to_bool_array(image_path):
    try:
        img = Image.open(image_path)
        img = img.convert("L")
        pixel_data = list(img.getdata())
        width, height = img.size
        pixel_2d_array = [
            pixel_data[i : i + width] for i in range(0, width * height, width)
        ]
        bool_2d_array = [[pixel == 255 for pixel in row] for row in pixel_2d_array]
        return bool_2d_array

    except Exception as e:
        print(f"Error: {e}")
        return None


def print_rust_code(bool_2d_array):
    print("let mut pixels = [")

    for row in bool_2d_array:
        print(", ".join([str(cell).lower() for cell in row]), end=",\n")

    print("];")


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python convert.py <image_path>")
        sys.exit(1)

    image_path = sys.argv[1]
    result = image_to_bool_array(image_path)

    if result is not None:
        print_rust_code(result)
