import os
import cv2


SOURCE_IMAGE_DIR = ".data/images"
GRAYSCALE_IMAGE_DIR = ".data/grayscale"

CATEGORY_LABELS = [
    "not_food",
    "italian_food",
    "japanese_food",
    "meat",
    "seafood",
    "soup",
    "salad",
    "dessert"
]

def main():
    generate_grayscale()

def generate_grayscale():
    for img_dir in CATEGORY_LABELS:
        sample_count = len(os.listdir(os.path.join(SOURCE_IMAGE_DIR, img_dir)))
        print(f"Processing category: {img_dir} ({sample_count} samples)")
        for filename in os.listdir(os.path.join(SOURCE_IMAGE_DIR, img_dir)):
            if not (filename.endswith(".jpg") or filename.endswith(".png")): continue
            
            # Processing
            img = cv2.imread(os.path.join(SOURCE_IMAGE_DIR, img_dir, filename), cv2.IMREAD_GRAYSCALE)
            img_resized = cv2.resize(img, (300, 300), cv2.INTER_AREA)

            grayscale_dir = os.path.join(GRAYSCALE_IMAGE_DIR, f"{img_dir}")
            os.makedirs(grayscale_dir, exist_ok=True)
            cv2.imwrite(os.path.join(grayscale_dir, filename), img_resized)


def train():

    X, y = [], []
    
    for img_dir in CATEGORY_LABELS:
        sample_count = len(os.listdir(os.path.join(GRAYSCALE_IMAGE_DIR, img_dir)))
        print(f"Importing category: {img_dir} ({sample_count} samples)")

        for filename in os.listdir(os.path.join(GRAYSCALE_IMAGE_DIR, img_dir)):
            if not (filename.endswith(".jpg") or filename.endswith(".png")): continue

            img = cv2.imread(os.path.join(GRAYSCALE_IMAGE_DIR, img_dir, filename), cv2.IMREAD_GRAYSCALE)
            X.append(img)
            y.append(CATEGORY_LABELS.index(img_dir))

    print("------------------------------")
    print(f"Data imported. Total samples: {len(X)}")

    pass



if __name__ == "__main__":
    main()
