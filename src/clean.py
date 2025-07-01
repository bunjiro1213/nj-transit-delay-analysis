import pandas as pd
import os

# Define paths
base_path = "../src/data/raw-data"
file_names = [
    "2018_03.csv", "2018_04.csv", "2018_05.csv", "2018_06.csv", "2018_07.csv",
    "2018_08.csv", "2018_09.csv", "2018_10.csv", "2018_11.csv", "2018_12.csv",
    "2019_01.csv", "2019_02.csv", "2019_03.csv", "2019_04.csv", "2019_05.csv",
    "2019_06.csv", "2019_07.csv", "2019_08.csv", "2019_09.csv", "2019_10.csv",
    "2019_11.csv", "2019_12.csv", "2020_01.csv", "2020_02.csv", "2020_03.csv",
    "2020_04.csv", "2020_05.csv"
]
file_paths = [os.path.join(base_path, name) for name in file_names]

# Load data
dfs = []
for file_path in file_paths:
    try:
        df = pd.read_csv(file_path)
        dfs.append(df)
    except FileNotFoundError:
        print(f"File not found: {file_path}")
        continue

# Attach year and month
dates = [
    (2018, 3), (2018, 4), (2018, 5), (2018, 6), (2018, 7),
    (2018, 8), (2018, 9), (2018, 10), (2018, 11), (2018, 12),
    (2019, 1), (2019, 2), (2019, 3), (2019, 4), (2019, 5),
    (2019, 6), (2019, 7), (2019, 8), (2019, 9), (2019, 10),
    (2019, 11), (2019, 12), (2020, 1), (2020, 2), (2020, 3),
    (2020, 4), (2020, 5)
]

if len(dfs) != len(dates):
    print(f"Loaded {len(dfs)} dataframes, expected {len(dates)}.")
else:
    for df, (year, month) in zip(dfs, dates):
        df["year"] = year
        df["month"] = month

    # Combine all data
    df_combined = pd.concat(dfs, axis=0, ignore_index=True)
    print("Combined shape:", df_combined.shape)

    # Filter non-Amtrak records
    df_filtered = df_combined[df_combined["type"] != "Amtrak"].copy()

    # Drop missing values
    df_filtered = df_filtered.dropna(subset=["delay_minutes", "from", "to"])

    # Clean station names
    df_filtered["from"] = df_filtered["from"].str.strip()
    df_filtered["to"] = df_filtered["to"].str.strip()

    # Sample for Rust code
    df_sampled = df_filtered.sample(n=1001, random_state=42)
    df_sampled.to_csv("stations_filtered.csv", index=False)

    print("Cleaned + sampled data saved to stations_filtered.csv")
