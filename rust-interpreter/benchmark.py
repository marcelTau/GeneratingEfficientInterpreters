import os
import subprocess
import re
import matplotlib.pyplot as plt
import numpy as np

values = {
        "PushAdd": {},
        "AssignPushAdd,PushAdd": {},
        "AssignPushAdd,PushAdd,PushAssign": {},
        "": {}
}

def execute_command(features):
    directory = "./benchmarks"
    num_runs = 10

    # Iterate over files in the directory
    for filename in os.listdir(directory):
        filepath = os.path.join(directory, filename)
        if os.path.isfile(filepath):
            values[features][filename] = { "generating": [], "interpreting": [], "interpreting (threaded)": [] }

            # Run the command 10 times
            for _ in range(num_runs):
                # output = subprocess.check_output(["your_program", filepath, *features]).decode("utf-8")
                output = subprocess.check_output(["cargo", "run", "-q", "--features", features, "--", filepath]).decode("utf-8")

                # Extract the two values from the output
                match1 = re.search(r"Generating bytecode took (\d+)ms", output)
                match2 = re.search(r"Interpreting took (\d+)ms", output)
                match3 = re.search(r"Interpreting \(threaded\) took (\d+)ms", output)

                value1 = int(match1.group(1)) if match1 else 0
                value2 = int(match2.group(1)) if match2 else 0
                value3 = int(match3.group(1)) if match3 else 0

                values[features][filename]["generating"].append(value1)
                values[features][filename]["interpreting"].append(value2)
                values[features][filename]["interpreting (threaded)"].append(value3)

def show_results(data):

    # Calculate the average of 'generating' and 'interpreting' for each file
    averages = {}
    for group in data:
        averages[group] = {}
        for file in data[group]:
            generating_avg = np.mean(data[group][file]['generating'])
            interpreting_avg = np.mean(data[group][file]['interpreting'])
            interpreting_threaded_avg = np.mean(data[group][file]['interpreting (threaded)'])
            averages[group][file] = {
                'generating': generating_avg,
                'interpreting': interpreting_avg,
                'interpreting (threaded)': interpreting_threaded_avg,
            }
    # plotting
    for group in averages:
        plt.figure()
        plt.title(group if group != "" else "No optimizations")
        x = np.arange(len(averages[group]))
        generating_avg_values = [averages[group][file]['generating'] for file in averages[group]]
        interpreting_avg_values = [averages[group][file]['interpreting'] for file in averages[group]]
        interpreting_threaded_avg_values = [averages[group][file]['interpreting (threaded)'] for file in averages[group]]
        plt.bar(x - 0.3, generating_avg_values, width=0.3, label='generating')
        plt.bar(x + 0, interpreting_avg_values, width=0.3, label='interpreting')
        plt.bar(x + 0.3, interpreting_threaded_avg_values, width=0.3, label='interpreting (threaded)')
        plt.xlabel('Files')
        plt.ylabel('Time (ms)')
        plt.xticks(x, list(averages[group].keys()), rotation=45)
        plt.legend()
        plt.tight_layout()

        # Add numerical values to the plot
        for i, val in enumerate(generating_avg_values):
            plt.annotate( str(round(val, 2)), xy=(x[i] - 0.3, val), xytext=(x[i] - 0.3, 0), ha='center', va='bottom', color='black', fontsize=8)
        for i, val in enumerate(interpreting_avg_values):
            plt.annotate( str(round(val, 2)), xy=(x[i] + 0, val), xytext=(x[i] + 0.0, 0), ha='center', va='bottom', color='black', fontsize=8)
        for i, val in enumerate(interpreting_threaded_avg_values):
            plt.annotate( str(round(val, 2)), xy=(x[i] + 0.3, val), xytext=(x[i] + 0.3, 0), ha='center', va='bottom', color='black', fontsize=8)

# Display the plots
    plt.show()


for feature in values.keys():
    execute_command(feature)

show_results(values)

# print(values)
# pprint.pprint(values)
