import os, cv2, time, requests
import numpy as np
import torch
import numpy as np
import torch.nn as nn
import torch.optim as optim
from torch.utils.data import DataLoader, TensorDataset
from cnn import ResNet, Bottleneck

from dataset import FoodDataset, GRAYSCALE_IMAGE_DIR

SOURCE_IMAGE_DIR = ".data/images"

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

    dataset = FoodDataset(CATEGORY_LABELS)
    dataloader = DataLoader(dataset, batch_size=64, shuffle=True)

    net = ResNet(Bottleneck, [2, 2, 2, 2], num_classes=len(CATEGORY_LABELS), num_channels=1)
    criterion = nn.CrossEntropyLoss()
    optimizer = optim.SGD(net.parameters(), lr=0.001, momentum=0.9)

    print("-------------------------------")
    print("Dataset loaded. Statistics:")
    print(f"Number of samples: {len(dataset)}")
    print(f"Number of categories: {len(CATEGORY_LABELS)}")
    print(f"Samples for each category: {[
        f"\n  * {category}: {len(os.listdir(os.path.join(GRAYSCALE_IMAGE_DIR, category)))}" 
        for category in CATEGORY_LABELS
    ]}")
    print("-------------------------------")

    for epoch in range(20):
        start = time.time()
        running_loss = 0.0
        for i, data in enumerate(dataloader, 0):
            inputs, labels = data
            optimizer.zero_grad()
            outputs = net(inputs)
            loss = criterion(outputs, labels)
            loss.backward()
            optimizer.step()

            running_loss += loss.item()
        print(f"[Epoch {epoch + 1}] loss: {running_loss / len(dataloader):.3f} - Time: {time.time() - start:.2f} seconds")

        if epoch % 5 == 4:
            print("------------------------------")
            print(f"Saving model at epoch {epoch + 1}...")
            torch.save(net.state_dict(), f"resnet_food_classifier_epoch_{epoch + 1:02d}.pth")
            print("Model saved.")
            print("------------------------------")


if __name__ == "__main__":
    main()
