import pandas as pd
from sqlalchemy import create_engine
import matplotlib.pyplot as plt
from matplotlib.animation import FuncAnimation
import seaborn as sns
import numpy as np

sns.set_theme(style="whitegrid", rc={'axes.facecolor':'#121212', 'grid.color': '#444444'})

engine = create_engine('postgresql://yassin:yassin@localhost:5432/simulation_data')


# Define the SQL query to fetch data from the database
query = """
    SELECT timestamp, smart_score, high_score, close_score, 
           smart_start_x, smart_start_y, 
           high_start_x, high_start_y, 
           close_start_x, close_start_y
    FROM simulation_data
    ORDER BY timestamp DESC
    LIMIT 5000;
"""

# Initialize the plot with empty data
fig, (ax1, ax2) = plt.subplots(2, 1, figsize=(12, 12))  # Create two subplots

# Initialize empty line objects for the three scores
smart_line, = ax1.plot([], [], label='Smart Score', color='black', lw=2)  # Black line
high_line, = ax1.plot([], [], label='High Score', color='red', lw=2)  # Red line
close_line, = ax1.plot([], [], label='Close Score', color='magenta', lw=2)  # Magenta line

# Initialize empty line objects for the three scores
smart_eucl, = ax2.plot([], [], label='Smart Distance', color='black', lw=2)  # Black line
high_eucl, = ax2.plot([], [], label='High Distance', color='red', lw=2)  # Red line
close_eucl, = ax2.plot([], [], label='Close Distance', color='magenta', lw=2)  # Magenta line


# Set the labels, title, and legend for the first plot
ax1.set_title('Streaming Data of Scores', fontsize=20, color='black', weight='bold')  # Black title for visibility
ax1.set_xlabel('Data Points', fontsize=14, color='black')  # Black for labels
ax1.set_ylabel('Score', fontsize=14, color='black')  # Black for labels
ax1.legend(loc="upper left", fontsize=12, facecolor='#F5F5DC', edgecolor='black', labelcolor='black')  # Beige background, black text for legend

# Adjust the plot aesthetics (grid, ticks, background)
ax1.set_facecolor('#F5F5DC')  # Beige background for the plot area
ax1.grid(True, color='#CCCCCC', linestyle='--', linewidth=0.7)  # Light gray grid lines

# Customize tick colors for better visibility on both x and y axes
ax1.tick_params(axis='x', colors='black')  # Black for x-axis ticks
ax1.tick_params(axis='y', colors='black')  # Black for y-axis ticks

# Customize axis spine colors for visibility
ax1.spines['bottom'].set_color('black')
ax1.spines['left'].set_color('black')


# Set the labels, title, and legend for the first plot
ax2.set_title('Distance to other chasers', fontsize=20, color='black', weight='bold')  # Black title for visibility
ax2.set_xlabel('Data Points', fontsize=14, color='black')  # Black for labels
ax2.set_ylabel('Distance', fontsize=14, color='black')  # Black for labels
ax2.legend(loc="upper left", fontsize=12, facecolor='#F5F5DC', edgecolor='black', labelcolor='black')  # Beige background, black text for legend

# Adjust the plot aesthetics (grid, ticks, background)
ax2.set_facecolor('#F5F5DC')  # Beige background for the plot area
ax1.grid(True, color='#CCCCCC', linestyle='--', linewidth=0.7)  # Light gray grid lines

# Customize tick colors for better visibility on both x and y axes
ax2.tick_params(axis='x', colors='black')  # Black for x-axis ticks
ax2.tick_params(axis='y', colors='black')  # Black for y-axis ticks

# Customize axis spine colors for visibility
ax2.spines['bottom'].set_color('black')
ax2.spines['left'].set_color('black')


# Initialize lists to store the score data
smart_scores = []
high_scores = []
close_scores = []

# Initialize lists to store the distance data
smart_distance = []
high_distance = []
close_distance = []

# Fix the y-axis between 0 and 10,000 for scores
ax1.set_ylim(0, 10000)
ax2.set_ylim(0, 10000)

# Initialize plot limits for 5000 points on the x-axis
def init():
    ax1.set_xlim(0, 5000)  # Display window of the latest 5000 points on x-axis
    ax2.set_xlim(0, 5000)  # Set x-limits for the second plot
    return smart_line, high_line, close_line

# Update function for each frame in the animation
def update(frame):
    # Fetch fresh data from the database
    df = pd.read_sql(query, engine)

    # Get the scores from the latest data
    latest_smart = df['smart_score'].iloc[::-1].values  # Reversing since SQL query returns DESC
    latest_high = df['high_score'].iloc[::-1].values
    latest_close = df['close_score'].iloc[::-1].values

    # Get coordinates for distance calculation
    latest_smart_start_x = df['smart_start_x'].iloc[::-1].values
    latest_smart_start_y = df['smart_start_y'].iloc[::-1].values
    latest_high_start_x = df['high_start_x'].iloc[::-1].values
    latest_high_start_y = df['high_start_y'].iloc[::-1].values
    latest_close_start_x = df['close_start_x'].iloc[::-1].values
    latest_close_start_y = df['close_start_y'].iloc[::-1].values

    # Append the latest scores to the respective lists
    smart_scores.extend(latest_smart)
    high_scores.extend(latest_high)
    close_scores.extend(latest_close)

    # Ensure that only the latest 5000 data points are kept
    smart_scores[:] = smart_scores[-5000:]  # Slice the last 5000 points
    high_scores[:] = high_scores[-5000:]
    close_scores[:] = close_scores[-5000:]

    # Calculate Euclidean distances
    smart_distance.extend(difference_distance(latest_smart_start_x, latest_smart_start_y, latest_high_start_x, latest_high_start_y, latest_close_start_x, latest_close_start_y))
    high_distance.extend(difference_distance(latest_high_start_x, latest_high_start_y, latest_smart_start_x, latest_smart_start_y, latest_close_start_x, latest_close_start_y))
    close_distance.extend(difference_distance(latest_close_start_x, latest_close_start_y, latest_smart_start_x, latest_smart_start_y, latest_high_start_x, latest_high_start_y))

    # Ensure that only the latest 5000 distances are kept
    smart_distance[:] = smart_distance[-5000:]  # Slice the last 5000 points
    high_distance[:] = high_distance[-5000:]
    close_distance[:] = close_distance[-5000:]

    # Generate the x-axis indices (0, 1, 2, ..., up to length of scores)
    x = range(len(smart_scores))

    # Update the line data with the raw scores
    smart_line.set_data(x, smart_scores)
    high_line.set_data(x, high_scores)
    close_line.set_data(x, close_scores)

    smart_eucl.set_data(x, smart_distance)
    high_eucl.set_data(x, high_distance)
    close_eucl.set_data(x, close_distance)
    
    # Dynamically adjust the x-axis to show the last 5000 data points
    ax1.set_xlim(max(0, len(smart_scores) - 5000), len(smart_scores))

    # Plot Euclidean distances on the second subplot
    ax2.set_xlim(max(0, len(smart_scores) - 5000), len(smart_scores))

    
    # Set dynamic y-limit based on the maximum distance calculated

    return smart_line, high_line, close_line, smart_eucl, high_eucl, close_eucl

def difference_distance(x1, y1, x2, y2, x3, y3):
    return np.sqrt((x2 - x1)**2 +(y2 - y1)**2) + np.sqrt(((x3 - x1)**2 +(y3 - y1)**2)) 

# Create the animation, updating every 2000 milliseconds (2 seconds)
ani = FuncAnimation(fig, update, init_func=init, interval=100, blit=True)

# Display the plot
plt.tight_layout()  # Adjust the layout
plt.show()