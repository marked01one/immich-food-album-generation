import os, cv2, time, json
import numpy as np
import matplotlib.pyplot as plt
import torch.nn.functional as F
from IPython.display import Image, display, clear_output
import torch
import torch.nn as nn
import torch.optim as optim
from torch.utils.data import DataLoader, TensorDataset
from cnn import ResNet, Bottleneck

EVAL_IMAGE_DIR = ".data/eval_rgb_224"
EVAL_IMAGE_METADATA_PATH = os.path.join(EVAL_IMAGE_DIR, ".labels.json")
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
 
def load_model(model_path) -> ResNet:
    net = ResNet(Bottleneck, [3, 4, 6, 3], num_classes=len(CATEGORY_LABELS), num_channels=3)
    net.load_state_dict(torch.load(model_path, weights_only=True))
    net.eval()
    return net

def evaluate_model():

    TP, TN, FP, FN = 0, 0, 0, 0
    
    label_file = open(EVAL_IMAGE_METADATA_PATH, "r")
    eval_image_labels = json.load(label_file)
    label_file.close()

    resnet = load_model("../models/resnet_food_classifier_big_epoch_10.pth")

    with torch.no_grad():
        eval_images = [img for img in sorted(os.listdir(os.path.join(EVAL_IMAGE_DIR))) if not img.endswith(".json")]

        for img_path in eval_images:
            img = cv2.imread(os.path.join(EVAL_IMAGE_DIR, img_path), cv2.IMREAD_COLOR_RGB).transpose(2, 0, 1) / 255.0  # Convert to CHW format and normalize

            tensor = torch.tensor(np.array(img), dtype=torch.float32).unsqueeze(0) # Add batch and channel dimensions
            prediction = resnet(tensor)
            probabilities = F.softmax(prediction, dim=1).numpy()[0]


            predicted_label = CATEGORY_LABELS[np.argmax(probabilities)]
            real_label = eval_image_labels[img_path]

            TN += int(predicted_label == "not_food" and real_label == "not_food")
            FP += int(predicted_label != "not_food" and real_label == "not_food")
            FN += int(predicted_label == "not_food" and real_label != "not_food")
            TP += int(predicted_label != "not_food" and real_label != "not_food")

            clear_output(wait=True)

    accuracy = (TP + TN) / (TP + TN + FP + FN) * 100 if (TP + TN + FP + FN) > 0 else 0
    precision = TP / (TP + FP) * 100 if (TP + FP) > 0 else 0
    recall = TP / (TP + FN) * 100 if (TP + FN) > 0 else 0
    f1_score = 2 * (precision * recall) / (precision + recall) if (precision + recall) > 0 else 0

    print("EVALUATION RESULTS:")
    print("------------------------------")

    print("True Positives (TP):", TP)
    print("True Negatives (TN):", TN)
    print("False Positives (FP):", FP)
    print("False Negatives (FN):", FN)
    print("------------------------------")

    print(f"1. Accuracy:\t{accuracy:.2f} %")
    print(f"2. Precision:\t{precision:.2f} %")
    print(f"3. Recall:\t{recall:.2f} %")
    print(f"4. F1 Score:\t{f1_score:.2f} %")



def main():
    evaluate_model()

if __name__ == "__main__":
    main()