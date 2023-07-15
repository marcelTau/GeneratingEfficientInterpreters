import os
import subprocess
import re
import matplotlib.pyplot as plt
import pprint
import numpy as np

values = {
        "AssignPushAdd,PushAdd": {},
        "": {}
}

# features = "AssignPushAdd,PushAdd"

def execute_command(features):
    directory = "./benchmarks"
    num_runs = 10

    # Initialize dictionaries to store the values for each feature
    value1_dict = {}
    value2_dict = {}

    # Iterate over files in the directory
    for filename in os.listdir(directory):
        filepath = os.path.join(directory, filename)
        if os.path.isfile(filepath):
            value1_results = []
            value2_results = []

            # print(filename)
            values[features][filename] = { "generating": [], "interpreting": [] }

            # Run the command 10 times
            for _ in range(num_runs):
                # output = subprocess.check_output(["your_program", filepath, *features]).decode("utf-8")
                output = subprocess.check_output(["cargo", "run", "-q", "--features", features, "--", filepath]).decode("utf-8")

                # Extract the two values from the output
                match1 = re.search(r"Generating bytecode took (\d+)ms", output)
                match2 = re.search(r"Interpreting took (\d+)ms", output)

                value1 = int(match1.group(1)) if match1 else 0
                value2 = int(match2.group(1)) if match2 else 0

                values[features][filename]["generating"].append(value1)
                values[features][filename]["interpreting"].append(value2)
                # value1_results.append(value1)
                # value2_results.append(value2)

            # Calculate the average for each file
            # value1_average = sum(value1_results) / num_runs
            # value2_average = sum(value2_results) / num_runs

            # Store the averages in the dictionaries
            # value1_dict[filename] = value1_average
            # value2_dict[filename] = value2_average

    # Plotting the values
    # x = range(len(value1_dict))

    # plt.figure(figsize=(10, 5))

    # # Plot value 1
    # plt.subplot(1, 2, 1)
    # plt.plot(x, list(value1_dict.values()), marker='o')
    # plt.xticks(x, list(value1_dict.keys()), rotation=90)
    # plt.xlabel('Files')
    # plt.ylabel('Generating time')
    # plt.title('Average generating time (ms)')
    # plt.grid(True)

    # # Plot value 2
    # plt.subplot(1, 2, 2)
    # plt.plot(x, list(value2_dict.values()), marker='o')
    # plt.xticks(x, list(value2_dict.keys()), rotation=90)
    # plt.xlabel('Files')
    # plt.ylabel('Interpreting time')
    # plt.title('Average Interpreting time (ms)')
    # plt.grid(True)

    # plt.tight_layout()
    # plt.show()

# Example usage:
# execute_command(features)
# execute_command("")

def show_results(data):

# Calculate the average of 'generating' and 'interpreting' for each file
    averages = {}
    for group in data:
        averages[group] = {}
        for file in data[group]:
            generating_avg = np.mean(data[group][file]['generating'])
            interpreting_avg = np.mean(data[group][file]['interpreting'])
            averages[group][file] = {
                'generating': generating_avg,
                'interpreting': interpreting_avg
            }

# Plotting
    for group in averages:
        plt.figure()
        plt.title(group)
        x = np.arange(len(averages[group]))
        generating_avg_values = [averages[group][file]['generating'] for file in averages[group]]
        interpreting_avg_values = [averages[group][file]['interpreting'] for file in averages[group]]
        plt.bar(x - 0.2, generating_avg_values, width=0.4, label='generating')
        plt.bar(x + 0.2, interpreting_avg_values, width=0.4, label='interpreting')
        plt.xlabel('Files')
        plt.ylabel('Time (ms)')
        plt.xticks(x, list(averages[group].keys()), rotation=45)
        plt.legend()
        plt.tight_layout()

        # Add numerical values to the plot
        for i, val in enumerate(generating_avg_values):
            plt.annotate( str(round(val, 2)), xy=(x[i] - 0.2, val), xytext=(x[i] - 0.2, 0), ha='center', va='bottom', color='black')
        for i, val in enumerate(interpreting_avg_values):
            plt.annotate( str(round(val, 2)), xy=(x[i] + 0.2, val), xytext=(x[i] + 0.2, 0), ha='center', va='bottom', color='black')

# Display the plots
    plt.show()


for feature in values.keys():
    execute_command(feature)

show_results(values)

# print(values)
# pprint.pprint(values)
