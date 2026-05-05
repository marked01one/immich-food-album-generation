import os, cv2, time, requests
import numpy as np
import torch
import numpy as np
import torch.nn as nn
import torch.optim as optim
from torch.utils.data import DataLoader, TensorDataset
from cnn import ResNet, Bottleneck

from dataset import FoodDataset, GRAYSCALE_IMAGE_DIR

SOURCE_IMAGE_DIR = ".data/rgb_224/"

CATEGORY_LABELS = [
    "not_food",
    "italian_food",
    "japanese_food",
    "fast_food",
    "meat",
    "seafood",
    "soup",
    "salad",
    "dessert",
    "rice",
    "eggs",
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


def print_log(message):
    print(f"[{time.strftime('%Y-%m-%d %H:%M:%S')}] {message}")

def train():

    dataset = FoodDataset(CATEGORY_LABELS)
    dataloader = DataLoader(dataset, batch_size=32, shuffle=True)

    restnet50 = ResNet(Bottleneck, [3,4,6,3], num_classes=len(CATEGORY_LABELS), num_channels=3)
    criterion = nn.CrossEntropyLoss()
    optimizer = optim.SGD(restnet50.parameters(), lr=0.001, momentum=0.9)

    print_log("-------------------------------")
    print_log("Dataset loaded. Statistics:")
    print_log(f"Number of samples: {len(dataset)}")
    print_log(f"Number of categories: {len(CATEGORY_LABELS)}")
    print_log(f"Samples for each category: {[
        f"\n  * {category}: {len(os.listdir(os.path.join(GRAYSCALE_IMAGE_DIR, category)))}" 
        for category in CATEGORY_LABELS
    ]}")
    print_log("-------------------------------")

    for epoch in range(20):
        start = time.time()
        running_loss = 0.0
        for i, data in enumerate(dataloader, 0):
            inputs, labels = data
            optimizer.zero_grad()
            outputs = restnet50(inputs)
            loss = criterion(outputs, labels)
            loss.backward()
            optimizer.step()

            running_loss += loss.item()
        print_log(f"COMPLETED EPOCH {epoch + 1:02d} (loss = {running_loss / len(dataloader):.3f})")

        if epoch % 5 == 4:
            print_log("------------------------------")
            print_log(f"Saving model at epoch {epoch + 1}...")
            torch.save(restnet50.state_dict(), f"resnet_food_classifier_big_epoch_{epoch + 1:02d}.pth")
            print_log("Model saved.")
            print_log("------------------------------")


if __name__ == "__main__":
    main()
