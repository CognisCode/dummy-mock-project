import pandas as pd
from sqlalchemy import create_engine
import matplotlib.pyplot as plt
from matplotlib.animation import FuncAnimation
import seaborn as sns
import numpy as np

sns.set_theme(style="whitegrid", rc={'axes.facecolor':'#121212', 'grid.color': '#444444'})

engine = create_engine('postgresql://yassin:yassin@localhost:5432/simulation_data')

query = """
    SELECT timestamp, value_score, high_score, close_score, custom_score,
           value_x, value_y, 
           high_x, high_y, 
           close_x, close_y,
           custom_x, custom_y
    FROM simulation_data
    ORDER BY timestamp DESC
    LIMIT 5000;
"""

fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(8, 8))  

value_line, = ax1.plot([], [], label='value Score', color='black', lw=2) 
high_line, = ax1.plot([], [], label='High Score', color='red', lw=2)  
close_line, = ax1.plot([], [], label='Close Score', color='magenta', lw=2)  
custom_line, = ax1.plot([], [], label='custom Score', color='blue', lw=2)  

value_eucl, = ax2.plot([], [], label='value Distance', color='black', lw=2)  
high_eucl, = ax2.plot([], [], label='High Distance', color='red', lw=2) 
close_eucl, = ax2.plot([], [], label='Close Distance', color='magenta', lw=2)  
custom_eucl, = ax2.plot([], [], label='custom Distance', color='blue', lw=2)  

ax1.set_title('Streaming Data of Scores', fontsize=20, color='black', weight='bold') 
ax1.set_xlabel('Data Points', fontsize=14, color='black') 
ax1.set_ylabel('Score', fontsize=14, color='black') 
ax1.legend(loc="upper left", fontsize=12, facecolor='#F5F5DC', edgecolor='black', labelcolor='black') 

ax1.set_facecolor('#F5F5DC')  
ax1.grid(True, color='#CCCCCC', linestyle='--', linewidth=0.7)  

ax1.tick_params(axis='x', colors='black')  
ax1.tick_params(axis='y', colors='black')  

ax1.spines['bottom'].set_color('black')
ax1.spines['left'].set_color('black')

ax2.set_title('Distance to other chasers', fontsize=20, color='black', weight='bold') 
ax2.set_xlabel('Data Points', fontsize=14, color='black')  
ax2.set_ylabel('Distance', fontsize=14, color='black')  
ax2.legend(loc="upper left", fontsize=12, facecolor='#F5F5DC', edgecolor='black', labelcolor='black')  

ax2.set_facecolor('#F5F5DC') 
ax1.grid(True, color='#CCCCCC', linestyle='--', linewidth=0.7)  

ax2.tick_params(axis='x', colors='black')  
ax2.tick_params(axis='y', colors='black')  

ax2.spines['bottom'].set_color('black')
ax2.spines['left'].set_color('black')

value_scores = []
high_scores = []
close_scores = []
custom_scores = []

value_distance = []
high_distance = []
close_distance = []
custom_distance = []

ax1.set_ylim(0, 15000)
ax2.set_ylim(0, 15000)

def init():
    ax1.set_xlim(0, 5000) 
    ax2.set_xlim(0, 5000) 
    return value_line, high_line, close_line, custom_line

def update(frame):
    df = pd.read_sql(query, engine)

    latest_value = df['value_score'].iloc[::-1].values 
    latest_high = df['high_score'].iloc[::-1].values
    latest_close = df['close_score'].iloc[::-1].values
    latest_custom = df['custom_score'].iloc[::-1].values


    latest_value_x = df['value_x'].iloc[::-1].values
    latest_value_y = df['value_y'].iloc[::-1].values
    latest_high_x = df['high_x'].iloc[::-1].values
    latest_high_y = df['high_y'].iloc[::-1].values
    latest_close_x = df['close_x'].iloc[::-1].values
    latest_close_y = df['close_y'].iloc[::-1].values
    latest_custom_x = df['custom_x'].iloc[::-1].values
    latest_custom_y = df['custom_y'].iloc[::-1].values

    value_scores.extend(latest_value)
    high_scores.extend(latest_high)
    close_scores.extend(latest_close)
    custom_scores.extend(latest_custom)


    value_scores[:] = value_scores[-5000:]  
    high_scores[:] = high_scores[-5000:]
    close_scores[:] = close_scores[-5000:]
    custom_scores[:] = custom_scores[-5000:]


    value_distance.extend(difference_distance(latest_value_x, latest_value_y, latest_high_x, latest_high_y, latest_close_x, latest_close_y, latest_custom_x, latest_custom_y))
    high_distance.extend(difference_distance(latest_high_x, latest_high_y, latest_value_x, latest_value_y, latest_close_x, latest_close_y, latest_custom_x, latest_custom_y))
    close_distance.extend(difference_distance(latest_close_x, latest_close_y, latest_value_x, latest_value_y, latest_high_x, latest_high_y, latest_custom_x, latest_custom_y))
    custom_distance.extend(difference_distance(latest_custom_x, latest_custom_y, latest_value_x, latest_value_y, latest_high_x, latest_high_y, latest_close_x, latest_close_y))

    value_distance[:] = value_distance[-5000:]  
    high_distance[:] = high_distance[-5000:]
    close_distance[:] = close_distance[-5000:]
    custom_distance[:] = custom_distance[-5000:]


    x = range(len(value_scores))

    value_line.set_data(x, value_scores)
    high_line.set_data(x, high_scores)
    close_line.set_data(x, close_scores)
    custom_line.set_data(x, custom_scores)

    value_eucl.set_data(x, value_distance)
    high_eucl.set_data(x, high_distance)
    close_eucl.set_data(x, close_distance)
    custom_eucl.set_data(x, close_distance)

    ax1.set_xlim(max(0, len(value_scores) - 5000), len(value_scores))
    ax2.set_xlim(max(0, len(value_scores) - 5000), len(value_scores))

    return value_line, high_line, close_line, custom_line, value_eucl, high_eucl, close_eucl, custom_eucl

def difference_distance(x1, y1, x2, y2, x3, y3, x4, y4):
    return np.sqrt((x2 - x1)**2 +(y2 - y1)**2) + np.sqrt(((x3 - x1)**2 +(y3 - y1)**2)) + np.sqrt(((x4 - x1)**2 +(y4 - y1)**2)) 

ani = FuncAnimation(fig, update, init_func=init, interval=100, blit=True)

plt.tight_layout()
plt.show()