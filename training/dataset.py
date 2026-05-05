import os, cv2
import numpy as np
import torch
from torch.utils.data import Dataset

GRAYSCALE_IMAGE_DIR = ".data/grayscale"
RGB_IMAGE_DIR = ".data/rgb_224"

class FoodDataset(Dataset):
    def __init__(self, categories: list[str]):
        self.categories = categories
        self.images_labels = []

        for idx, category in enumerate(categories):
            category_dir = os.path.join(RGB_IMAGE_DIR, category)
            for filename in os.listdir(category_dir):
                if not (filename.endswith(".jpg") or filename.endswith(".png")): continue
                self.images_labels.append((os.path.join(category_dir, filename), idx))
    
    def __len__(self):
        return len(self.images_labels)
    
    def __getitem__(self, idx):
        image_path, label = self.images_labels[idx]
        # Reverse the dimensions to (C, H, W) for PyTorch and normalize pixel values to [0, 1]
        image = cv2.imread(image_path, cv2.IMREAD_COLOR_RGB)
        image_array = np.array(image.transpose(2, 0, 1) / 255.0, dtype=np.float32)
        
        # Convert numpy array to PyTorch tensor with float32 dtype
        image_tensor = torch.from_numpy(image_array)
        
        return image_tensor, label

    def labels(self):
        return self.categories
        
