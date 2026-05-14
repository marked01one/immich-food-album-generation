# Immich Food Collage Generator

Automatically detect and organize food photos in your [Immich](https://immich.app) library into a
dedicated album using a custom-trained image classifier.

---
 
## Features
 
- Classifies photos as food or non-food using a trained ML model
- Automatically creates and populates a dedicated food album in Immich
- Incremental updates — only newly added photos are scanned on subsequent runs
- Fast inference via a compiled Rust service
- Fully self-hosted; no data leaves your machine

---
 
## Prerequisites
 
| Requirement | Version |
|---|---|
| [Immich](https://immich.app/) | v1.90+ |
| [Rust](https://www.rust-lang.org/tools/install) | 1.75+ |
| [Python](https://www.python.org/) | 3.10+ |
| An Immich API key | — |
 
---
 
## Getting Started
 
### 1. Clone the repository
 
```bash
git clone https://github.com/marked01one/immich-food-album-generation.git
cd immich-food-album-generation
```
 
### 2. Train the model (optional — use a pre-trained checkpoint if available)
 
```bash
cd training
pip install -r requirements.txt
jupyter notebook
```
 
Open the training notebook and follow the steps to train and export the model weights. The exported model file should be placed in `server/model/`.
 
### 3. Configure the server
 
Create a `.env` file in the `server/` directory (or set the following environment variables):
 
```env
IMMICH_BASE_URL=http://your-immich-instance:2283
IMMICH_API_KEY=your_api_key_here
ALBUM_NAME=Food
MODEL_PATH=./model/food_classifier.onnx   # or whatever format your model uses
SCAN_INTERVAL_SECS=3600                   # how often to re-scan (in seconds)
```
 
### 4. Build and run the server
 
```bash
cd server
cargo build --release
./target/release/server
```
 
On first run, the server will scan all assets in your Immich library, classify them, and populate the album. Subsequent runs will only process new assets.
 
---
 
## Training
 
The `training/` directory contains Jupyter notebooks and Python scripts for:
 
- Downloading or preparing a food image dataset (e.g., [Food-101](https://data.vision.ee.ethz.ch/cvl/datasets_extra/food-101/))
- Fine-tuning a pre-trained image classification model
- Evaluating accuracy on a validation set
- Exporting the trained model for use by the Rust server
See [`training/README.md`](training/) for detailed instructions.
 
---
 
## Project Structure
 
```
immich-food-album-generation/
├── server/          # Rust service — Immich API client + inference
│   ├── src/
│   └── Cargo.toml
├── training/        # Python/Jupyter — model training & evaluation
│   ├── notebooks/
│   └── requirements.txt
├── .gitignore
└── README.md
```
 
---
