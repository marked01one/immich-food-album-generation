import cv2
import numpy as np

def resize_and_pad(image: cv2.Mat, target_size: tuple) -> cv2.Mat:
    height, width = image.shape[:2]
    center = np.array(image.shape[:2]) / 2
    
    # Get smallest side to determine the crop size
    crop = min(height, width)
    x = center[1] - crop / 2
    y = center[0] - crop / 2

    img_cropped = image[int(y):int(y+crop), int(x):int(x+crop)]

    # Crop the image to a square
    img_cropped = cv2.resize(img_cropped, target_size, interpolation=cv2.INTER_AREA)

    return img_cropped
