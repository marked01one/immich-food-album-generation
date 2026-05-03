import os, cv2
from torch.utils.data import Dataset

GRAYSCALE_IMAGE_DIR = ".data/grayscale"

class FoodDataset(Dataset):
    def __init__(self, categories: list[str]):
        self.categories = categories
        self.images_labels = []

        for idx, category in enumerate(categories):
            category_dir = os.path.join(GRAYSCALE_IMAGE_DIR, category)
            for filename in os.listdir(category_dir):
                if not (filename.endswith(".jpg") or filename.endswith(".png")): continue
                self.images_labels.append((os.path.join(category_dir, filename), idx))
    
    def __len__(self):
        return len(self.images_labels)
    
    def __getitem__(self, idx):
        image_path, label = self.images_labels[idx]
        image = cv2.imread(image_path, cv2.IMREAD_GRAYSCALE)
        return image, label

    def labels(self):
        return self.categories
        